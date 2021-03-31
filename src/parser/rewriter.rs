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
};

pub enum RewriteError {
    Invalid,
}

impl From<LexError> for RewriteError {
    fn from(_: LexError) -> RewriteError {
        RewriteError::Invalid
    }
}

enum AST {
    Tree(Vec<AST>),
    Leaf(Token),
}

impl AST {

}
pub struct Rewriter<R>
where R: Read + Seek {
    lexer: Lexer<R>,
    lookahead: Token,
    ast: AST,
    // pub contexts: Vec<Context>,
}

macro_rules! find_segment {
    ($this:ident, $kw:ident, $func:ident) => {
        loop {
            if let kw!(Keyword::$kw) = &$this.lookahead {
                let segment = $this.$func()?;
                return Ok(segment);
            }
        }
    };
}

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite(&mut self, reader: R) -> Result<(), RewriteError> {
        self.lexer = Lexer::new(reader);
    
        self.lookahead = self.lexer.next_token()?;
        self.match_lparen()?;
        self.rewrite_module()
    }
    
    fn rewrite_module(&mut self) -> Result<(), RewriteError> {
        let leaf = AST::Leaf(Token::keyword(Keyword::Module, self.lookahead.loc));
        self.ast = AST::Tree(vec![leaf]);
        if let kw!(Keyword::Module) = &self.lookahead {
            self.rewrite_modulefields()?;
            self.match_rparen()
        } else {
            self.rewrite_modulefields()
        }
    }

    fn rewrite_modulefields(&mut self) -> Result<(), RewriteError> {
        let mut segments = vec![];

        segments.push(self.find_type_segment()?);
        segments.push(self.find_import_segment()?);
        segments.push(self.find_table_segment()?);
        segments.push(self.find_memory_segment()?);
        segments.push(self.find_global_segment()?);
        segments.push(self.find_export_segment()?);
        segments.push(self.find_start_segment()?);
        segments.push(self.find_elem_segment()?);
        segments.push(self.find_data_segment()?);

        Ok(())
    }

    fn append_child(&mut self, child: AST) -> Result<(), RewriteError> { 
        let tree = if let AST::Tree(tree) = &self.ast {
            tree
        } else {
            return Err(RewriteError::Invalid);
        };
        tree.push(child);
        Ok(())
    }

    // fn extend_children(&mut self, child: Vec<AST>) -> Result<(), RewriteError> { 
    //     if let AST::Tree(tree) = &self.ast {
    //         tree.extend(child);
    //         Ok(())
    //     } else {
    //         Err(RewriteError::Invalid)
    //     }        
    // }

    fn find_type_segment(&mut self) -> Result<AST, RewriteError> {
        find_segment!(self, Type, rewrite_type)
    }
    fn find_import_segment(&mut self) -> Result<AST, RewriteError> {
        find_segment!(self, Import, rewrite_import)
    }
    fn find_table_segment(&mut self) -> Result<AST, RewriteError> {
        find_segment!(self, Table, rewrite_table)
    }
    fn find_memory_segment(&mut self) -> Result<AST, RewriteError> {
        find_segment!(self, Memory, rewrite_memory)
    }
    fn find_global_segment(&mut self) -> Result<AST, RewriteError> {
        find_segment!(self, Global, rewrite_global)
    }
    fn find_export_segment(&mut self) -> Result<AST, RewriteError> {
        find_segment!(self, Export, rewrite_export)
    }
    fn find_start_segment(&mut self) -> Result<AST, RewriteError> {
        find_segment!(self, Start, rewrite_start)
    }
    fn find_elem_segment(&mut self) -> Result<AST, RewriteError> {
        find_segment!(self, Elem, rewrite_start)
    }
    fn find_data_segment(&mut self) -> Result<AST, RewriteError> {
        find_segment!(self, Data, rewrite_start)
    }

    fn rewrite_type(&mut self) -> Result<AST, RewriteError> {
        unimplemented!()
    }
    fn rewrite_import(&mut self) -> Result<AST, RewriteError> {
        unimplemented!()
    }
    fn rewrite_table(&mut self) -> Result<AST, RewriteError> {
        unimplemented!()
    }
    fn rewrite_memory(&mut self) -> Result<AST, RewriteError> {
        unimplemented!()
    }
    fn rewrite_global(&mut self) -> Result<AST, RewriteError> {
        unimplemented!()
    }
    fn rewrite_export(&mut self) -> Result<AST, RewriteError> {
        unimplemented!()
    }
    fn rewrite_start(&mut self) -> Result<AST, RewriteError> {
        unimplemented!()
    }
    fn rewrite_elem(&mut self) -> Result<AST, RewriteError> {
        unimplemented!()
    }
    fn rewrite_data(&mut self) -> Result<AST, RewriteError> {
        unimplemented!()
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
