use std::convert::TryFrom;
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use std::fmt;
use crate::spirv::{Instruction, InstructionRef, OpCode, Operand, Spirv};
use crate::error::{LiellaError as Error, LiellaResult as Result};

const OP_STORE: OpCode = 62;
const OP_LABEL: OpCode = 248;
const OP_BRANCH: u32 = 249;
const OP_BRANCH_CONDITIONAL: u32 = 250;
const OP_SWITCH: u32 = 251;
const OP_KILL: u32 = 252;
const OP_RETURN: u32 = 253;
const OP_RETURN_VALUE: u32 = 254;
const OP_UNREACHABLE: u32 = 255;

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
    pub fn label_instr(&self) -> Instruction {
        self.instrs.first().unwrap().upgrade().unwrap()
    }
    pub fn branch_instr(&self) -> Instruction {
        self.instrs.last().unwrap().upgrade().unwrap()
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
        f.debug_struct(&make_block_name(&self.0))
            .field("instrs", &self.instrs)
            .finish()
    }
}
impl PartialEq for Block {
    fn eq(&self, b: &Self) -> bool {
        Rc::ptr_eq(&self.0, &b.0)
    }
}
impl Eq for Block {}

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

#[derive(Clone, Debug)]
pub struct GraphEdge {
    pub src: BlockRef,
    pub dst: BlockRef,
}
fn collect_edges(
    blocks: &HashMap<InstructionRef, Block>,
) -> Result<Vec<GraphEdge>> {
    let mut out = Vec::new();
    for block in blocks.values() {
        let src = block.downgrade();
        for operand in block.branch_instr().operands() {
            if let Operand::Instruction(dst_label) = operand {
                if dst_label.upgrade().unwrap().opcode() == OP_LABEL {
                    let dst = blocks.get(&dst_label)
                        .ok_or(Error::UNEXPECTED_OP)?
                        .downgrade();
                    let edge = GraphEdge { src: src.clone(), dst };
                    out.push(edge);
                }
            }
        }
    }
    Ok(out)
}

fn find_itervar(instr: Instruction) -> Option<Instruction> {
    if instr.opcode() == OP_STORE {
        let operands = instr.operands();
        let candidate = operands[0].as_instr().and_then(|x| x.upgrade())?;
        let value = operands[1].as_instr().and_then(|x| x.upgrade())?;
        if find_itervar_impl(&candidate, &value) {
            Some(candidate)
        } else {
            None
        }
    } else {
        None
    }
}
fn find_itervar_impl(candidate: &Instruction, instr: &Instruction) -> bool {
    if candidate == instr {
        true
    } else {
        instr.operands().iter()
            .any(|x| {
                if let Some(child_instr) = x.as_instr() {
                    let child_instr = child_instr.upgrade().unwrap();
                    find_itervar_impl(candidate, &child_instr)
                } else {
                    false
                }
            })
    }
}
fn find_itervars(instrs: &[InstructionRef]) -> Vec<Instruction> {
    instrs.iter()
        .filter_map(|x| find_itervar(x.upgrade().unwrap()))
        .collect::<Vec<_>>()
}

