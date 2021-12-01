use std::ops::{Deref, DerefMut};
use std::cmp::{PartialEq, Eq};
use std::rc::{Rc, Weak};
use std::fmt;
use std::collections::HashMap;
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
impl fmt::Debug for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Operand::*;
        match self {
            Literal(x) => x.fmt(f),
            Instruction(x) => f.write_str(&make_instr_name_weak(&x.0)),
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
    pub fn downgrade(self) -> InstructionRef {
        InstructionRef(Rc::downgrade(&self.0))
    }
}
impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct(&make_instr_name(&self.0))
            .field("operands", &self.operands)
            .finish()
    }
}
impl PartialEq for Instruction {
    fn eq(&self, b: &Self) -> bool {
        Rc::ptr_eq(&self.0, &b.0)
    }
}
impl Eq for Instruction {}

#[derive(Clone)]
pub struct InstructionRef(Weak<InstructionInner>);
impl InstructionRef {
    pub fn upgrade(self) -> Option<Instruction> {
        self.0.upgrade().map(|x| Instruction(x))
    }
}
impl fmt::Debug for InstructionRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&make_instr_name_weak(&self.0))
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
            Ok(false)
        }
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
fn deserialize_instrs(instrs: &[Instr]) -> Result<Vec<Instruction>> {
    let mut de = SpirvDeserializer::new(instrs.len());
    let mut done = true;
    for _ in 0..100 {
        done = true;
        for (i, instr) in instrs.iter().enumerate() {
            // Ignore some in-function debug instructions because they can show
            // up before `OpLabel` which break other processing.
            if is_line_debug_instr(instr) { continue; }
            done &= de.deserialize_instr(i, instr)?;
        }
        if done { break; }
    }
    if !done {
        return Err(Error::UNUSUAL_REFERENCE_COMPLEXITY);
    }
    let out = de.instrs.into_iter().filter_map(|x| x).collect();
    Ok(out)
}

pub struct Spirv {
    header: SpirvHeader,
    instrs: Vec<Instruction>,
}
impl Spirv {
    pub fn header(&self) -> &SpirvHeader { &self.header }
    pub fn instrs(&self) -> &[Instruction] { &self.instrs }
}
impl<'a> TryFrom<Spv<'a>> for Spirv {
    type Error = Error;
    fn try_from(spv: Spv<'a>) -> Result<Spirv> {
        let instrs = deserialize_instrs(spv.instrs())?;
        let out = Spirv { header: spv.header().clone(), instrs };
        Ok(out)
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
        let operand_lens1: Vec<_> = spv.instrs()
            .iter()
            .map(|x| x.len())
            .collect();
        let spv = Spirv::try_from(spv).unwrap();
        let operand_lens2: Vec<_> = spv.instrs()
            .iter()
            .map(|x| x.len())
            .collect();
        // There should be a same numbder of instructions emitted.
        assert_eq!(operand_lens1.len(), operand_lens2.len());
        // Also the same number of operands converted for each instruction.
        for (a, b) in operand_lens1.iter().zip(operand_lens2.iter()) {
            assert_eq!(a, b);
        }
    }
}
