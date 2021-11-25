use std::error::Error;
use std::fmt;

#[derive(Debug)]
#[non_exhaustive]
pub enum LiellaError {
    CorruptedSpirv(&'static str),
}
use LiellaError::*;
impl LiellaError {
    pub const INCOMPLETE_HEADER: LiellaError = CorruptedSpirv("incomplete header");
}
impl Error for LiellaError {}
impl fmt::Display for LiellaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiellaError::CorruptedSpirv(msg) => f.write_str(msg),
        }
    }
}

pub type LiellaResult<T> = std::result::Result<T, LiellaError>;
