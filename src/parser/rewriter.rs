mod func;
mod table;
mod mem;
mod global;
mod dataelem;
mod type_;
mod import;

use std::io::{Read, Seek};
use std::collections::VecDeque;
use crate::parser::lexer::{
    Lexer,
    LexError,
    Token,
    TokenKind,
    Keyword,
    Number,
};
use crate::parser::{
    Annot,
    Loc,
};
use crate::{
    Instr,
};

#[derive(Debug, PartialEq)]
pub enum RewriteError {
    Invalid(String),
    OnLex(LexError),
    Break,
    EOF,
}

impl From<LexError> for RewriteError {
    fn from(lex_error: LexError) -> RewriteError {
        RewriteError::OnLex(lex_error)
    }
}

pub struct Rewriter<R>
where R: Read + Seek {
    lexer: Lexer<R>,
    lookahead: Token,

    types: Vec<Token>,
    imports: Vec<Token>,
    funcs: Vec<Token>,
    tables: Vec<Token>,
    mems: Vec<Token>,
    globals: Vec<Token>,
    start: Vec<Token>,
    exports: Vec<Token>,
    elem: Vec<Token>,
    data: Vec<Token>,

    pub ast: Vec<Token>,
    current: usize,
    next_symbol_index: u32,

    precedings: VecDeque<Token>,
}

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn new(reader: R) -> Self {
        let mut lexer = Lexer::new(reader);
        if let Ok(lookahead) = lexer.next_token() {
            Rewriter {
                lexer: lexer,
                lookahead: lookahead,

                types: Vec::default(),
                imports: Vec::default(),
                funcs: Vec::default(),
                tables: Vec::default(),
                mems: Vec::default(),
                globals: Vec::default(),
                start: Vec::default(),
                exports: Vec::default(),
                elem: Vec::default(),
                data: Vec::default(),

                ast: Vec::default(),
                current: 0,
                next_symbol_index: 0,

                precedings: VecDeque::default(),
            }
        } else {
            unimplemented!()
        }
    }
    
    pub fn rewrite(&mut self) -> Result<(), RewriteError> {
        if self.lookahead.value == TokenKind::Empty {
            self.ast.push(Token::left_paren(Loc::zero()));
            self.ast.push(Token::keyword(Keyword::Module, Loc::zero()));
            self.ast.push(Token::right_paren(Loc::zero()));
            self.ast.push(self.lookahead.clone());
            Ok(())
        } else {
            self.rewrite_module()
        }
    }

    fn rewrite_module(&mut self) -> Result<(), RewriteError> {
        let first_lparen = self.lookahead.clone();
        self.match_lparen()?;
        if let kw!(Keyword::Module) = &self.lookahead {
            self.ast.push(first_lparen);
            self.ast.push(self.lookahead.clone());
            self.rewrite_module_internal()?;

            let lookahead = self.lookahead.clone();
            self.match_rparen()?;
            self.ast.push(lookahead);
            Ok(())
        } else {
            let token_lparen = Token::left_paren(Loc::zero());
            let token_module = Token::keyword(Keyword::Module, Loc::zero());
            self.ast = vec![token_lparen, token_module];
            self.rewrite_segment(first_lparen, self.lookahead.clone())?;
            self.rewrite_module_internal()?;  // p!(self.ast);
            self.ast.insert(self.ast.len()-2, Token::right_paren(Loc::zero()));
            
            Ok(())
        }
    }

    fn rewrite_module_internal(&mut self) -> Result<(), RewriteError> {
        let mut last: Option<Token> = None;

        loop {
            let lookahead = 
            if let Some(token) = self.precedings.pop_front() {
                token
            } else {
                if let Ok(lookahead) = self.lexer.next_token() {
                    lookahead
                } else {
                    break;
                }
            };

            match lookahead.value {
                TokenKind::LeftParen => {
                    match self.rewrite_list(lookahead) {
                        Err(RewriteError::Break) => break,
                        _ => {},
                    }
                },
                TokenKind::Empty => {
                    last = Some(lookahead);
                    break;
                },
                TokenKind::RightParen => {
                    last = Some(lookahead);
                    break;
                }
                _ => {
                    self.ast.push(lookahead);
                },
            }
        }
        
        let debugging = true;
        if debugging {
            pp!("types", tokens_to_string(&self.types));
            pp!("imports", tokens_to_string(&self.imports));
            pp!("funcs", tokens_to_string(&self.funcs));
            pp!("tables", tokens_to_string(&self.tables));
            pp!("mems", tokens_to_string(&self.mems));
            pp!("globals", tokens_to_string(&self.globals));
            pp!("exports", tokens_to_string(&self.exports));
            pp!("elem", tokens_to_string(&self.elem));
            pp!("data", tokens_to_string(&self.data));
            pp!("start", tokens_to_string(&self.start));
        }        

        self.ast.extend(self.types.clone());
        self.ast.extend(self.imports.clone());  
        self.ast.extend(self.tables.clone());
        self.ast.extend(self.mems.clone());
        self.ast.extend(self.globals.clone());
        self.ast.extend(self.funcs.clone());
        self.ast.extend(self.exports.clone());
        self.ast.extend(self.elem.clone());
        self.ast.extend(self.data.clone());
        self.ast.extend(self.start.clone());

        if let Some(last) = last {
            self.ast.push(last);
        }

        self.ast.push(Token::empty(Loc::zero()));

        Ok(())
    }

    fn rewrite_list(&mut self, lparen_segment: Token) -> Result<(), RewriteError> {
        match self.lexer.next_token() {
            Err(_) => Err(RewriteError::Break),
            Ok(tk!(TokenKind::Empty)) => {
                self.ast.push(lparen_segment);
                Err(RewriteError::Break)
            },

            Ok(kw!(Keyword::Param)) => {
                self.ast.push(lparen_segment);
                let holding = self.rewrite_valtypes(Keyword::Param)?;
                self.ast.extend(holding);
                Ok(())
            },
            Ok(kw!(Keyword::Result)) => {
                self.ast.push(lparen_segment);
                let holding = self.rewrite_valtypes(Keyword::Result)?;
                self.ast.extend(holding);
                Ok(())
            },
            Ok(kw!(Keyword::Local)) => {
                self.ast.push(lparen_segment);
                let holding = self.rewrite_valtypes(Keyword::Local)?;
                self.ast.extend(holding);
                Ok(())
            },
 
            Ok(lookahead) => self.rewrite_segment(lparen_segment, lookahead),
        }
    }

    fn rewrite_segment(&mut self, lparen_segment: Token, segment: Token) -> Result<(), RewriteError> {
        match segment {
            kw!(Keyword::Import) => self.rewrite_import(lparen_segment, segment),
            kw!(Keyword::Type) => self.rewrite_type(lparen_segment, segment),
            kw!(Keyword::Func) => self.rewrite_func(lparen_segment, segment),
            kw!(Keyword::Table) => self.rewrite_table(lparen_segment, segment),
            kw!(Keyword::Memory) => self.rewrite_memory(lparen_segment, segment),
            kw!(Keyword::Global) => self.rewrite_global(lparen_segment, segment),
            kw!(Keyword::Export) => self.scan_export(lparen_segment, segment),
            kw!(Keyword::Elem) => self.rewrite_elem(lparen_segment, segment),
            kw!(Keyword::Data) => self.rewrite_data(lparen_segment, segment),
            kw!(Keyword::Start) => self.scan_start(lparen_segment, segment),
            _ => {
                self.ast.push(lparen_segment);
                self.ast.push(segment);
                Ok(())
            },
        }
    }

    fn scan_start(&mut self, lparen_start: Token, start: Token) -> Result<(), RewriteError> {
        self.start.push(lparen_start);
        self.start.push(start);
        let funcidx = self.lexer.next_token()?;
        self.start.push(funcidx);
        let rparen_start = self.lexer.next_token()?;
        self.start.push(rparen_start);
        Ok(())
    }

    fn make_type_gensym_tokens(&mut self) -> Vec<Token> {
        vec![
            Token::left_paren(Loc::zero()),
            Token::keyword(Keyword::Type, Loc::zero()),
            self.make_gensym(),
            Token::right_paren(Loc::zero()),
        ]
    }

    fn scan_export(&mut self, lparen_segment: Token, segment: Token) -> Result<(), RewriteError> {
        let exports = self.scan_imexport(lparen_segment, segment)?;
        self.exports.extend(exports);
        Ok(())
    }

    fn scan_imexport(&mut self, lparen_segment: Token, segment: Token) -> Result<Vec<Token>, RewriteError> {
        let mut tokens = vec![];
        tokens.push(lparen_segment);
        tokens.push(segment);
        while let Ok(token) = self.lexer.next_token() {
            tokens.push(token.clone());
            match token {
                tk!(TokenKind::LeftParen) => {
                    let importdesc = self.scan_simple_list()?;
                    tokens.extend(importdesc);
                },
                tk!(TokenKind::RightParen) => break,
                _ => {},
            }
        }
        Ok(tokens)
    }

    fn scan_simple_list(&mut self) -> Result<Vec<Token>, RewriteError> {
        let mut tokens = vec![];
        while let Ok(token) = self.lexer.next_token() {
            tokens.push(token.clone());
            match token {
                tk!(TokenKind::RightParen) => break,
                tk!(TokenKind::LeftParen) => {
                    let child = self.scan_simple_list()?;
                    tokens.extend(child);
                },
                _ => {},
            }
        }
        Ok(tokens)
    }
}


