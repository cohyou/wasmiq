#[macro_use]mod util;
pub mod error;

mod typeuse_parser;
mod type_parser;
mod import_parser;
mod table_parser;
mod memory_parser;
mod global_parser;
mod func_parser;
mod export_parser;
mod elem_parser;
mod data_parser;
mod expr_parser;

mod lexer;
mod context;
mod annot;

mod rewriter;

use std::io::{Read, Seek};
use std::convert::TryFrom;

use annot::*;
use context::*;
use crate::instr::*;
use lexer::*;

use crate::{
    Module,
    Start,
    Name,
    Byte,
    ValType,
    Limits,
};

use rewriter::{
    Rewriter,
};

pub use self::error::*;
pub use self::typeuse_parser::*;
pub use self::table_parser::*;
pub use self::global_parser::*;
pub use self::expr_parser::*;

pub struct Parser<R>
where R: Read + Seek {
    rewriter: Rewriter<R>,  // lexer: Lexer<R>,
    lookahead: Token,
    pub contexts: Vec<Context>,
    pub module: Module,
}

impl<R> Parser<R> where R: Read + Seek {
    pub fn new(reader: R) -> Self {
        let mut rewriter = Rewriter::new(reader);
        let _ = rewriter.rewrite();
        Self {
            rewriter: rewriter,
            lookahead: Token::empty(Loc::default()),
            contexts: vec![Context::default()],
            module: Module::default(),
        }
    }

    pub fn parse(&mut self) -> Result<(), ParseError> {
        self.lookahead = self.rewriter.next_token()?;
        self.match_lparen()?;
        self.parse_module()
    }

    fn parse_module(&mut self) -> Result<(), ParseError> {

        self.match_keyword(Keyword::Module)?;

        if let tk!(TokenKind::Id(s)) = &self.lookahead {
            self.module.id = Some(s.clone());
            self.consume()?;
        }

        self.parse_module_internal()?;

        self.match_rparen()?;

        Ok(())
    }

    fn parse_module_internal(&mut self) -> Result<(), ParseError> {
        parse_field!(self, Type, self.parse_type()?);
        parse_field!(self, Import, self.parse_import()?);
        parse_field!(self, Func, self.parse_func()?);
        parse_field!(self, Table, self.parse_table()?);
        parse_field!(self, Memory, self.parse_memory()?);
        parse_field!(self, Global, self.parse_global()?);
        parse_field!(self, Export, self.parse_export()?);

        if !self.is_rparen()? {
            if let tk!(TokenKind::LeftParen) = self.lookahead {
                self.consume()?;
            }
            if let kw!(Keyword::Start) = self.lookahead {
                self.parse_start()?;
            }
        }
        parse_field!(self, Elem, self.parse_elem()?);
        parse_field!(self, Data, self.parse_data()?);    

        Ok(())
    }

    fn parse_start(&mut self) -> Result<(), ParseError> {
        self.match_keyword(Keyword::Start)?;

        // func id
        let funcidx = self.resolve_id(&self.contexts[0].funcs.clone())?;
        self.module.start = Some(Start(funcidx));

        self.match_rparen()?;

        Ok(())
    }

    fn match_keyword(&mut self, matching: Keyword) -> Result<(), ParseError> {
        match &self.lookahead {
            kw!(kw) => {
                if kw == &matching {
                    self.consume()?;
                    Ok(())
                } else {
                    Err(self.err())
                }
            },
            _ => Err(self.err()),
        }
    }

    fn is_lparen(&mut self) -> Result<bool, ParseError> {
        if let tk!(TokenKind::LeftParen) = self.lookahead { Ok(true) } else { Ok(false) }
    }

    fn is_rparen(&mut self) -> Result<bool, ParseError> {
        if let tk!(TokenKind::RightParen) = self.lookahead { Ok(true) } else { Ok(false) }
    }

    fn match_lparen(&mut self) -> Result<(), ParseError> {
        self.match_token(TokenKind::LeftParen)
    }

    fn match_rparen(&mut self) -> Result<(), ParseError> {
        self.match_token(TokenKind::RightParen)
    }

