use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use std::fmt;
use crate::spirv::{Instruction, OpCode, Spirv};
use crate::error::{LiellaError as Error, LiellaResult as Result};

fn make_block_name<'a>(inner: &Rc<BlockInner<'a>>) -> String {
    format!("Block@{:016x}", (Rc::as_ptr(inner) as *const BlockInner<'a>) as usize)
}
fn make_block_name_weak<'a>(inner: &Weak<BlockInner<'a>>) -> String {
    inner.upgrade()
        .map(|x| make_block_name(&x))
        .unwrap_or("Block@DROPPED".to_owned())
}

pub struct BlockInner<'a> {
    instrs: &'a [Instruction],
}
impl<'a> BlockInner<'a> {
    pub fn instrs(&self) -> &'a [Instruction] {
        &self.instrs
    }
    pub fn label_instr(&self) -> &'a Instruction {
        self.instrs.first().unwrap()
    }
    pub fn branch_instr(&self) -> &'a Instruction {
        self.instrs.last().unwrap()
    }
}

#[derive(Clone)]
pub struct Block<'a>(Rc<BlockInner<'a>>);
impl<'a> Deref for Block<'a> {
    type Target = BlockInner<'a>;
    fn deref(&self) -> &BlockInner<'a> {
        self.0.deref()
    }
}
impl<'a> DerefMut for Block<'a> {
    fn deref_mut(&mut self) -> &mut BlockInner<'a> {
        Rc::get_mut(&mut self.0).unwrap()
    }
}
impl<'a> Block<'a> {
    pub fn downgrade(self) -> BlockRef<'a> {
        let out = Rc::downgrade(&self.0);
        BlockRef(out)
    }
}
impl<'a> fmt::Debug for Block<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct(&make_block_name(&self.0))
            .field("instrs", &self.instrs)
            .finish()
    }
}
impl<'a> PartialEq for Block<'a> {
    fn eq(&self, b: &Self) -> bool {
        Rc::ptr_eq(&self.0, &b.0)
    }
}
impl<'a> Eq for Block<'a> {}

#[derive(Clone)]
pub struct BlockRef<'a>(Weak<BlockInner<'a>>);
impl<'a> BlockRef<'a> {
    pub fn upgrade(self) -> Option<Block<'a>> {
        let out = self.0.upgrade();
        out.map(|x| Block(x))
    }
}
impl<'a> fmt::Debug for BlockRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&make_block_name_weak(&self.0))
    }
}
impl<'a> PartialEq for BlockRef<'a> {
    fn eq(&self, b: &Self) -> bool {
        self.0.ptr_eq(&b.0)
    }
}
impl<'a> Eq for BlockRef<'a> {}

#[derive(Clone, Debug)]
pub struct FunctionGraphEdge<'a> {
    pub src: BlockRef<'a>,
    pub dst: BlockRef<'a>,
}
fn collect_edges<'a>(blocks: &[Block<'a>]) -> Result<Vec<FunctionGraphEdge<'a>>> {
    use crate::spirv::Operand;

    const OP_LABEL: OpCode = 248;

    let mut out = Vec::new();
    for block in blocks.iter() {
        let src = block.clone().downgrade();
        for operand in block.branch_instr().operands() {
            if let Operand::Instruction(dst_label) = operand {
                let dst_label = dst_label.clone().upgrade().unwrap();
                if dst_label.opcode() == OP_LABEL {
                    let dst = blocks.iter()
                        .find(|x| x.label_instr() == &dst_label)
                        .ok_or(Error::UNEXPECTED_OP)?
                        .clone()
                        .downgrade();
                    let edge = FunctionGraphEdge { src: src.clone(), dst };
                    out.push(edge);
                }
            }
        }
    }
    Ok(out)
}