impl<R> Rewriter<R> where R: Read + Seek {
    fn scan_id(&mut self, token_maybe_id: Token, tokens: &mut Vec<Token>) -> Result<Token, RewriteError> {
        if let token_id @ tk!(TokenKind::Id(_)) = token_maybe_id {
            tokens.push(token_id.clone());
            let next = self.lexer.next_token()?;
            Ok(next)
        } else {
            Ok(token_maybe_id)
        }
    }

    fn scan_label(&mut self) -> Result<(Vec<Token>, Token), RewriteError> {
        let token = self.lexer.next_token()?;
        self.scan_label_internal(token)
    }

    fn scan_label_internal(&mut self, token: Token) -> Result<(Vec<Token>, Token), RewriteError> {
        let mut holding = vec![];
        let token = match token {
            id_ @ tk!(TokenKind::Id(_)) => {
                holding.push(id_);
                self.lexer.next_token()?
            },
            t @ _ => t,
        };
        Ok((holding, token))
    }
}

impl<R> Rewriter<R> where R: Read + Seek {
    fn scan_typeidx_holding(&mut self, token1: Token, token2: Token) -> Result<Vec<Token>, RewriteError> {
        let mut holding = vec![token1, token2];
        let typeidx = self.lexer.next_token()?;
        holding.push(typeidx);
        let rparen = self.lexer.next_token()?;
        holding.push(rparen);
        Ok(holding)
    }

