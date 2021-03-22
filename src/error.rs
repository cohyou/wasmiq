#[derive(Debug)]
pub enum Error {
    Invalid,
    OutOfIndex(String),
    OutOfRange(String),
    Mutability(String),
    PreCondition(String),
}
