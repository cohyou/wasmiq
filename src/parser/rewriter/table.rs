use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_table(&mut self, lparen_global: Token, global: Token) -> Result<(), RewriteError> {
        let mut header = vec![lparen_global, global];
        let maybe_id = self.lexer.next_token()?;
        let token1 = self.scan_id(maybe_id, &mut header)?;
        let token2 = self.lexer.next_token()?;
        self.rewrite_table_internal(header, token1, token2, false)
    }

    fn rewrite_table_internal(&mut self, header: Vec<Token>, token1: Token, token2: Token, exporting: bool) -> Result<(), RewriteError> {
        match token1 {
            lparen @ tk!(TokenKind::LeftParen) => {
                match token2 {
                    import @ kw!(Keyword::Import) => {
                        self.ast.push(lparen);
                        self.ast.push(import);
                        let name1 = self.lexer.next_token()?;
                        self.ast.push(name1);
                        let name2 = self.lexer.next_token()?;
                        self.ast.push(name2);

                        for t in header.clone() { self.ast.push(t); }
                        if exporting && header.len() == 2 {
                            self.ast.push(Token::gensym(Loc::zero()))
                        }

                        let rparen = self.lexer.next_token()?;

                        match self.lexer.next_token()? {
                            num1 @ tk!(TokenKind::Number(Number::Integer(_))) => {
                                self.ast.push(num1);
                                match self.lexer.next_token()? {
                                    num2 @ tk!(TokenKind::Number(Number::Integer(_))) => { 
                                        self.ast.push(num2);
                                        let rparen = self.lexer.next_token()?;
                                        self.ast.push(rparen);
                                    },
                                    rparen_memory @ _ => {
                                        // let rparen = self.lexer.next_token()?;
                                        self.ast.push(rparen_memory);
                                    },
                                }      
                            },
                            token @ _ => self.ast.push(token),
                        }
                        self.ast.push(rparen);
                    },
                    export @ kw!(Keyword::Export) => {
                        self.ast.push(lparen);
                        self.ast.push(export);
                        let name = self.lexer.next_token()?;
                        self.ast.push(name);

                        for t in header.clone() { self.ast.push(t); }
                        if header.len() == 2 {
                            self.ast.push(Token::gensym(Loc::zero()))
                        }

                        self.ast.push(Token::right_paren(Loc::zero()));
                        let rparen_global = self.lexer.next_token()?;
                        self.ast.push(rparen_global);

                        let token1 = self.lexer.next_token()?;
                        let token2 = self.lexer.next_token()?;
                        return self.rewrite_table_internal(header, token1, token2, true);
                    },
                    _ => {
                        for t in header { self.ast.push(t); }
                        self.ast.push(lparen);
                        self.ast.push(token2);
                    },
                }
            },
            num1 @ tk!(TokenKind::Number(Number::Integer(_))) => {
                for t in header.clone() { self.ast.push(t); }
                if exporting && header.len() == 2 {
                    self.ast.push(Token::gensym(Loc::zero()))
                }

                self.ast.push(num1);
                self.ast.push(token2);
            },
            data @ kw!(Keyword::FuncRef) => {
                for t in header.clone() { self.ast.push(t); }
                if header.len() == 2 {
                    self.ast.push(Token::gensym(Loc::zero()))
                }
                return self.rewrite_table_elem(&header, data, token2);
            },
            _ => {
                for t in header { self.ast.push(t); }
                self.ast.push(token1);
                self.ast.push(token2);
            },
        }

        Ok(())
    }
    
    fn rewrite_table_elem(&mut self, tokens: &Vec<Token>, token1: Token, token2: Token) -> Result<(), RewriteError> {
        let mut tokens_elem = vec![];
        tokens_elem.push(token1);
        tokens_elem.push(Token::right_paren(Loc::zero()));

        tokens_elem.push(token2);                  
        let token_elem = self.lexer.next_token()?;
        tokens_elem.push(token_elem); 
        
        match tokens.len() {
            2 => tokens_elem.push(Token::gensym(Loc::zero())),
            3 => tokens_elem.push(tokens[2].clone()),
            _ => {},
        }

        tokens_elem.push(Token::left_paren(Loc::zero()));
        tokens_elem.push(Token::keyword(Keyword::Instr(Instr::I32Const(0)), Loc::zero()));
        tokens_elem.push(Token::number_u(0, Loc::zero()));
        tokens_elem.push(Token::right_paren(Loc::zero()));

        let mut n = 0;
        while let Ok(token) = self.lexer.next_token() {
            match token {
                tk!(TokenKind::RightParen) => {
                    tokens_elem.push(token);
                    break;
                },
                tk!(TokenKind::Empty) => {
                    tokens_elem.push(token);
                    break;
                },
                _ => {                                
                    tokens_elem.push(token);
                    n += 1;
                }
            }
        }

        self.ast.push(Token::number_u(n, Loc::zero()));
        self.ast.push(Token::number_u(n, Loc::zero()));
        for t in tokens_elem {
            self.ast.push(t);
        }

        Ok(())
    }
}

