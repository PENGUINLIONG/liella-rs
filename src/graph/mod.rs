use std::convert::TryFrom;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use std::fmt;
use crate::spirv::{Instruction, InstructionRef, OpCode, Operand, Spirv};
use crate::error::{LiellaError as Error, LiellaResult as Result};

mod block;
mod looop; // It's not a typo my friend.
mod unfold_fn_vars;

use block::{Block, BlockInner, BlockRef};
use looop::{Loop, LoopInner, LoopRef};

const OP_STORE: OpCode = 62;
const OP_LABEL: OpCode = 248;
const OP_BRANCH: u32 = 249;
const OP_BRANCH_CONDITIONAL: u32 = 250;
const OP_SWITCH: u32 = 251;
const OP_KILL: u32 = 252;
const OP_RETURN: u32 = 253;
const OP_RETURN_VALUE: u32 = 254;
const OP_UNREACHABLE: u32 = 255;


#[derive(Clone)]
pub enum Node {
    Instruction(InstructionRef),
    Block(BlockRef),
    Loop(LoopRef),
}
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Instruction(instr) => {
                instr.upgrade().unwrap().fmt(f)
            },
            Node::Block(block) => {
                block.upgrade().unwrap().fmt(f)
            },
            Node::Loop(looop) => {
                looop.upgrade().unwrap().fmt(f)
            },
        }
    }
}


#[derive(Clone, Debug)]
struct Edge {
    pub src: BlockRef,
    pub dst: BlockRef,
}


#[derive(Default)]
struct GraphIntermediate {
    nodes: Option<Vec<Node>>,
    edges: HashMap<BlockRef, Vec<BlockRef>>,
    instr_pool: Vec<Instruction>,
    block_pool: Vec<Block>,
    loop_pool: Vec<Loop>,
}
impl GraphIntermediate {
    fn new(instrs: &[InstructionRef]) -> GraphIntermediate {
        //let (instrs, instr_pool) = unfold_fn_vars::apply(instrs);
        let instr_pool = instrs.iter()
            .map(|x| x.upgrade().unwrap())
            .collect::<Vec<_>>();
        let nodes = instrs.iter()
            .cloned()
            .map(|x| Node::Instruction(x))
            .collect();
        GraphIntermediate {
            nodes: Some(nodes),
            instr_pool,
            ..Default::default()
        }
    }

    fn elevate_blocks(&mut self) -> Result<()> {
        let mut it = self.nodes.take().unwrap().into_iter();

        let mut out: Vec<Node> = Vec::new();
        let mut block_pool: Vec<Block> = Vec::new();
        let mut block_instrs: Option<Vec<InstructionRef>> = None;
        while let Some(node) = it.next() {
            if let Node::Instruction(instr) = node {
                match instr.upgrade().unwrap().opcode() {
                    OP_LABEL => {
                        if block_instrs.is_some() {
                            return Err(Error::UNEXPECTED_OP);
                        }
                        block_instrs = Some(vec![instr.clone()]);
                    },
                    OP_BRANCH | OP_BRANCH_CONDITIONAL | OP_SWITCH | OP_KILL |
                        OP_RETURN | OP_RETURN_VALUE | OP_UNREACHABLE =>
                    {
                        let mut instrs = block_instrs.take()
                            .ok_or(Error::UNEXPECTED_OP)?;
                        instrs.push(instr.clone());
                        let block = Block::from(instrs);
                        out.push(Node::Block(block.downgrade()));
                        block_pool.push(block);
                    },
                    _ => {
                        if let Some(instrs) = block_instrs.as_mut() {
                            instrs.push(instr.clone());
                        } else {
                            out.push(Node::Instruction(instr));
                        }
                    },
                }
            } else {
                out.push(node);
            }
        }
        self.nodes = Some(out);
        self.block_pool = block_pool;
        Ok(())
    }
    fn collect_edges(&mut self) -> Result<()> {
        let blocks = self.block_pool.iter()
            .map(|x| (x.label_instr().clone(), x.downgrade()))
            .collect::<HashMap<InstructionRef, BlockRef>>();
        let mut out = HashMap::<BlockRef, Vec<BlockRef>>::new();
        for block in self.block_pool.iter() {
            let src = block.downgrade();
            let branch_instr = block.branch_instr().upgrade().unwrap();
            for operand in branch_instr.operands() {
                if let Operand::Instruction(dst_label) = operand {
                    // Here we assume that if `dst_label` can be found as keys
                    // in `blocks`, it is guaranteed to be a `OpLabel`.
                    if let Some(dst) = blocks.get(dst_label) {
                        out.entry(src.clone()).or_default().push(dst.clone());
                    }
                }
            }
        }
        self.edges = out;
        Ok(())
    }