    fn add_typeidx(&mut self) -> Vec<Token> {
        vec![
            Token::left_paren(Loc::zero()),
            Token::keyword(Keyword::Type, Loc::zero()),
            self.make_gensym(),
            Token::right_paren(Loc::zero()),
        ]
    }

    fn rewrite_valtypes(&mut self, keyword: Keyword) -> Result<Vec<Token>, RewriteError> {
        let mut holding = vec![];
        let mut valtypes = vec![];
        let mut right_paren: Option<Token> = None;
        while let Ok(token) = self.lexer.next_token() {
            match token {
                id @ tk!(TokenKind::Id(_)) => {
                    holding.push(id);
                    let valtype = self.lexer.next_token()?;
                    holding.push(valtype);
                    let rparen = self.lexer.next_token()?;
                    holding.push(rparen);
                    return Ok(holding);
                },
                lookahead @ tk!(TokenKind::RightParen) => { 
                    right_paren = Some(lookahead);
                    break;
                },
                lookahead @ kw!(Keyword::ValType(_)) => {
                    valtypes.push(lookahead)
                },
                lookahead @ _ => {
                    for valtype in &valtypes {
                        holding.push(valtype.clone());
                    }
                    holding.push(lookahead);
                    break;
                },
            }
        }

        for (i, valtype) in valtypes.iter().enumerate() {
            if i == 0 {
                holding.push(valtype.clone());
            } else {
                holding.push(Token::right_paren(Loc::zero()));
                holding.push(Token::left_paren(Loc::zero()));
                holding.push(Token::keyword(keyword.clone(), Loc::zero()));
                holding.push(valtype.clone());
            }
        }

        if let Some(right_paren) = right_paren {
            holding.push(right_paren);
        }

        Ok(holding)
    }
}

