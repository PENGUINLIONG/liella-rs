use std::ops::{Deref, DerefMut};
use std::cmp::{PartialEq, Eq};
use std::rc::{Rc, Weak};
use std::fmt;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use crate::error::{LiellaError as Error, LiellaResult as Result};
use crate::spv::{Instr, Spv};

mod gen;

pub type SpirvHeader = crate::spv::SpirvHeader;
pub type OpCode = crate::spv::OpCode;

type SpvId = crate::spv::SpvId;

fn make_instr_name(inner: &Rc<InstructionInner>) -> String {
    format!("{}@{:016x}",
        gen::opcode2name(inner.opcode),
        Rc::as_ptr(inner) as *const InstructionInner as usize)
}
fn make_instr_name_weak(inner: &Weak<InstructionInner>) -> String {
    inner.upgrade()
        .map(|x| make_instr_name(&x))
        .unwrap_or("Instruction@DROPPED".to_owned())
}

#[derive(Clone)]
pub enum Operand {
    Literal(u32),
    Instruction(InstructionRef),
    ResultPlaceholder,
}
impl Operand {
    pub fn as_lit(&self) -> Option<u32> {
        if let Self::Literal(x) = self { Some(*x) } else { None }
    }
    pub fn as_instr(&self) -> Option<InstructionRef> {
        if let Self::Instruction(x) = self { Some(x.clone()) } else { None }
    }
}
impl fmt::Debug for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Operand::*;
        match self {
            Literal(x) => x.fmt(f),
            Instruction(x) => x.upgrade().unwrap().fmt(f),
            ResultPlaceholder => write!(f, "<result>"),
        }
    }
}

pub struct InstructionInner {
    opcode: OpCode,
    operands: Vec<Operand>,
}
impl InstructionInner {
    pub fn opcode(&self) -> OpCode {
        self.opcode
    }
    pub fn opname(&self) -> &'static str {
        gen::opcode2name(self.opcode)
    }
    pub fn operands(&self) -> &[Operand] {
        &self.operands
    }
    pub fn len(&self) -> usize {
        self.operands.len() + 1
    }
}

#[derive(Clone)]
pub struct Instruction(Rc<InstructionInner>);
impl Deref for Instruction {
    type Target = InstructionInner;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for Instruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Rc::get_mut(&mut self.0).unwrap()
    }
}
impl Instruction {
    pub fn new(opcode: OpCode, operands: Vec<Operand>) -> Instruction {
        let inner = InstructionInner { opcode, operands };
        Instruction(Rc::new(inner))
    }
    pub fn downgrade(&self) -> InstructionRef {
        InstructionRef(Rc::downgrade(&self.0))
    }
}
impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{} ", make_instr_name(&self.0)))?;
        f.debug_list()
            .entries(self.operands())
            .finish()
    }
}
impl PartialEq for Instruction {
    fn eq(&self, b: &Self) -> bool {
        Rc::ptr_eq(&self.0, &b.0)
    }
}
impl Eq for Instruction {}
impl std::hash::Hash for Instruction {
    fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher {
        state.write_usize(Rc::as_ptr(&self.0) as usize);
    }
}

#[derive(Clone)]
pub struct InstructionRef(Weak<InstructionInner>);
impl InstructionRef {
    pub fn upgrade(&self) -> Option<Instruction> {
        self.0.upgrade().map(|x| Instruction(x))
    }
}
impl fmt::Debug for InstructionRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&make_instr_name_weak(&self.0))
    }
}
impl PartialEq for InstructionRef {
    fn eq(&self, b: &Self) -> bool {
        self.0.ptr_eq(&b.0)
    }
}
impl Eq for InstructionRef {}
impl std::hash::Hash for InstructionRef {
    fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher {
        state.write_usize(self.0.as_ptr() as usize);
    }
}

type InstrIdx = usize;
pub(crate) struct SpirvDeserializer {
    instrs: Vec<Option<Instruction>>,
    id_map: HashMap<SpvId, InstrIdx>,
}
impl SpirvDeserializer {
    fn new(ninstr: usize) -> Self {
        SpirvDeserializer {
            instrs: std::iter::repeat(None).take(ninstr).collect(),
            id_map: HashMap::new(),
        }
    }
    fn get_instr_by_id(&self, id: SpvId) -> Option<&Instruction> {
        if let Some(idx) = self.id_map.get(&id) {
            if let Some(instr) = self.instrs.get(*idx as usize) {
                instr.as_ref()
            } else {
                None
            }
        } else {
            None
        }
    }
    fn deserialize_instr(
        &mut self,
        idx: InstrIdx,
        raw_instr: &Instr
    ) -> Result<bool> {
        use std::collections::hash_map::Entry;
        if self.instrs[idx].is_some() { return Ok(true); }
        if let Some((id, instr)) = gen::deserialize_instr(self, raw_instr)? {
            self.instrs[idx] = Some(instr);
            if id != 0 {
                match self.id_map.entry(id) {
                    Entry::Occupied(_) => {
                        return Err(Error::RESULT_ID_COLLISION);
                    },
                    Entry::Vacant(entry) => {
                        entry.insert(idx);
                    },
                }
            }
            Ok(true)
        } else {
            // Found forward references that cannot be resolved now.
            Ok(false)
        }
    }
    /// Collect statment instructions (those doesn't have a reference ID).
    fn into_stmts_exprs(self) -> (Vec<Instruction>, Vec<Instruction>) {
        const OP_LABEL: u32 = 248;
        let expr_idxs = self.id_map.into_iter()
            .map(|(_id, idx)| idx)
            .collect::<HashSet<_>>();
        let mut stmts = Vec::new();
        let mut exprs = Vec::new();
        for (idx, instr) in self.instrs.into_iter().enumerate() {
            if let Some(instr) = instr {
                // TODO: (penguinliong) Find another way to probe labels to
                // replace this dirty workaround.
                if expr_idxs.contains(&idx) && instr.opcode() != OP_LABEL {
                    exprs.push(instr);
                } else {
                    stmts.push(instr);
                }
            }
        }
        (stmts, exprs)
    }
}

