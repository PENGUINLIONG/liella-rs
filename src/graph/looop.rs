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
    blocks: Vec<BlockRef>,
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
    pub fn len(&self) -> usize {
        self.0.blocks.len()
    }
    pub fn provoking_block(&self) -> &BlockRef {
        self.0.blocks.first().unwrap()
    }
    pub fn blocks(&self) -> &[BlockRef] {
        &self.0.blocks
    }
    pub fn downgrade(&self) -> LoopRef {
        let out = Rc::downgrade(&self.0);
        LoopRef(out)
    }
}
impl fmt::Debug for Loop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let blocks = self.blocks.iter()
            .map(|x| x.upgrade().unwrap())
            .collect::<Vec<_>>();
        f.write_str(&format!("{} ", make_loop_name(&self.0)))?;
        f.debug_list()
            .entries(blocks)
            .finish()
    }
}
impl From<Vec<BlockRef>> for Loop {
    fn from(blocks: Vec<BlockRef>) -> Self {
        let inner = LoopInner { blocks };
        Loop(Rc::new(inner))
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
