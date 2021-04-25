use std::fmt::{
    Debug,
    Display,
};

use super::super::annot::{Annot, Loc};
use super::keyword::*;

#[derive(PartialEq, Clone)]
pub enum Number {
    Integer(usize),
    FloatingPoint(f64),
}

impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match &self {
           Number::Integer(num) => write!(f, "{:?}", num),
           Number::FloatingPoint(num) => write!(f, "{:?}", num),        
       }        
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Number::Integer(num) => write!(f, "{}", num),
            Number::FloatingPoint(num) => write!(f, "{}", num),        
        }        
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Empty,

    Keyword(Keyword),
    Number(Number),
    String(String),
    Id(String), // $で始まる
    LeftParen,
    RightParen,
    Reserved(String),
    GenSym(u32),
}

pub type Token = Annot<TokenKind>;

impl Token {
    pub fn empty(loc: Loc) -> Self { Self::new(TokenKind::Empty, loc) }

    pub fn keyword(kw: Keyword, loc: Loc) -> Self { Self::new(TokenKind::Keyword(kw), loc) }
    pub fn number_u(num: usize, loc: Loc) -> Self { Self::new(TokenKind::Number(Number::Integer(num)), loc) }
    pub fn number_i(num: isize, loc: Loc) -> Self {
            let n = unsafe { std::mem::transmute::<isize, usize>(num) };
            Self::new(TokenKind::Number(Number::Integer(n)), loc)
        }
    pub fn number_f(num: f64, loc: Loc) -> Self { Self::new(TokenKind::Number(Number::FloatingPoint(num)), loc) }
    pub fn string(s: String, loc: Loc) -> Self { Self::new(TokenKind::String(s), loc) }
    pub fn id(n: String, loc: Loc) -> Self { Self::new(TokenKind::Id(n), loc) }
    pub fn left_paren(loc: Loc) -> Self { Self::new(TokenKind::LeftParen, loc) }
    pub fn right_paren(loc: Loc) -> Self { Self::new(TokenKind::RightParen, loc) }
    pub fn reserved(s: Vec<u8>, loc: Loc) -> Self { Self::new(TokenKind::Reserved(String::from_utf8(s).unwrap()), loc) }
    pub fn gensym(index: u32, loc: Loc) -> Self { Self::new(TokenKind::GenSym(index), loc) }
}

impl Debug for Token {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match &self.value {
           TokenKind::Keyword(kw) => write!(f, "{:?}<{:?}>", kw, self.loc),
           TokenKind::Number(num) => write!(f, "{:?}<{:?}>", num, self.loc),
           TokenKind::String(s) => write!(f, "{:?}<{:?}>", s, self.loc),
           TokenKind::Id(id) => write!(f, "${}<{:?}>", id, self.loc),
           TokenKind::Reserved(r) => write!(f, "Reserved({})<{:?}>", r, self.loc),
           _ => write!(f, "{:?}<{:?}>", self.value, self.loc)
       }        
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            TokenKind::Empty => write!(f, ""),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::Keyword(kw) => write!(f, "{}", kw),
            TokenKind::Number(num) => write!(f, "{}", num),
            TokenKind::String(s) => write!(f, "{:?}", s),
            TokenKind::Id(id) => write!(f, "${}", id),
            TokenKind::Reserved(r) => write!(f, "Reserved({})<{:?}>", r, self.loc),
            TokenKind::GenSym(idx) => write!(f, "<#:gensym({})>", idx),
            // _ => write!(f, "{:?}<{:?}>", self.value, self.loc)
        }        
     }
 }