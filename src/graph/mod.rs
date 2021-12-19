use std::convert::TryFrom;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use std::fmt;
use crate::spirv::{Instruction, InstructionRef, OpCode, Operand, Spirv};
use crate::error::{LiellaError as Error, LiellaResult as Result};

mod block;
mod graph;
mod looop; // It's not a typo my friend.
mod unfold_fn_vars;

use block::{Block, BlockInner, BlockRef};
use graph::{Graph, GraphInner, GraphRef};
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


#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Node {
    Instruction(InstructionRef),
    Block(BlockRef),
    Graph(GraphRef),
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
            Node::Graph(graph) => {
                graph.upgrade().unwrap().fmt(f)
            }
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
struct ContextIntermediate {
    nodes: Option<Vec<Node>>,
    edges: HashMap<BlockRef, Vec<BlockRef>>,
    instr_pool: Vec<Instruction>,
    block_pool: Vec<Block>,
    graph_pool: Vec<Graph>,
    loop_pool: Vec<Loop>,
}
impl ContextIntermediate {
    fn new(instrs: &[InstructionRef]) -> ContextIntermediate {
        //let (instrs, instr_pool) = unfold_fn_vars::apply(instrs);
        let instr_pool = instrs.iter()
            .map(|x| x.upgrade().unwrap())
            .collect::<Vec<_>>();
        let nodes = instrs.iter()
            .cloned()
            .map(|x| Node::Instruction(x))
            .collect();
        ContextIntermediate {
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

    fn elevate_graphs(&mut self) -> Result<()> {
        let dst_blocks = self.block_pool.iter()
            .map(|x| {
                let branch_instr = x.branch_instr().upgrade().unwrap();
                branch_instr.operands().iter()
                    .filter_map(|x| {
                        if let Operand::Instruction(dst) = x {
                            self.block_pool.iter().find(|x| {
                                x.label_instr() == dst
                            })
                        } else {
                            None
                        }
                    })
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let edges = self.block_pool.iter()
            .cloned()
            .zip(dst_blocks)
            .collect::<HashMap<_, _>>();

        // Step 1: Find the provoking blocks, which don't have edges pointing to
        // them. A provoking block is usually the first block in a function
        // which is the root of a graph.
        let provoking_blocks = {
            let src_blocks = edges.keys()
                .cloned()
                .collect::<HashSet<_>>();
            let dst_blocks = edges.values()
                .flat_map(|x| x.iter().cloned())
                .collect::<HashSet<_>>();
            src_blocks.difference(&dst_blocks)
                .cloned()
                .collect::<HashSet<_>>()
        };

        // Step 2: Capture all directly and indirectly referenced blocks and
        // build a self-enclosed graph.
        fn collect_graph_blocks_impl(
            src_block: &Block,
            edges: &HashMap<Block, Vec<Block>>,
            out: &mut HashSet<BlockRef>
        ) {
            for dst_block in edges.get(src_block).unwrap() {
                if out.insert(dst_block.downgrade()) {
                    collect_graph_blocks_impl(dst_block, edges, out);
                }
            }
        }
        let graphs = provoking_blocks.iter()
            .map(|x| {
                let mut out = HashSet::new();
                out.insert(x.downgrade());
                collect_graph_blocks_impl(x, &edges, &mut out);
                (x.clone(), Graph::from(out))
            })
            .collect::<HashMap<Block, Graph>>();

        let nodes = self.nodes.take().unwrap().into_iter()
            .filter_map(|node| {
                if let Node::Block(block) = node {
                    let block = block.upgrade().unwrap();
                    if provoking_blocks.contains(&block) {
                        let graph = graphs.get(&block).unwrap().downgrade();
                        let graph = Node::Graph(graph);
                        Some(graph)
                    } else {
                        // Hide non-provoking blocks because they have been
                        // represented by graphs of provoking blocks.
                        None
                    }
                } else {
                    Some(node)
                }
            })
            .collect::<Vec<_>>();
        self.nodes = Some(nodes);
        self.graph_pool = graphs.into_values().collect();

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
pub struct Context {
    nodes: Vec<Node>,
    instr_pool: Vec<Instruction>,
    block_pool: Vec<Block>,
    graph_pool: Vec<Graph>,
    loop_pool: Vec<Loop>,
}
impl Context {
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }
}
impl From<ContextIntermediate> for Context {
    fn from(x: ContextIntermediate) -> Context {
        Context {
            nodes: x.nodes.unwrap(),
            instr_pool: x.instr_pool,
            block_pool: x.block_pool,
            graph_pool: x.graph_pool,
            loop_pool: x.loop_pool,
        }
    }
}

fn parse_grpah(spv: &Spirv) -> Result<Context> {
    let mut itm = ContextIntermediate::new(spv.stmts());
    itm.elevate_blocks()?;
    itm.elevate_graphs()?;
    //itm.elevate_loops()?;

    Ok(itm.into())
}

impl TryFrom<&Spirv> for Context {
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
        let graph = Context::try_from(&spv).unwrap();
    }
}