#[derive(Clone, Debug)]
pub struct GraphLoop {
    /// Definition of iteration variable instruction:
    /// 1. Autonomously mutate inside a loop, forming a dynamic system of its
    ///    own; thus can conclude a recursive function for each itervar.
    /// 2. Exert a force to drive mutation of other variables/states.
    /// 3. Atomic variables are NEVER itervars, because their mutation are not
    ///    guaranteed.
    ///
    /// In our SPIR-V case, an `OpStore` of variable using an `OpLoad`ed value
    /// from that same variable makes an itervar. Other variables referred to
    /// the itervar receives a driving force to mutate.
    pub itervars: HashSet<InstructionRef>,
    /// Edges from the back edge destination to the conditional-branch block.
    /// This ought to be empty if there is no conditional branch in loop, i.e.,
    /// an infinite loop.
    pub header_edges: Vec<GraphEdge>,
    /// Edges from the conditional-branch block to the destination of a back
    /// edge. The back edge is at the end of `body_edges`.
    pub body_edges: Vec<GraphEdge>,
}
impl GraphLoop {
    pub fn edges(&self) -> impl Iterator<Item=&GraphEdge> {
        let header_edges = self.header_edges.iter();
        let body_edges = self.body_edges.iter();
        header_edges.chain(body_edges)
    }
    pub fn blocks(&self) -> impl Iterator<Item=&BlockRef> {
        self.edges().map(|x| &x.src)
    }
    pub fn selection_block(&self) -> Option<Block> {
        self.header_edges.last()
            .and_then(|x| x.dst.upgrade())
    }
    pub fn selection_instr(&self) -> Option<InstructionRef> {
        self.selection_block()
            .and_then(|x| x.instrs().last().cloned())
    }
}
fn collect_loops_impl(
    edges: &[GraphEdge],
    block_stack: &mut Vec<BlockRef>,
    out: &mut Vec<GraphLoop>,
) {
    let src_block = block_stack.last().unwrap().clone();
    let dst_blocks = edges.iter()
        .filter_map(|x| {
            if &x.src == &src_block {
                Some(x.dst.clone())
            } else {
                None
            }
        });
    for dst_block in dst_blocks {
        let mut is_back_edge = false;
        for (i, ancester_block) in block_stack.iter().enumerate() {
            if ancester_block == &dst_block {
                let itervars = block_stack[i..].iter()
                    .flat_map(|x| {
                        let block = x.upgrade().unwrap();
                        find_itervars(block.instrs())
                    })
                    .map(|x| x.downgrade())
                    .collect::<HashSet<_>>();
                // One of the ancester is the destination block of this edge.
                // The current edge is an back edge.
                let mut loop_edges = block_stack[i..].windows(2)
                    .map(|pair| GraphEdge {
                        src: pair[0].clone(),
                        dst: pair[1].clone(),
                    })
                    .chain([GraphEdge {
                        src: src_block.clone(),
                        dst: dst_block.clone(),
                    }])
                    .collect::<Vec<_>>();
                // We assume the header edges ends at the first conditional
                // branch, which include all branches with multiple (more than
                // one) destinations.
                // TODO: (penguinliong) Need another pass to merge same-source
                // loops which have been split off because of branches in the
                // header. In such case the loop header should end after the
                // first conditional branch
                let icond_edge = loop_edges.iter()
                    .position(|x| {
                        let src_block = x.src.upgrade().unwrap();
                        let ndst = src_block.instrs.last()
                            .unwrap()
                            .upgrade()
                            .unwrap()
                            .operands()
                            .iter()
                            .filter(|x| {
                                if let Operand::Instruction(instr) = x {
                                    let instr = instr.clone()
                                        .upgrade()
                                        .unwrap();
                                    instr.opcode() == OP_LABEL
                                } else {
                                    false
                                }
                            })
                            .count();
                        ndst > 1
                    })
                    .unwrap_or(0);
                let body_edges = loop_edges.split_off(icond_edge);
                loop_edges.shrink_to_fit();
                let header_edges = loop_edges;
                out.push(GraphLoop {
                    itervars, header_edges, body_edges
                });
                is_back_edge = true;
                break;
            }
        }

        block_stack.push(dst_block.clone());
        if !is_back_edge {
            collect_loops_impl(edges, block_stack, out);
        }
        block_stack.pop();
    }
}
fn collect_loops(
    edges: &[GraphEdge]
) -> Vec<GraphLoop> {
    if edges.is_empty() { return Default::default(); }
    let mut block_stack = vec![edges[0].src.clone()];
    let mut out = Vec::new();
    collect_loops_impl(edges, &mut block_stack, &mut out);
    out
}

#[derive(Debug)]
pub struct Graph {
    blocks: Vec<Block>,
    edges: Vec<GraphEdge>,
    loops: Vec<GraphLoop>,
}
impl Graph {
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }
    pub fn edges(&self) -> &[GraphEdge] {
        &self.edges
    }
    pub fn loops(&self) -> &[GraphLoop] {
        &self.loops
    }
}

fn parse_grpah(spv: &Spirv) -> Result<Graph> {
    let mut blocks: HashMap<InstructionRef, Block> = HashMap::new();
    let mut cur_block_beg: Option<usize> = None;

    for (i, instr) in spv.stmts().iter().enumerate() {
        let instr = instr.upgrade().unwrap();
        match instr.opcode() {
            OP_LABEL => {
                if cur_block_beg.is_some() {
                    return Err(Error::UNEXPECTED_OP);
                }
                cur_block_beg = Some(i);
            },
            OP_BRANCH | OP_BRANCH_CONDITIONAL | OP_SWITCH | OP_KILL |
                OP_RETURN | OP_RETURN_VALUE | OP_UNREACHABLE =>
            {
                if let Some(beg) = cur_block_beg.take() {
                    let label_instr = spv.stmts()[beg].clone();
                    let inner = BlockInner {
                        instrs: spv.stmts()[beg..=i].to_owned(),
                    };
                    let block = Block(Rc::new(inner));
                    blocks.insert(label_instr, block);
                } else {
                    return Err(Error::UNEXPECTED_OP);
                }
            }
            _ => {},
        }
    }

    let edges = collect_edges(&blocks)?;
    let loops = collect_loops(&edges);
    let blocks = blocks.into_values().collect::<Vec<_>>();
    let out = Graph { blocks, edges, loops };
    Ok(out)
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
        // Selection + true + false + merge -> 4 blocks.
        assert_eq!(graph.blocks().len(), 4);
    }
}
