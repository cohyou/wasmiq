mod func;
mod func_if;
mod func_folded;
mod table;
mod mem;
mod global;
mod dataelem;


use std::io::{Read, Seek};
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
#[derive(Debug)]
pub enum RewriteError {
    Invalid,
    Break,
}

impl From<LexError> for RewriteError {
    fn from(_: LexError) -> RewriteError {
        RewriteError::Invalid
    }
}

pub struct Rewriter<R>
where R: Read + Seek {
    lexer: Lexer<R>,
    lookahead: Token,
    pub ast: Vec<Token>,
    current: usize,
}

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn new(reader: R) -> Self {
        let mut lexer = Lexer::new(reader);
        if let Ok(lookahead) = lexer.next_token() {
            Rewriter {
                lexer: lexer,
                lookahead: lookahead,
                ast: Vec::default(),
                current: 0,
            }
        } else {
            unimplemented!()
        }
    }
    
    pub fn rewrite(&mut self) -> Result<(), RewriteError> {
        self.rewrite_module()
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
            self.rewrite_module_internal()?;
            self.ast.insert(self.ast.len()-2, Token::right_paren(Loc::zero()));
            Ok(())
        }
    }

    fn rewrite_module_internal(&mut self) -> Result<(), RewriteError> {
        while let Ok(lookahead) = self.lexer.next_token() {
            println!("lookahead: {:?}", lookahead);
            match lookahead.value {
                TokenKind::LeftParen => {
                    match self.rewrite_list(lookahead) {
                        Err(RewriteError::Break) => break,
                        _ => {},
                    }
                },
                TokenKind::Empty => {
                    self.ast.push(lookahead);
                    break;
                },
                _ => {
                    self.ast.push(lookahead);
                },
            }
        }

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
                self.rewrite_param()
            },
            Ok(kw!(Keyword::Result)) => {
                self.ast.push(lparen_segment);
                self.rewrite_result()
            },
            Ok(kw!(Keyword::Local)) => {
                self.ast.push(lparen_segment);
                self.rewrite_local()
            },
 
            Ok(lookahead) => self.rewrite_segment(lparen_segment, lookahead),
        }
    }

    fn rewrite_segment(&mut self, lparen_segment: Token, segment: Token) -> Result<(), RewriteError> {
        match segment {
            kw!(Keyword::Func) => self.rewrite_func(lparen_segment, segment),
            kw!(Keyword::Table) => self.rewrite_table(lparen_segment, segment),
            kw!(Keyword::Memory) => self.rewrite_memory(lparen_segment, segment),
            kw!(Keyword::Global) => self.rewrite_global(lparen_segment, segment),
            kw!(Keyword::Elem) => {
                self.ast.push(lparen_segment);
                self.ast.push(segment);
                self.rewrite_elem()
            },
            kw!(Keyword::Data) => {
                self.ast.push(lparen_segment);
                self.ast.push(segment);
                self.rewrite_data()
            },
            _ => {
                self.ast.push(lparen_segment);
                self.ast.push(segment);
                Ok(())
            },
        }
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

    fn is_import_or_export(token1: &Token, token2: &Token) -> bool {
        token1.value == TokenKind::LeftParen &&
        (token2.value == TokenKind::Keyword(Keyword::Import) ||
         token2.value == TokenKind::Keyword(Keyword::Export) )
    }

    fn rewrite_inline_export_import(&mut self, tokens: Vec<Token>, token_leftparen: Token, token2: Token)
     -> Result<(), RewriteError> {
        let mut token1 = token_leftparen.clone();
        let mut token2 = token2.clone();
        loop {
            match token1 {
                tk!(TokenKind::LeftParen) => {
                    match token2 {
                        token_import @ kw!(Keyword::Import) => {
                            self.ast.push(token_import.clone());
                            let token_name1 = self.lexer.next_token()?;
                            self.ast.push(token_name1.clone());
                            let token_name2 = self.lexer.next_token()?;
                            self.ast.push(token_name2.clone());
                    
                            self.ast.push(token_leftparen);
                            for t in &tokens { self.ast.push(t.clone()); }
        
                            break;
                        }, 
                        token_export @ kw!(Keyword::Export) => {
                            self.ast.push(token_export.clone());
                            let token_name1 = self.lexer.next_token()?;
                            self.ast.push(token_name1.clone());
                            self.ast.push(Token::left_paren(Loc::zero()));
                            if tokens.len() == 1 {
                                for t in &tokens { self.ast.push(t.clone()); }
                                self.ast.push(Token::gensym(Loc::zero()))
                            } else {
                                for t in &tokens { self.ast.push(t.clone()); }
                            }
                            let token_rightparen_keyword = self.lexer.next_token()?;
                            self.ast.push(token_rightparen_keyword);
        
                            token1 = self.lexer.next_token()?;
                            token2 = self.lexer.next_token()?;
                            if Rewriter::<R>::is_import_or_export(&token1, &token2) {
                                self.ast.push(Token::right_paren(Loc::zero()));
                                self.ast.push(token1.clone());
                            } else {
                                self.ast.push(Token::right_paren(Loc::zero()));
                                self.ast.push(Token::left_paren(Loc::zero()));
                                if tokens.len() == 1 {
                                    for t in &tokens { self.ast.push(t.clone()); }
                                    self.ast.push(Token::gensym(Loc::zero()))
                                } else {
                                    for t in &tokens { self.ast.push(t.clone()); }
                                }
                                self.ast.push(token1.clone());
                                self.ast.push(token2.clone());
                                break;
                            }
                        },
                        _ => {
                            break;
                        },
                    }
                },
                _ => {
                    break;
                },
            }

            token2 = token1.clone();
            token1 = self.lexer.next_token()?;
        }

        Ok(())
    }
}