    fn match_token(&mut self, t: TokenKind) -> Result<(), ParseError> {
        if self.lookahead.value == t {
            self.consume()
        } else {
            Err(ParseError::NotMatch(self.lookahead.clone(), t))
        }
    }

    fn parse_name(&mut self) -> Result<Name, ParseError> {
        self.parse_string()
    }

    fn parse_data_string(&mut self) -> Result<Vec<Byte>, ParseError> {
        // self.parse_string();
        unimplemented!();
    }

    fn parse_string(&mut self) -> Result<String, ParseError> {
        if let tk!(TokenKind::String(s)) = &self.lookahead {
            let res = Ok(s.clone());
            self.consume()?;
            res
        } else {
            Err(ParseError::NotMatch(self.lookahead.clone(), TokenKind::String("".into())))
        }
    }

    fn parse_valtype(&mut self) -> Result<ValType, ParseError> {
        if let kw!(Keyword::ValType(vt)) = &self.lookahead {
            let res = vt.clone();
            self.consume()?;
            Ok(res)
        } else {
            Err(self.err())
        }
    }

    fn parse_num<T: TryFrom<usize>>(&mut self) -> Result<T, ParseError> {
        if let nm!(Number::Integer(n)) = &self.lookahead {
            if let Ok(num) = T::try_from(n.clone()) {
                self.consume()?;
                Ok(num)
            } else {
                Err(ParseError::NumCast(self.lookahead.clone()))
            }
        } else {
            Err(self.err())
        }
    }

    fn parse_limits(&mut self) -> Result<Limits, ParseError> {
        let mut limits = Limits::default();

        // min
        limits.min = self.parse_num::<u32>()?;

        // max(optional)
        if let nm!(Number::Integer(_)) = &self.lookahead {
            limits.max = Some(self.parse_num::<u32>()?);
        }

        Ok(limits)
    }

    fn parse_offset(&mut self) -> Result<Expr, ParseError> {
        self.match_keyword(Keyword::Offset)?;

        // expr
        let expr = self.parse_expr()?;

        self.match_rparen()?;

        Ok(expr)
    }

    fn resolve_id(&mut self, from: &Vec<Option<Id>>) -> Result<u32, ParseError> {
        match &self.lookahead {
            nm!(Number::Integer(n)) => {
                let res = u32::try_from(n.clone())?;
                self.consume()?;
                Ok(res)
            },
            tk!(TokenKind::Id(id)) => {

                if let Some(idx) = from.iter()
                // .inspect(|c| println!("before: {:?}", c))
                .position(|t|
                    if let Some(typeidx) = t {
                        typeidx == id
                    } else {
                        false
                    }
                ) {
                    self.consume()?;
                    Ok(u32::try_from(idx)?)
                } else {
                    Err(ParseError::CantResolveId(self.lookahead.clone()))
                }
            }
            _ => Err(self.err()),
        }
    }

    fn peek(&mut self) -> Result<Token, ParseError> {
        let peeked = self.rewriter.peek_token()?;
        Ok(peeked)
    }

    fn consume(&mut self) -> Result<(), ParseError> {
        self.lookahead = self.rewriter.next_token()?;
        pp!("consume", self.lookahead);
        Ok(())
    }

    fn err(&self) -> ParseError {
        ParseError::Invalid(self.lookahead.clone())
    }

    fn err2(&self, mes: &'static str) -> ParseError {
        ParseError::InvalidMessage(self.lookahead.clone(), mes.to_string())
    }
}

#[test]
#[ignore]
fn test() {
    let s = r#"
    (func $const (type 0) i32.const 42)
    (type (func (param i32)))
    "#;
    parse_str(s);
}

#[allow(dead_code)]
fn parse_str(s: &str) {
    use std::io::{Cursor, BufReader};
    let cursor = Cursor::new(s);
    let reader = BufReader::new(cursor);
    let mut parser = Parser::new(reader);
    match parser.parse() {
        Ok(_) => println!("module: {:?}", parser.module),
        Err(err) => println!("parse error: {:?}", err),
    }
}