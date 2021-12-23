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


#[derive(Default)]
struct ContextIntermediate {
    nodes: Option<Vec<Node>>,
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

    fn elevate_loops(&mut self) -> Result<()> {
        use std::collections::hash_map::{Entry, OccupiedEntry, VacantEntry};

        let mut graph_loops = HashMap::<GraphRef, Vec<LoopRef>>::new();
        for node in self.nodes.take().unwrap() {
            if let Node::Graph(graph) = node {
                let graph = graph.upgrade().unwrap();
                let edges = graph.edges();

                let mut in_degs = HashMap::<BlockRef, u32>::new();
                let mut out_degs = HashMap::<BlockRef, u32>::new();
                for (src_block, dst_block) in edges.iter() {
                    *out_degs.entry(src_block.clone()).or_default() += 1;
                    *in_degs.entry(dst_block.clone()).or_default() += 1;
                }

                let mut diverges = Vec::new();
                let mut converges = Vec::new();
                in_degs.keys()
                    .for_each(|x| {
                        let in_deg = in_degs.get(&x).cloned().unwrap_or(1);
                        let out_deg = out_degs.get(&x).cloned().unwrap_or(1);
                        if in_deg < out_deg {
                            diverges.push(x.clone());
                        }
                        if in_deg > out_deg {
                            converges.push(x.clone());
                        }
                    });

                println!("{:#?}", edges);

                println!("{:?}", diverges);
                println!("{:?}", converges);

                fn find_loop_impl(
                    graph: &Graph,
                    stack: &mut Vec<BlockRef>,
                ) -> bool {
                    let target = stack.iter().next().unwrap().clone();
                    let src = stack.iter().last().unwrap().clone();
                    for dst in graph.get_dst_blocks(&src) {
                        stack.push(dst.clone());
                        if &target == dst || find_loop_impl(graph, stack) {
                            return true;
                        }
                        stack.pop();
                    }
                    false
                }
                fn find_loop(
                    graph: &Graph,
                    src: &BlockRef,
                ) -> Option<Vec<BlockRef>> {
                    let mut stack = vec![src.clone()];
                    if find_loop_impl(graph, &mut stack) {
                        stack.pop();
                        Some(stack)
                    } else {
                        None
                    }
                }

                let mut found_loops = HashSet::<Vec<BlockRef>>::new();
                while let Some(mut min_loop) = diverges.iter()
                    .filter_map(|diverge| {
                        if let Some(looop) = find_loop(&graph, diverge) {
                            if !found_loops.contains(&looop) {
                                found_loops.insert(looop.clone());
                                return Some(looop)
                            }
                        }
                        None
                    })
                    .min_by_key(|x| x.len())
                {
                    if let Some(iconverge) = min_loop.iter()
                        .position(|x| converges.contains(x))
                    {
                        let header = min_loop.drain(..iconverge)
                            .collect::<Vec<_>>();
                        let select = min_loop.remove(0);
                        let body = min_loop;
                        println!("{:?}-{:?}-{:?}", header, select, body);
                    } else {
                        // If a converge point cannot be found, it's not a
                        // structured loop. We don't handle such case.
                    }
                }

                let loops: Vec<Loop> = found_loops.into_iter()
                    .map(Loop::from)
                    .collect();
                let loop_refs = loops.iter()
                    .map(Loop::downgrade)
                    .collect::<Vec<LoopRef>>();
                graph_loops.insert(graph.downgrade(), loop_refs);
            }

        }

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
    itm.elevate_loops()?;

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
