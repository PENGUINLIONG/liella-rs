//! # SPIR-V Ser/de
use std::convert::{TryFrom};
use crate::error::{LiellaError as Error, LiellaResult as Result};

#[derive(Clone, Debug, Hash)]
pub struct SpirvHeader {
    pub magic: u32,
    pub version: u32,
    pub generator_magic: u32,
    pub bound: u32,
    pub reserved: u32,
}
impl TryFrom<&[u32]> for SpirvHeader {
    type Error = Error;
    fn try_from(words: &[u32]) -> Result<SpirvHeader> {
        let out = SpirvHeader {
            magic: *words.get(0).ok_or(Error::INCOMPLETE_HEADER)?,
            version: *words.get(1).ok_or(Error::INCOMPLETE_HEADER)?,
            generator_magic: *words.get(2).ok_or(Error::INCOMPLETE_HEADER)?,
            bound: *words.get(3).ok_or(Error::INCOMPLETE_HEADER)?,
            reserved: *words.get(4).ok_or(Error::INCOMPLETE_HEADER)?,
        };
        Ok(out)
    }
}



pub type OpCode = u32;
pub type SpvId = u32;



pub struct Instr<'a> {
    opcode: OpCode,
    operands: &'a [u32],
}
impl<'a> Instr<'a> {
    pub fn opcode(&self) -> OpCode {
        self.opcode
    }
    pub fn operands(&self) -> Operands<'a> {
        Operands {
            inner: self.operands.iter().peekable()
        }
    }
    pub fn len(&self) -> usize {
        self.operands.len() + 1
    }
}
pub struct Operands<'a> {
    inner: std::iter::Peekable<std::slice::Iter<'a, u32>>,
}
impl<'a> Iterator for Operands<'a> {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        self.inner.next().copied()
    }
}
impl<'a> Operands<'a> {
    pub fn u32(&mut self) -> u32 {
        self.next().unwrap()
    }
    pub fn id(&mut self) -> SpvId {
        self.next().unwrap() as SpvId
    }
    pub fn ate(&mut self) -> bool {
        self.inner.peek().is_none()
    }
}

pub struct Spv<'a> {
    header: SpirvHeader,
    instrs: Vec<Instr<'a>>,
}
impl<'a> Spv<'a> {
    pub fn header(&self) -> &SpirvHeader {
        &self.header
    }
    pub fn instrs(&self) -> &[Instr<'a>] {
        &self.instrs
    }
} 
impl<'a> TryFrom<&'a [u32]> for Spv<'a> {
    type Error = Error;
    fn try_from(words: &'a [u32]) -> Result<Spv<'a>> {
        let header = SpirvHeader::try_from(words)?;

        let mut instrs = Vec::new();
        let mut i: usize = 5;
        while i < words.len() {
            let instr_header = words[i];
            let instr_len = instr_header >> 16;
            let opcode = instr_header & 0xFFFF;

            let next_i = i + instr_len as usize;
            let operands: &'a [u32] = &words[(i + 1)..next_i];
            let instr = Instr { opcode, operands };
            instrs.push(instr);

            i = next_i;
        }

        let out = Spv { header, instrs };
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inline_spirv::inline_spirv;
    #[test]
    fn test_parse() {
        let spv: &'static [u32] =
            inline_spirv!("#version 450\nvoid main() {}", comp, vulkan1_0);
        let spv = Spv::try_from(spv).unwrap();
        assert_ne!(spv.instrs().len(), 0);

        let header = spv.header();
        assert_eq!(header.magic, spirv::MAGIC_NUMBER);
        assert_eq!(header.version, 0x00010000);
    }
}
