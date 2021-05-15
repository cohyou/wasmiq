use crate::{
    parser::error::ParseError,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    Invalid(String),
    OutOfIndex(String),
    OutOfRange(String),
    Mutability(String),
    PreCondition(String),
    OnParse(ParseError),
    LackOfArgs(String),
    InvalidTypeOfArgs(String),
    InvalidTypeOfResult(String),
}
