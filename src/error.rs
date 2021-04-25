use crate::{
    parser::error::ParseError,
};

#[derive(Debug, PartialEq)]
pub enum Error {
    Invalid(String),
    OutOfIndex(String),
    OutOfRange(String),
    Mutability(String),
    PreCondition(String),
    OnParse(ParseError),
}
