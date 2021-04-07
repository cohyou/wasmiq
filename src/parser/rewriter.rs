use std::io::{Read, Seek};
use crate::parser::lexer::{
    Lexer,
    LexError,
    Token,
    TokenKind,
    Keyword,
};
use crate::parser::{
    Annot,
    Loc,
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

// macro_rules! find_segment {
//     ($this:ident, $kw:ident, $func:ident) => {
//         loop {
//             if let kw!(Keyword::$kw) = &$this.lookahead {
//                 let segment = $this.$func()?;
//                 return Ok(segment);
//             }
//         }
//     };
// }

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
            self.ast.push(first_lparen);
            // self.ast.push(self.lookahead.clone());
            self.rewrite_list_internal(self.lookahead.clone())?;
            self.rewrite_module_internal()?;
            self.ast.insert(self.ast.len()-2, Token::right_paren(Loc::zero()));
            Ok(())
        }
    }

    fn rewrite_module_internal(&mut self) -> Result<(), RewriteError> {
        while let Ok(lookahead) = self.lexer.next_token() {
            match lookahead.value {
                TokenKind::LeftParen => {
                    self.ast.push(lookahead.clone());
                    match self.rewrite_list() {
                        Err(RewriteError::Break) => break,
                        _ => {},
                    }
                },
                _ => {
                    self.ast.push(lookahead.clone());
                },
            }
            
            if lookahead.value == TokenKind::Empty {
                break;
            }
        }

        Ok(())
    }

    fn rewrite_list(&mut self) -> Result<(), RewriteError> {
        match self.lexer.next_token() {
            Err(_) => Err(RewriteError::Break),
            Ok(la) => {
                self.rewrite_list_internal(la)
            },
        }
    }

    fn rewrite_list_internal(&mut self, token: Token) -> Result<(), RewriteError> {
        match token {
            lookahead @ tk!(TokenKind::Empty) => {
                self.ast.push(lookahead.clone());
                Err(RewriteError::Break)
            },
            lookahead @ kw!(Keyword::Param) => {
                self.ast.push(lookahead.clone());
                self.rewrite_param()
            },
            lookahead @ kw!(Keyword::Result) => {
                self.ast.push(lookahead.clone());
                self.rewrite_param()
            },
            _ => {
                Ok(())
            },
        }
    }

    fn rewrite_param(&mut self) -> Result<(), RewriteError> {
        self.rewrite_valtypes(Keyword::Param)
    }

    fn rewrite_result(&mut self) -> Result<(), RewriteError> {
        self.rewrite_valtypes(Keyword::Result)
    }

    fn rewrite_valtypes(&mut self, keyword: Keyword) -> Result<(), RewriteError> {
        let mut valtypes = vec![];
        let mut right_paren: Option<Token> = None;
        while let Ok(token) = self.lexer.next_token() {
            match token {
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

    // fn rewrite_modulefields(&mut self) -> Result<(), RewriteError> {
    //     let mut segments = vec![];

    //     segments.push(self.find_type_segment()?);
    //     segments.push(self.find_import_segment()?);
    //     segments.push(self.find_table_segment()?);
    //     segments.push(self.find_memory_segment()?);
    //     segments.push(self.find_global_segment()?);
    //     segments.push(self.find_export_segment()?);
    //     segments.push(self.find_start_segment()?);
    //     segments.push(self.find_elem_segment()?);
    //     segments.push(self.find_data_segment()?);

    //     Ok(())
    // }

    // fn find_type_segment(&mut self) -> Result<AST, RewriteError> {
    //     find_segment!(self, Type, rewrite_type)
    // }
    // fn find_import_segment(&mut self) -> Result<AST, RewriteError> {
    //     find_segment!(self, Import, rewrite_import)
    // }
    // fn find_table_segment(&mut self) -> Result<AST, RewriteError> {
    //     find_segment!(self, Table, rewrite_table)
    // }
    // fn find_memory_segment(&mut self) -> Result<AST, RewriteError> {
    //     find_segment!(self, Memory, rewrite_memory)
    // }
    // fn find_global_segment(&mut self) -> Result<AST, RewriteError> {
    //     find_segment!(self, Global, rewrite_global)
    // }
    // fn find_export_segment(&mut self) -> Result<AST, RewriteError> {
    //     find_segment!(self, Export, rewrite_export)
    // }
    // fn find_start_segment(&mut self) -> Result<AST, RewriteError> {
    //     find_segment!(self, Start, rewrite_start)
    // }
    // fn find_elem_segment(&mut self) -> Result<AST, RewriteError> {
    //     find_segment!(self, Elem, rewrite_start)
    // }
    // fn find_data_segment(&mut self) -> Result<AST, RewriteError> {
    //     find_segment!(self, Data, rewrite_start)
    // }

    // fn rewrite_type(&mut self) -> Result<AST, RewriteError> {
    //     unimplemented!()
    // }
    // fn rewrite_import(&mut self) -> Result<AST, RewriteError> {
    //     unimplemented!()
    // }
    // fn rewrite_table(&mut self) -> Result<AST, RewriteError> {
    //     unimplemented!()
    // }
    // fn rewrite_memory(&mut self) -> Result<AST, RewriteError> {
    //     unimplemented!()
    // }
    // fn rewrite_global(&mut self) -> Result<AST, RewriteError> {
    //     unimplemented!()
    // }
    // fn rewrite_export(&mut self) -> Result<AST, RewriteError> {
    //     unimplemented!()
    // }
    // fn rewrite_start(&mut self) -> Result<AST, RewriteError> {
    //     unimplemented!()
    // }
    // fn rewrite_elem(&mut self) -> Result<AST, RewriteError> {
    //     unimplemented!()
    // }
    // fn rewrite_data(&mut self) -> Result<AST, RewriteError> {
    //     unimplemented!()
    // }

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

#[test]
fn test_rewriter3() {
    fn rewrite_tokens(src: &str) -> Vec<Token> {
        let cursor = std::io::Cursor::new(src);
        let reader = std::io::BufReader::new(cursor);
        let mut rewriter = Rewriter::new(reader);
        
        let _ = rewriter.rewrite();
    
        // println!("{:?}", rewriter.ast);
        rewriter.ast
    }

    // rewrite_tokens("(module (type $void (func)))");
    // rewrite_tokens("(type $void (func))");

    // assert_eq!(rewrite_tokens("(param i32)"), vec![]);
    assert_eq!(rewrite_tokens("(param f32 i32 i64)"), vec![]);
}

#[test]
fn test_param() {
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
    // print_tokens("(param i32)");
    print_tokens("(param F32 i32 i64)");
}
