 use crate::{
     FuncType,
 };
 use annot::Loc;
 use lexer::{Token, TokenKind};
 use super::*;
 
#[derive(Debug)]
pub enum ParseError {
    Lex(LexError),
    NotMatch(Token, TokenKind),
    Invalid(Token),
    NumCast(Token),
    CantResolveId(Token),
    InvalidTypeuseDef(Token, FuncType, FuncType),
    InvalidMessage(Token, String),
    // LastItem,    
}

use lexer::LexError;
impl From<LexError> for ParseError {
    fn from(e: LexError) -> Self { ParseError::Lex(e) }
}

use std::num::TryFromIntError;
impl From<TryFromIntError> for ParseError {
    fn from(_e: TryFromIntError) -> Self {
        ParseError::NumCast(Token::empty(Loc::default()))
    }
}
