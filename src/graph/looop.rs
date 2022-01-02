use std::fmt;
use std::rc::{Rc, Weak};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use crate::spirv::{Instruction, InstructionRef};
use super::{Block, BlockInner, BlockRef};

fn make_loop_name(inner: &Rc<LoopInner>) -> String {
    format!("Loop@{:016x}",
        (Rc::as_ptr(inner) as *const LoopInner) as usize)
}
fn make_loop_name_weak(inner: &Weak<LoopInner>) -> String {
    inner.upgrade()
        .map(|x| make_loop_name(&x))
        .unwrap_or("Loop@DROPPED".to_owned())
}

pub struct LoopInner {
    /// From the converge node to the diverge node, excluding the diverge node.
    /// The first block is the provoking node.
    forward_blocks: Vec<BlockRef>,
    /// From the diverge node to the converge node, excluding the converge node.
    /// The first block is the branching node deciding whether the loop should
    /// continue.
    backward_blocks: Vec<BlockRef>,
}

#[derive(Clone)]
pub struct Loop(Rc<LoopInner>);
impl Deref for Loop {
    type Target = LoopInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl DerefMut for Loop {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Rc::get_mut(&mut self.0).unwrap()
    }
}
impl Loop {
    pub fn new(
        forward_blocks: Vec<BlockRef>,
        backward_blocks: Vec<BlockRef>,
    ) -> Loop {
        let inner = LoopInner { forward_blocks, backward_blocks };
        Loop(Rc::new(inner))
    }

    pub fn len(&self) -> usize {
        self.0.forward_blocks.len() + self.0.backward_blocks.len()
    }
    pub fn provoking_block(&self) -> &BlockRef {
        self.0.forward_blocks.first().unwrap()
    }
    pub fn branching_block(&self) -> &BlockRef {
        self.0.backward_blocks.first().unwrap()
    }
    pub fn forward_blocks(&self) -> &[BlockRef] {
        &self.0.forward_blocks
    }
    pub fn backward_blocks(&self) -> &[BlockRef] {
        &self.0.forward_blocks
    }
    pub fn blocks(&self) -> impl Iterator<Item=&BlockRef> {
        self.forward_blocks().iter()
            .chain(self.backward_blocks().iter())
    }
    pub fn downgrade(&self) -> LoopRef {
        let out = Rc::downgrade(&self.0);
        LoopRef(out)
    }
}
impl fmt::Debug for Loop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{} ", make_loop_name(&self.0)))?;
        f.debug_list()
            .entries(self.forward_blocks())
            .entry(&"[branch]")
            .entries(self.backward_blocks())
            .finish()
    }
}
impl PartialEq for Loop {
    fn eq(&self, b: &Self) -> bool {
        Rc::ptr_eq(&self.0, &b.0)
    }
}
impl Eq for Loop {}
impl Hash for Loop {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (Rc::as_ptr(&self.0) as usize).hash(state);
    }
}

#[derive(Clone)]
pub struct LoopRef(Weak<LoopInner>);
impl LoopRef {
    pub fn upgrade(&self) -> Option<Loop> {
        let out = self.0.upgrade();
        out.map(|x| Loop(x))
    }
}
impl fmt::Debug for LoopRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&make_loop_name_weak(&self.0))
    }
}
impl PartialEq for LoopRef {
    fn eq(&self, b: &Self) -> bool {
        self.0.ptr_eq(&b.0)
    }
}
impl Eq for LoopRef {}
impl Hash for LoopRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.0.as_ptr() as usize).hash(state);
    }
}
