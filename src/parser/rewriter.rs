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
        let first_lparen = self.lookahead.clone();
        self.match_lparen()?;
        if let kw!(Keyword::Module) = &self.lookahead {
            self.ast.push(first_lparen);
            self.ast.push(self.lookahead.clone());
            // self.rewrite_modulefields()?;
            self.make_ast()?;
            let lookahead = self.lookahead.clone();
            self.match_rparen()?;
            self.ast.push(lookahead);
            Ok(())
        } else {

            let token_lparen = Token::left_paren(Loc::zero());
            let token_module = Token::keyword(Keyword::Module, Loc::zero());
            self.ast = vec![token_lparen, token_module];
            // self.rewrite_modulefields()
            self.ast.push(first_lparen);
            self.ast.push(self.lookahead.clone());
            self.make_ast()?;
            self.ast.insert(self.ast.len()-2, Token::right_paren(Loc::zero()));
            Ok(())
        }
    }

    fn make_ast(&mut self) -> Result<(), RewriteError> {
        while let Ok(lookahead) = self.lexer.next_token() {
            self.ast.push(lookahead.clone());
            if lookahead.value == TokenKind::Empty {
                break;
            }
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
    // rewrite_tokens("(module (type $void (func)) (import \"wasi\" \"log\" (func (type 1))) (table $tab 5 anyfunc))");

    fn rewrite_tokens(src: &str) {
        let cursor = std::io::Cursor::new(src);
        let reader = std::io::BufReader::new(cursor);
        let mut rewriter = Rewriter::new(reader);
        
        let _ = rewriter.rewrite();
    
        println!("{:?}", rewriter.ast);
    }
    
    rewrite_tokens("(module (type $void (func)))");
    rewrite_tokens("(type $void (func))");
}