    fn collect_loops_impl(
        &self,
        block_stack: &mut Vec<BlockRef>,
        out: &mut Vec<Loop>,
    ) -> Result<()> {
        let src_block = block_stack.last().unwrap().clone();
        if let Some(dst_blocks) = self.edges.get(&src_block) {
            for dst_block in dst_blocks {
                let mut is_back_edge = false;
                let loops = block_stack.iter()
                    .enumerate()
                    .rev() // `rev` to find the smallest structured loop.
                    .filter_map(|(i, ancester_block)| {
                        if ancester_block == dst_block {
                            is_back_edge = true;
                            Some(i)
                        } else { None }
                    })
                    .map(|i| Loop::from(block_stack[i..].to_owned()));
                out.extend(loops);

                if !is_back_edge {
                    block_stack.push(dst_block.clone());
                    self.collect_loops_impl(block_stack, out)?;
                    block_stack.pop();
                }
            }
        }
        Ok(())
    }
    fn elevate_loops(&mut self) -> Result<()> {
        let src_blocks = self.edges.keys()
            .cloned()
            .collect::<HashSet<_>>();
        let dst_blocks = self.edges.values()
            .flatten()
            .cloned()
            .collect::<HashSet<_>>();
        let provoking_blocks = src_blocks.difference(&dst_blocks);

        let mut loop_pool = Vec::new();
        for provoking_block in provoking_blocks {
            let mut block_stack = vec![provoking_block.clone()];
            self.collect_loops_impl(&mut block_stack, &mut loop_pool)?;
        }

        let mut loops_by_len = BTreeMap::<usize, HashMap<BlockRef, Loop>>::new();
        for looop in loop_pool.iter() {
            loops_by_len.entry(looop.len())
                .or_default()
                .insert(looop.provoking_block().clone(), looop.clone());
        }

        let mut nodes = self.nodes.take().unwrap();
        let mut visited_blocks = HashSet::new();
        for (_, loops) in loops_by_len {
            nodes = nodes.into_iter()
                .filter_map(|node| {
                    match node {
                        Node::Block(block) => {
                            if !visited_blocks.contains(&block) {
                                if let Some(looop) = loops.get(&block) {
                                    visited_blocks.extend(looop.blocks().iter().cloned());
                                    let node = Node::Loop(looop.downgrade());
                                    Some(node)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        },
                        node => Some(node),
                    }
                })
                .collect();
        }
        self.nodes = Some(nodes);
        self.loop_pool = loop_pool;
        Ok(())
    }
}
















#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    instr_pool: Vec<Instruction>,
    block_pool: Vec<Block>,
    loop_pool: Vec<Loop>,
}
impl Graph {
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }
}
impl From<GraphIntermediate> for Graph {
    fn from(x: GraphIntermediate) -> Graph {
        Graph {
            nodes: x.nodes.unwrap(),
            instr_pool: x.instr_pool,
            block_pool: x.block_pool,
            loop_pool: x.loop_pool,
        }
    }
}

fn parse_grpah(spv: &Spirv) -> Result<Graph> {
    let mut itm = GraphIntermediate::new(spv.stmts());
    itm.elevate_blocks()?;
    itm.collect_edges()?;
    itm.elevate_loops()?;

    Ok(itm.into())
}

impl TryFrom<&Spirv> for Graph {
    type Error = Error;
    fn try_from(spv: &Spirv) -> Result<Self> {
        let out = parse_grpah(spv)?;
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inline_spirv::inline_spirv;
    #[test]
    fn test_parse() {
        let spv: &'static [u32] = inline_spirv!(r#"
            #version 450
            layout(location=0) in int pred;
            layout(location=0) out int ans;
            void main() {
                if (pred > 0) { ans = 0; } else { ans = 1; }
            }
        "#, vert, vulkan1_0);
        let spv = crate::spv::Spv::try_from(spv).unwrap();
        let spv = Spirv::try_from(spv).unwrap();
        let graph = Graph::try_from(&spv).unwrap();
    }
}