#[derive(Clone, Debug)]
pub struct FunctionGraphLoop<'a> {
    pub edges: Vec<FunctionGraphEdge<'a>>,
}
fn collect_loops_impl<'a>(
    edges: &[FunctionGraphEdge<'a>],
    block_stack: &mut Vec<BlockRef<'a>>,
    out: &mut Vec<FunctionGraphLoop<'a>>,
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
                // One of the ancester is the destination block of this edge.
                // The current edge is an back edge.
                let loop_edges = block_stack[i..].windows(2)
                    .map(|pair| FunctionGraphEdge {
                        src: pair[0].clone(),
                        dst: pair[1].clone(),
                    })
                    .chain([FunctionGraphEdge {
                        src: src_block.clone(),
                        dst: dst_block.clone(),
                    }])
                    .collect::<Vec<_>>();
                out.push(FunctionGraphLoop { edges: loop_edges });
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
fn collect_loops<'a>(edges: &[FunctionGraphEdge<'a>]) -> Vec<FunctionGraphLoop<'a>> {
    if edges.is_empty() { return Default::default(); }
    let mut block_stack = vec![edges[0].src.clone()];
    let mut out = Vec::new();
    collect_loops_impl(edges, &mut block_stack, &mut out);
    out
}

#[derive(Debug)]
pub struct FunctionGraph<'a> {
    blocks: Vec<Block<'a>>,
    edges: Vec<FunctionGraphEdge<'a>>,
    loops: Vec<FunctionGraphLoop<'a>>,
}
impl<'a> FunctionGraph<'a> {
    pub fn blocks(&self) -> &[Block<'a>] {
        &self.blocks
    }
    pub fn edges(&self) -> &[FunctionGraphEdge<'a>] {
        &self.edges
    }
    pub fn loops(&self) -> &[FunctionGraphLoop<'a>] {
        &self.loops
    }
}

#[derive(Debug)]
pub struct SpirvGraph<'a> {
    subgraphs: Vec<FunctionGraph<'a>>,
}
impl<'a> SpirvGraph<'a> {
    pub fn subgraphs(&self) -> &[FunctionGraph<'a>] {
        &self.subgraphs
    }
}

fn parse_grpah<'a>(spv: &'a Spirv) -> Result<SpirvGraph<'a>> {
    const OP_FUNCTION: OpCode = 54;
    const OP_FUNCTION_END: OpCode = 56;
    const OP_LABEL: OpCode = 248;

    let mut fns: Vec<FunctionGraph<'a>> = Vec::new();
    let mut cur_blocks: Option<Vec<Block<'a>>> = None;
    let mut cur_block_beg: Option<usize> = None;

    for (i, instr) in spv.instrs().iter().enumerate() {
        match instr.opcode() {
            OP_FUNCTION => {
                if cur_blocks.is_some() { return Err(Error::UNEXPECTED_OP); }
                cur_blocks = Some(Default::default());
            },
            OP_FUNCTION_END => {
                if let Some(mut blocks) = cur_blocks.take() {
                    if let Some(beg) = cur_block_beg.take() {
                        let inner = BlockInner { instrs: &spv.instrs()[beg..i] };
                        let block = Block(Rc::new(inner));
                        blocks.push(block);
                    }
                    let edges = collect_edges(&blocks)?;
                    let loops = collect_loops(&edges);
                    let f = FunctionGraph { blocks, edges, loops };
                    fns.push(f);
                } else {
                    return Err(Error::UNEXPECTED_OP);
                }
            },
            OP_LABEL => {
                if let Some(blocks) = cur_blocks.as_mut() {
                    if let Some(beg) = cur_block_beg.take() {
                        let inner = BlockInner { instrs: &spv.instrs()[beg..i] };
                        let block = Block(Rc::new(inner));
                        blocks.push(block);
                    }
                } else {
                    return Err(Error::UNEXPECTED_OP);
                }
                cur_block_beg = Some(i);
            },
            _ => {},
        }
    }
    let out = SpirvGraph { subgraphs: fns };
    Ok(out)
}

impl<'a> TryFrom<&'a Spirv> for SpirvGraph<'a> {
    type Error = Error;
    fn try_from(spv: &'a Spirv) -> Result<Self> {
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
        let graph = SpirvGraph::try_from(&spv).unwrap();
        // One entry point -> 1 function.
        assert_eq!(graph.subgraphs().len(), 1);
        // Selection + true + false + merge -> 4 blocks.
        assert_eq!(graph.subgraphs()[0].blocks().len(), 4);
    }
}