#[test]
fn test_rewrite_table_normal() {
    assert_eq_rewrite("(table 1 funcref)", "(module (table 1 funcref))");
    assert_eq_rewrite("(table 10 100 funcref)", "(module (table 10 100 funcref))");
    assert_eq_rewrite("(table $id1 1000 funcref)", "(module (table $id1 1000 funcref))");
    assert_eq_rewrite("(table $id2 10000 20000 funcref)", "(module (table $id2 10000 20000 funcref))");
}

#[test]
fn test_rewrite_table_import() {
    assert_eq_rewrite(
        r#"(table (import "n1" "n2") 2 funcref)"#, 
        r#"(module (import "n1" "n2" (table 2 funcref)))"#
    );
    assert_eq_rewrite(
        r#"(table (import "name3" "name4") 20 40 funcref)"#, 
        r#"(module (import "name3" "name4" (table 20 40 funcref)))"#
    );
    assert_eq_rewrite(
        r#"(table $imp_table1 (import "name1" "name2") 2 funcref)"#, 
        r#"(module (import "name1" "name2" (table $imp_table1 2 funcref)))"#
    );
    assert_eq_rewrite(
        r#"(table $imp_table2 (import "name3" "name4") 20 40 funcref)"#, 
        r#"(module (import "name3" "name4" (table $imp_table2 20 40 funcref)))"#
    );
}

#[test]
fn test_rewrite_table_export() {
    assert_eq_rewrite(
        r#"(table (export "expname1") 100 200 funcref)"#, 
        r#"(module (export "expname1" (table <#:gensym>)) (table <#:gensym> 100 200 funcref))"#
    );
    assert_eq_rewrite(
        r#"(table $expid1 (export "expname2") 15 funcref)"#, 
        r#"(module (export "expname2" (table $expid1)) (table $expid1 15 funcref))"#
    );
    assert_eq_rewrite(
        r#"(table (export "e3") (export "e4") 32786 funcref)"#, 
        r#"(module (export "e3" (table <#:gensym>)) (export "e4" (table <#:gensym>)) (table <#:gensym> 32786 funcref))"#
    );
    assert_eq_rewrite(
        r#"(table $id (export "e5") (export "e6") 0 0 funcref)"#, 
        r#"(module (export "e5" (table $id)) (export "e6" (table $id)) (table $id 0 0 funcref))"#
    );
}

#[test]
fn test_rewrite_table_import_export() {
    assert_eq_rewrite(
        r#"(table $id (export "e3") (import "n1" "n2") 1234 funcref)"#, 
        r#"(module (export "e3" (table $id)) (import "n1" "n2" (table $id 1234 funcref)))"#
    );
    assert_eq_rewrite(
        r#"(table (export "e3") (import "n1" "n2") 4321 5678 funcref)"#, 
        r#"(module (export "e3" (table <#:gensym>)) (import "n1" "n2" (table <#:gensym> 4321 5678 funcref)))"#
    );
}

#[test]
fn test_rewrite_table_elem() {
    assert_eq_rewrite(
        r#"(module (table funcref (elem 0 1 2 3 100)))"#, 
        r#"(module (table <#:gensym> 5 5 funcref) (elem <#:gensym> (i32.const 0) 0 1 2 3 100)))"#
    );
    assert_eq_rewrite(
        r#"(module (table $id funcref (elem 0 1 2 3 100)))"#, 
        r#"(module (table $id 5 5 funcref) (elem $id (i32.const 0) 0 1 2 3 100)))"#
    );
    assert_eq_rewrite(
        r#"(module (table (export "n1") funcref (elem 1 2 4 8 16 32 64 128 256 512)))"#, 
        r#"(module (export "n1" (table <#:gensym>)) (table <#:gensym> 10 10 funcref) (elem <#:gensym> (i32.const 0) 1 2 4 8 16 32 64 128 256 512)))"#
    );
    assert_eq_rewrite(
        r#"(module (table $id (export "n1") funcref (elem 1 2 4 8 16 32 64 128 256 512)))"#, 
        r#"(module (export "n1" (table $id)) (table $id 10 10 funcref) (elem $id (i32.const 0) 1 2 4 8 16 32 64 128 256 512)))"#
    );
}