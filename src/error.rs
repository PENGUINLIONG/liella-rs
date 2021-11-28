use std::error::Error;
use std::fmt;

#[derive(Debug)]
#[non_exhaustive]
pub enum LiellaError {
    CorruptedSpirv(&'static str),
    UnsupportedSpirv(&'static str),
}
use LiellaError::*;
impl LiellaError {
    pub const INCOMPLETE_HEADER: LiellaError = CorruptedSpirv("incomplete header");
    pub const INCOMPLETE_INSTR: LiellaError = CorruptedSpirv("incomplete instruction");
    pub const RESULT_ID_COLLISION: LiellaError = CorruptedSpirv("result id collision");
    pub const RESULT_ID_MISSING: LiellaError = CorruptedSpirv("result id missing");
    pub const UNEXPECTED_OP: LiellaError = CorruptedSpirv("unexpected op");
    pub const UNUSUAL_REFERENCE_COMPLEXITY: LiellaError = CorruptedSpirv("unusual reference complexity");

    pub const UNSUPPORTED_OP: LiellaError = UnsupportedSpirv("unsupported op");
}
impl Error for LiellaError {}
impl fmt::Display for LiellaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiellaError::CorruptedSpirv(msg) => f.write_str(msg),
            LiellaError::UnsupportedSpirv(msg) => f.write_str(msg),
        }
    }
}

pub type LiellaResult<T> = std::result::Result<T, LiellaError>;
