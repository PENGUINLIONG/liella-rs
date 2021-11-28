use core::convert::TryFrom;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use crate::spirv::{Instruction, OpCode, Spirv};
use crate::error::{LiellaError as Error, LiellaResult as Result};

pub struct BlockInner<'a> {
    instrs: &'a [Instruction],
}
pub struct Block<'a>(Rc<BlockInner<'a>>);
pub struct BlockRef<'a>(Weak<BlockInner<'a>>);

pub struct FunctionGraph<'a> {
    blocks: Vec<Block<'a>>,
}
impl<'a> FunctionGraph<'a> {
    pub fn blocks(&self) -> &[Block<'a>] {
        &self.blocks
    }
}

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
    let mut cur_fn: Option<FunctionGraph<'a>> = None;
    let mut cur_block_beg: Option<usize> = None;

    for (i, instr) in spv.instrs().iter().enumerate() {
        match instr.opcode() {
            OP_FUNCTION => {
                if cur_fn.is_some() { return Err(Error::UNEXPECTED_OP); }
                let f = FunctionGraph { blocks: Default::default() };
                cur_fn = Some(f);
            },
            OP_FUNCTION_END => {
                if let Some(mut f) = cur_fn.take() {
                    if let Some(beg) = cur_block_beg.take() {
                        let inner = BlockInner { instrs: &spv.instrs()[beg..i] };
                        let block = Block(Rc::new(inner));
                        f.blocks.push(block);
                    }
                    fns.push(f);
                } else {
                    return Err(Error::UNEXPECTED_OP);
                }
            },
            OP_LABEL => {
                if let Some(f) = cur_fn.as_mut() {
                    if let Some(beg) = cur_block_beg.take() {
                        let inner = BlockInner { instrs: &spv.instrs()[beg..i] };
                        let block = Block(Rc::new(inner));
                        f.blocks.push(block);
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
