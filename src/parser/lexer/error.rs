use std::fmt::Debug;


use super::super::annot::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LexErrorKind {
    InvalidChar(u8),
    Io,
    Eof,
}

pub type LexError = Annot<LexErrorKind>;

impl LexError {
    pub fn invalid_char(c: u8, loc: Loc) -> Self {
        LexError::new(LexErrorKind::InvalidChar(c), loc)
    }
    pub fn io() -> Self { LexError::new(LexErrorKind::Io, Loc::default()) }
    pub fn eof(loc: Loc) -> Self { LexError::new(LexErrorKind::Eof, loc) }
}

use std::io::Error as StdError;
impl From<StdError> for LexError {
    fn from(_e: StdError) -> Self { LexError::io() }
}

use std::string::FromUtf8Error;
impl From<FromUtf8Error> for LexError {
    fn from(_e: FromUtf8Error) -> Self { LexError::io() }
}

// pub type Result<T> = std::result::Result<T, LexError>;

impl Debug for LexError {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}<{:?}>", self.value, self.loc)
    }
}