impl<R> Rewriter<R> where R: Read + Seek {
    fn make_gensym(&mut self) -> Token {
        let index = self.next_symbol_index;
        self.next_symbol_index += 1;
        Token::gensym(index, Loc::zero())
    }
}

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn next_token(&mut self) -> Result<Token, RewriteError> {
        if let Some(token) = self.ast.get(self.current) {
            self.current += 1;
            Ok(token.clone())
        } else {
            Err(RewriteError::EOF)
        }
    }

    pub fn peek_token(&mut self) -> Result<Token, RewriteError> {
        if let Some(token) = self.ast.get(self.current) {
            Ok(token.clone())
        } else {
            Err(RewriteError::Invalid("peek_token".to_owned()))
        }
    }

    fn match_lparen(&mut self) -> Result<(), RewriteError> {
        self.match_token(TokenKind::LeftParen)
    }

    fn match_rparen(&mut self) -> Result<(), RewriteError> {
        self.match_token(TokenKind::RightParen)
    }

    fn match_token(&mut self, t: TokenKind) -> Result<(), RewriteError> {
        if self.lookahead.value == t {
            self.consume()
        } else {
            Err(RewriteError::Invalid("match_token".to_owned()))
        }
    }

    fn consume(&mut self) -> Result<(), RewriteError> {
        self.lookahead = self.lexer.next_token()?;
        Ok(())
    }
}

#[allow(dead_code)]
fn print_tokens(src: &str) {
    let cursor = std::io::Cursor::new(src);
    let reader = std::io::BufReader::new(cursor);
    let mut lexer = Lexer::new(reader);
    while let Ok(token) = lexer.next_token() {
        println!("{:?}", token);
        if let tk!(TokenKind::Empty) = token {
            break;
        }
    }
}

#[allow(dead_code)]
fn rewrite_tokens(src: &str) -> Vec<Token> {
    let cursor = std::io::Cursor::new(src);
    let reader = std::io::BufReader::new(cursor);
    let mut rewriter = Rewriter::new(reader);
    
    let _ = rewriter.rewrite();

    rewriter.ast
}

#[allow(dead_code)]
pub fn tokens_to_string(tokens: &Vec<Token>) -> String {
    let mut result = String::new();
    let mut prev = Token::empty(Loc::zero());
    let mut nospace = false;
    // print!("[");
    for token in tokens {
        // println!("token: {:?}", token);
        match token {
            tk!(TokenKind::Empty) => {
                result.push_str(format!("{}", prev).as_ref());
            },
            tk!(TokenKind::LeftParen) => {
                result.push_str(format!("{}", prev).as_ref());
                match prev {
                    tk!(TokenKind::Empty) => {},
                    tk!(TokenKind::LeftParen) => {},
                    tk!(TokenKind::RightParen) => {
                        result.push_str(" ");
                    },
                    _ => {
                        if !nospace {
                            result.push_str(" ")  // print!("_");
                        }
                    },
                }
                nospace = true;
            },
            tk!(TokenKind::RightParen) => {
                result.push_str(format!("{}", prev).as_ref());
            },
            _ => {
                result.push_str(format!("{}", prev).as_ref());
                if let tk!(TokenKind::LeftParen) = prev {
                } else {
                    if !nospace { 
                        result.push_str(" ");  // print!("~");
                    }
                }

                nospace = false;
            },
        };
        prev = token.clone();
    }
    // print!("]");

    result.push_str(format!("{}", prev).as_ref());

    result
}

#[allow(dead_code)]
fn assert_eq_rewrite(before: &str, after:&str) {
    assert_eq!(tokens_to_string(&rewrite_tokens(before)), after.to_string());
}

#[test]
fn test_export() {
    assert_eq_rewrite(r#"(export "n1" (func 0))"#, r#"(module (export "n1" (func 0)))"#);
    assert_eq_rewrite(
        r#"(export "n1" (func 1)) (export "n2" (table 0))"#, 
        r#"(module (export "n1" (func 1)) (export "n2" (table 0)))"#
    );
}