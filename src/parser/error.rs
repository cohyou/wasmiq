 use crate::{
     FuncType,
 };
 use annot::Loc;
 use lexer::{Token, TokenKind};
 use super::*;
 
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    Lex(LexError),
    NotMatch(Token, TokenKind),
    Invalid(Token),
    NumCast(Token),
    CantResolveId(Token),
    InvalidTypeuseDef(Token, FuncType, FuncType),
    InvalidMessage(Token, String),
    Rewrite(RewriteError),  
    DuplicatedIds(Keyword, String),
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

use super::rewriter::RewriteError;
impl From<RewriteError> for ParseError {
    fn from(e: RewriteError) -> Self { ParseError::Rewrite(e) }
}