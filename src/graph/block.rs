use std::fmt;
use std::rc::{Rc, Weak};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use crate::spirv::{Instruction, InstructionRef};

fn make_block_name(inner: &Rc<BlockInner>) -> String {
    format!("Block@{:016x}",
        (Rc::as_ptr(inner) as *const BlockInner) as usize)
}
fn make_block_name_weak(inner: &Weak<BlockInner>) -> String {
    inner.upgrade()
        .map(|x| make_block_name(&x))
        .unwrap_or("Block@DROPPED".to_owned())
}

pub struct BlockInner {
    instrs: Vec<InstructionRef>,
}
impl BlockInner {
    pub fn instrs(&self) -> &[InstructionRef] {
        &self.instrs
    }
    pub fn label_instr(&self) -> &InstructionRef {
        self.instrs.first().unwrap()
    }
    pub fn branch_instr(&self) -> &InstructionRef {
        self.instrs.last().unwrap()
    }
}

#[derive(Clone)]
pub struct Block(Rc<BlockInner>);
impl Deref for Block {
    type Target = BlockInner;
    fn deref(&self) -> &BlockInner {
        self.0.deref()
    }
}
impl DerefMut for Block {
    fn deref_mut(&mut self) -> &mut BlockInner {
        Rc::get_mut(&mut self.0).unwrap()
    }
}
impl Block {
    pub fn downgrade(&self) -> BlockRef {
        let out = Rc::downgrade(&self.0);
        BlockRef(out)
    }
}
impl From<Vec<InstructionRef>> for Block {
    fn from(instrs: Vec<InstructionRef>) -> Self {
        let inner = BlockInner { instrs };
        Block(Rc::new(inner))
    }
}
impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let instrs = self.instrs.iter()
            .map(|x| x.upgrade().unwrap())
            .collect::<Vec<_>>();
        f.write_str(&format!("{} ", make_block_name(&self.0)))?;
        f.debug_list()
            .entries(instrs)
            .finish()
    }
}
impl PartialEq for Block {
    fn eq(&self, b: &Self) -> bool {
        Rc::ptr_eq(&self.0, &b.0)
    }
}
impl Eq for Block {}
impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (Rc::as_ptr(&self.0) as usize).hash(state);
    }
}

#[derive(Clone)]
pub struct BlockRef(Weak<BlockInner>);
impl BlockRef {
    pub fn upgrade(&self) -> Option<Block> {
        let out = self.0.upgrade();
        out.map(|x| Block(x))
    }
}
impl fmt::Debug for BlockRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&make_block_name_weak(&self.0))
    }
}
impl PartialEq for BlockRef {
    fn eq(&self, b: &Self) -> bool {
        self.0.ptr_eq(&b.0)
    }
}
impl Eq for BlockRef {}
impl Hash for BlockRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.0.as_ptr() as usize).hash(state);
    }
}