pub(crate) struct InstructionBuilder<'a> {
    ctxt: &'a SpirvDeserializer,
    inner: Option<InstructionInner>,
}
impl<'a> InstructionBuilder<'a> {
    pub fn new(ctxt: &'a SpirvDeserializer, opcode: OpCode) -> Self {
        let inner = InstructionInner { opcode, operands: Default::default() };
        InstructionBuilder { ctxt, inner: Some(inner) }
    }
    pub fn lit(&mut self, x: u32) {
        if let Some(inner) = self.inner.as_mut() {
            let operand = Operand::Literal(x);
            inner.operands.push(operand);
        }
    }
    pub fn id(&mut self, id: SpvId) {
        if let Some(x) = self.ctxt.get_instr_by_id(id) {
            if let Some(inner) = self.inner.as_mut() {
                let operand = Operand::Instruction(x.clone().downgrade());
                inner.operands.push(operand);
            }
        } else {
            self.inner = None;
        }
    }
    pub fn res(&mut self) {
        if let Some(inner) = self.inner.as_mut() {
            let operand = Operand::ResultPlaceholder;
            inner.operands.push(operand);
        }
    }
    pub fn build(self) -> Option<Instruction> {
        self.inner.map(|x| Instruction(Rc::new(x)))
    }
}
fn is_line_debug_instr(instr: &Instr) -> bool {
    const OP_LINE: u32 = 8;
    const OP_NO_LINE: u32 = 317;
    match instr.opcode() {
        OP_LINE | OP_NO_LINE => true,
        _ => false,
    }
}

#[derive(Clone)]
pub struct Spirv {
    /// SPIR-V header.
    header: SpirvHeader,
    /// Instruction pool to keep at least one reference count to the
    /// allocations. There is no guarantee the instructions are kept in order in
    /// `instr_pool`. Any demand on instruction ordering should be redirected to
    /// `stmts`.
    instr_pool: HashSet<Instruction>,
    /// Statements, which are instructions that don't have other instructions
    /// refering to them.
    stmts: Vec<InstructionRef>,
}
impl Spirv {
    pub fn new(
        header: SpirvHeader,
        instr_pool: HashSet<Instruction>,
        stmts: Vec<InstructionRef>
    ) -> Self {
        Spirv { header, instr_pool, stmts }
    }
    pub fn header(&self) -> &SpirvHeader { &self.header }
    pub fn stmts(&self) -> &[InstructionRef] { &self.stmts }
}
impl<'a> TryFrom<Spv<'a>> for Spirv {
    type Error = Error;
    fn try_from(spv: Spv<'a>) -> Result<Spirv> {
        let mut de = SpirvDeserializer::new(spv.instrs().len());
        let mut done = true;
        for _ in 0..100 {
            done = true;
            for (i, instr) in spv.instrs().iter().enumerate() {
                // Ignore some in-function debug instructions because they can
                // show up before `OpLabel` which break other processing.
                if is_line_debug_instr(instr) { continue; }
                done &= de.deserialize_instr(i, instr)?;
            }
            if done { break; }
        }
        if !done {
            return Err(Error::UNUSUAL_REFERENCE_COMPLEXITY);
        }
        let (stmts, exprs) = de.into_stmts_exprs();
        let stmt_refs = stmts.iter().map(|x| x.downgrade()).collect::<Vec<_>>();
        let instr_pool = stmts.into_iter().chain(exprs).collect();

        let out = Spirv {
            header: spv.header().clone(),
            instr_pool,
            stmts: stmt_refs
        };
        Ok(out)
    }
}
impl fmt::Debug for Spirv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn fmt_instr_lisp(instr: &InstructionRef) -> String {
            let instr = instr.upgrade().unwrap();
            let operands_lit_it = instr.operands().iter()
                .map(|operand| {
                    match operand {
                        Operand::Instruction(x) => fmt_instr_lisp(&x),
                        Operand::Literal(x) => format!("{}", x),
                        Operand::ResultPlaceholder => "<result>".to_owned(),
                    }
                });
            let mut segs = vec![instr.opname().to_owned()];
            segs.extend(operands_lit_it);

            let lit = segs.join(" ");
            format!("({})", lit)
        }

        f.debug_list()
            .entries(self.stmts().iter().map(fmt_instr_lisp))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inline_spirv::inline_spirv;
    #[test]
    fn test_parse() {
        let spv: &'static [u32] =
            inline_spirv!("#version 450\nvoid main() {}", comp, vulkan1_0);
        let spv = Spv::try_from(spv).unwrap();
        let _operand_lens1: Vec<_> = spv.instrs()
            .iter()
            .map(|x| x.len())
            .collect();
        let spv = Spirv::try_from(spv).unwrap();
        let _operand_lens2: Vec<_> = spv.stmts()
            .iter()
            .map(|x| x.upgrade().unwrap().len())
            .collect();
    }
}
