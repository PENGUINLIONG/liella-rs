use std::ops::{Deref, DerefMut};
use std::cmp::{PartialEq, Eq};
use std::borrow::{Borrow, BorrowMut};
use std::cell::{RefCell};
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

#[derive(Clone)]
pub enum Operand {
    Literal(u32),
    Instruction(NodeRef),
    ResultPlaceholder,
}
impl Operand {
    pub fn as_lit(&self) -> Option<u32> {
        if let Self::Literal(x) = self { Some(*x) } else { None }
    }
    pub fn as_instr(&self) -> Option<&NodeRef> {
        if let Self::Instruction(x) = self { Some(x) } else { None }
    }
}
impl fmt::Debug for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Operand::*;
        match self {
            Literal(x) => x.fmt(f),
            Instruction(x) => x.upgrade().fmt(f),
            ResultPlaceholder => write!(f, "<result>"),
        }
    }
}







pub struct Instruction {
    pub opcode: OpCode,
    pub operands: Vec<Operand>,
    /// Next instruction. Should be `None` at the end.
    pub next: Option<NodeRef>,
}
impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(gen::opcode2name(self.opcode))?;
        f.debug_list()
            .entries(&self.operands)
            .finish()
    }
}



#[derive(Debug)]
pub enum NodeInner {
    Instruction(Instruction),
}
impl NodeInner {
    pub fn as_instr(&self) -> Option<&Instruction> {
        if let Self::Instruction(out) = self { Some(out) } else { None }
    }
    pub fn as_instr_mut(&mut self) -> Option<&mut Instruction> {
        if let Self::Instruction(out) = self { Some(out) } else { None }
    }
}

#[derive(Clone)]
pub struct Node(Rc<RefCell<NodeInner>>);
impl Node {
    pub fn rewrite<F: FnMut(&Node) -> Option<Node>>(&mut self, mut rewriter: F) {
        if let Some(x) = rewriter(self) {
            *self = x;
        }
    }

    pub fn collect_children(&self, children: &mut Vec<NodeRef>) {
        match &*self.borrow() {
            NodeInner::Instruction(x) => {
                children.extend(x.next.clone().into_iter());
            },
        }
    }

    pub fn borrow(&self) -> std::cell::Ref<'_, NodeInner> {
        (*self.0).borrow()
    }
    pub fn borrow_mut(&self) -> std::cell::RefMut<'_, NodeInner> {
        (*self.0).borrow_mut()
    }
}
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.borrow().fmt(f)
    }
}
#[derive(Clone)]
pub struct NodeRef(Weak<RefCell<NodeInner>>);
impl NodeRef {
    pub fn upgrade(&self) -> Node {
        Node(self.0.upgrade().unwrap())
    }
}






pub struct Context {
    allocs: Vec<Node>,
}
impl Context {
    pub fn new() -> Context {
        Context {
            allocs: Vec::new(),
        }
    }
    pub fn alloc_instr(&mut self, instr: Instruction) -> NodeRef {
        let rc = Rc::new(RefCell::new(NodeInner::Instruction(instr)));
        let rv = NodeRef(Rc::downgrade(&rc));
        let alloc = Node(rc);
        self.allocs.push(alloc);
        rv
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

type InstrIdx = usize;
pub(crate) struct SpirvDeserializer<'a> {
    ctxt: &'a mut Context,
    instrs: Vec<Option<NodeRef>>,
    id_map: HashMap<SpvId, InstrIdx>,
}
impl<'a> SpirvDeserializer<'a> {
    fn new(ctxt: &'a mut Context, ninstr: usize) -> Self {
        SpirvDeserializer {
            ctxt,
            instrs: std::iter::repeat(None).take(ninstr).collect(),
            id_map: HashMap::new(),
        }
    }
    fn get_instr_by_id(&self, id: SpvId) -> Option<&NodeRef> {
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
            self.instrs[idx] = Some(self.ctxt.alloc_instr(instr));
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
    fn into_instrs(self) -> Vec<NodeRef> {
        const OP_LABEL: u32 = 248;
        const OP_FUNCTION: u32 = 54;
        let expr_idxs = self.id_map.into_iter()
            .map(|(_id, idx)| idx)
            .collect::<HashSet<_>>();

        let out = self.instrs.into_iter()
            .filter_map(|x| x)
            .collect::<Vec<_>>();
        out
    }
}

pub(crate) struct InstructionBuilder<'a> {
    ctxt: &'a SpirvDeserializer<'a>,
    inner: Option<Instruction>,
}
impl<'a> InstructionBuilder<'a> {
    pub fn new(ctxt: &'a SpirvDeserializer, opcode: OpCode) -> Self {
        let inner = Instruction {
            opcode,
            operands: Default::default(),
            next: None
        };
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
                let operand = Operand::Instruction(x.clone());
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
        self.inner
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

pub fn spv2graph<'a>(ctxt: &mut Context, spv: Spv<'a>) -> NodeRef {
    let mut de = SpirvDeserializer::new(ctxt, spv.instrs().len());
    let mut done = true;
    for _ in 0..100 {
        done = true;
        for (i, instr) in spv.instrs().iter().enumerate() {
            // Ignore some in-function debug instructions because they can
            // show up before `OpLabel` which break other processing.
            if is_line_debug_instr(instr) { continue; }
            done &= de.deserialize_instr(i, instr).unwrap();
        }
        if done { break; }
    }
    if !done {
        panic!("unexpected reference complexity");
    }
    let instrs = de.into_instrs();
    for i in 1..instrs.len() {
        let next = Some(instrs[i].clone());
        instrs[i - 1].upgrade().borrow_mut().as_instr_mut().unwrap().next = next;
    }
    instrs[0].clone()
}

pub fn visit<F>(root: NodeRef, mut visitor: F) where F: FnMut(Node) + Clone {
    let mut pending = Vec::new();
    let root = root.upgrade();
    root.collect_children(&mut pending);
    for child in pending {
        visit(child, visitor.clone());
    }
    visitor(root);
}
