use std::collections::HashMap;
use std::fmt;
use std::rc::{Rc, Weak};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use crate::spirv::{Instruction, InstructionRef, Operand};
use crate::graph::{Block, BlockRef};
use super::OP_LABEL;

fn make_graph_name(inner: &Rc<GraphInner>) -> String {
    format!("Graph@{:016x}",
        (Rc::as_ptr(inner) as *const GraphInner) as usize)
}
fn make_graph_name_weak(inner: &Weak<GraphInner>) -> String {
    inner.upgrade()
        .map(|x| make_graph_name(&x))
        .unwrap_or("Graph@DROPPED".to_owned())
}

pub struct GraphInner {
    blocks: Vec<BlockRef>,
    edges: HashMap<BlockRef, Vec<BlockRef>>
}
impl GraphInner {
    pub fn blocks(&self) -> &[BlockRef] {
        &self.blocks
    }
    pub fn provoking_block(&self) -> &BlockRef {
        self.blocks.first().unwrap()
    }

    pub fn get_dst(&self, src: &BlockRef) -> &[BlockRef] {
        self.edges.get(src)
            .map(|x| x as &[BlockRef])
            .unwrap_or(&[] as &[BlockRef])
    }
}

#[derive(Clone)]
pub struct Graph(Rc<GraphInner>);
impl Deref for Graph {
    type Target = GraphInner;
    fn deref(&self) -> &GraphInner {
        self.0.deref()
    }
}
impl DerefMut for Graph {
    fn deref_mut(&mut self) -> &mut GraphInner {
        Rc::get_mut(&mut self.0).unwrap()
    }
}
impl Graph {
    pub fn downgrade(&self) -> GraphRef {
        let out = Rc::downgrade(&self.0);
        GraphRef(out)
    }
}
impl<I: IntoIterator<Item=BlockRef>> From<I> for Graph {
    fn from(blocks: I) -> Self {
        let blocks = blocks.into_iter().collect::<Vec<_>>();
        let dsts_by_src = blocks.iter()
            .map(|src_block| {
                let src_block = src_block.upgrade().unwrap();
                let src_branch = src_block.branch_instr().upgrade().unwrap();
                let candidates = src_branch.operands();
                let dsts = candidates.iter()
                    .filter_map(|candidate| {
                        if let Operand::Instruction(dst_label) = candidate {
                            if dst_label.upgrade().unwrap().opcode() == OP_LABEL {
                                let dst_block = blocks.iter()
                                    .find(|x| x.upgrade().unwrap().label_instr() == dst_label)
                                    .expect("uncaptured block in graph is not allowed");
                                Some(dst_block)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .cloned()
                    .collect::<Vec<BlockRef>>();
                dsts
            });
        let edges = blocks.iter()
            .cloned()
            .zip(dsts_by_src)
            .collect::<HashMap<_, _>>();
        let inner = GraphInner { blocks, edges };
        Graph(Rc::new(inner))
    }
}
impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let blocks = self.blocks.iter()
            .map(|x| x.upgrade().unwrap())
            .collect::<Vec<_>>();
        f.write_str(&format!("{} ", make_graph_name(&self.0)))?;
        f.debug_list()
            .entries(blocks)
            .finish()
    }
}
impl PartialEq for Graph {
    fn eq(&self, b: &Self) -> bool {
        Rc::ptr_eq(&self.0, &b.0)
    }
}
impl Eq for Graph {}
impl Hash for Graph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (Rc::as_ptr(&self.0) as usize).hash(state);
    }
}

#[derive(Clone)]
pub struct GraphRef(Weak<GraphInner>);
impl GraphRef {
    pub fn upgrade(&self) -> Option<Graph> {
        let out = self.0.upgrade();
        out.map(|x| Graph(x))
    }
}
impl fmt::Debug for GraphRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&make_graph_name_weak(&self.0))
    }
}
impl PartialEq for GraphRef {
    fn eq(&self, b: &Self) -> bool {
        self.0.ptr_eq(&b.0)
    }
}
impl Eq for GraphRef {}
impl Hash for GraphRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.0.as_ptr() as usize).hash(state);
    }
}