impl<R> Rewriter<R> where R: Read + Seek {

    fn rewrite_param(&mut self) -> Result<(), RewriteError> {
        self.rewrite_valtypes(Keyword::Param)
    }

    fn rewrite_result(&mut self) -> Result<(), RewriteError> {
        self.rewrite_valtypes(Keyword::Result)
    }

    fn rewrite_local(&mut self) -> Result<(), RewriteError> {
        self.rewrite_valtypes(Keyword::Local)
    }

    fn rewrite_valtypes(&mut self, keyword: Keyword) -> Result<(), RewriteError> {
        let mut valtypes = vec![];
        let mut right_paren: Option<Token> = None;
        while let Ok(token) = self.lexer.next_token() {
            println!("rewrite_valtypes: {:?}", token);
            match token {
                token_id @ tk!(TokenKind::Id(_)) => {
                    self.ast.push(token_id);
                    let token_valtype = self.lexer.next_token()?;
                    self.ast.push(token_valtype);
                    let token_rightparen = self.lexer.next_token()?;
                    self.ast.push(token_rightparen);
                    return Ok(());
                },
                lookahead @ tk!(TokenKind::RightParen) => { 
                    right_paren = Some(lookahead.clone());
                    break;
                },
                lookahead @ kw!(Keyword::ValType(_)) => {
                    valtypes.push(lookahead)
                },
                lookahead @ _ => {
                    for valtype in &valtypes {
                        self.ast.push(valtype.clone());
                    }
                    self.ast.push(lookahead);
                    break;
                },
            }
        }

        for (i, valtype) in valtypes.iter().enumerate() {
            if i == 0 {
                self.ast.push(valtype.clone());
            } else {
                self.ast.push(Token::right_paren(Loc::zero()));
                self.ast.push(Token::left_paren(Loc::zero()));
                self.ast.push(Token::keyword(keyword.clone(), Loc::zero()));
                self.ast.push(valtype.clone());
            }
        }

        if let Some(right_paren) = right_paren {
            self.ast.push(right_paren.clone());
        }

        Ok(())
    }
}

#[test]
fn test_rewrite_valtypes() {
    assert_eq_rewrite("(param f32)", "(module (param f32))");
    assert_eq_rewrite("(param i32 i64)", "(module (param i32) (param i64))");
    assert_eq_rewrite("(result f64)", "(module (result f64))");
    assert_eq_rewrite("(result i64 f32)", "(module (result i64) (result f32))");
    assert_eq_rewrite("(local f64)", "(module (local f64))");
    assert_eq_rewrite("(module (local i64 f32))", "(module (local i64) (local f32))");
}

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn next_token(&mut self) -> Result<Token, RewriteError> {
        if let Some(token) = self.ast.get(self.current) {
            self.current += 1;
            Ok(token.clone())
        } else {
            Err(RewriteError::Invalid)
        }
    }

    pub fn peek_token(&mut self) -> Result<Token, RewriteError> {
        if let Some(token) = self.ast.get(self.current) {
            Ok(token.clone())
        } else {
            Err(RewriteError::Invalid)
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
            Err(RewriteError::Invalid)
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

    // println!("{:?}", rewriter.ast);
    rewriter.ast
}

#[allow(dead_code)]
fn tokens_to_string(tokens: Vec<Token>) -> String {
    let mut result = String::new();
    let mut prev = Token::empty(Loc::zero());
    let mut nospace = false;
    // print!("[");
    for token in tokens {
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
        prev = token;
    }
    // print!("]");
    result
}

#[allow(dead_code)]
fn assert_eq_rewrite(before: &str, after:&str) {
    assert_eq!(tokens_to_string(rewrite_tokens(before)), after.to_string());
}
