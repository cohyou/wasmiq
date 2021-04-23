use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_table(&mut self, lparen_global: Token, global: Token) -> Result<(), RewriteError> {
        let mut header = vec![lparen_global, global];
        let maybe_id = self.lexer.next_token()?;
        let token1 = self.scan_id(maybe_id, &mut header)?;
        let token2 = self.lexer.next_token()?;
        self.rewrite_table_internal(header, token1, token2, false)?;

        Ok(())
    }

    fn rewrite_table_internal(&mut self, header: Vec<Token>, token1: Token, token2: Token, exporting: bool) -> Result<(), RewriteError> {
        match token1 {
            lparen @ tk!(TokenKind::LeftParen) => {
                match token2 {
                    import @ kw!(Keyword::Import) => {
                        self.imports.push(lparen);
                        self.imports.push(import);
                        let name1 = self.lexer.next_token()?;
                        self.imports.push(name1);
                        let name2 = self.lexer.next_token()?;
                        self.imports.push(name2);

                        for t in header.clone() { self.imports.push(t); }
                        if exporting && header.len() == 2 {
                            self.imports.push(Token::gensym(Loc::zero()))
                        }

                        let rparen = self.lexer.next_token()?;

                        match self.lexer.next_token()? {
                            num1 @ tk!(TokenKind::Number(Number::Integer(_))) => {
                                self.imports.push(num1);
                                match self.lexer.next_token()? {
                                    num2 @ tk!(TokenKind::Number(Number::Integer(_))) => { 
                                        self.imports.push(num2);
                                        let funcref = self.lexer.next_token()?;
                                        self.imports.push(funcref);
                                    },
                                    funcref @ _ => {
                                        self.imports.push(funcref);
                                    },
                                }      
                            },
                            token @ _ => {
                                self.imports.push(token);

                            },
                        }
                        let rparen_table = self.lexer.next_token()?;
                        self.imports.push(rparen_table);
                        self.imports.push(rparen);
                    },
                    export @ kw!(Keyword::Export) => {
                        self.exports.push(lparen);
                        self.exports.push(export);
                        let name = self.lexer.next_token()?;
                        self.exports.push(name);

                        for t in header.clone() { self.exports.push(t); }
                        if header.len() == 2 {
                            self.exports.push(Token::gensym(Loc::zero()))
                        }

                        self.exports.push(Token::right_paren(Loc::zero()));
                        let rparen_table = self.lexer.next_token()?;
                        self.exports.push(rparen_table);

                        let token1 = self.lexer.next_token()?;
                        let token2 = self.lexer.next_token()?;
                        return self.rewrite_table_internal(header, token1, token2, true);
                    },
                    _ => {
                        for t in header { self.tables.push(t); }
                        self.tables.push(lparen);
                        self.tables.push(token2);
                    },
                }
            },
            num1 @ tk!(TokenKind::Number(Number::Integer(_))) => {
                for t in header.clone() { self.tables.push(t); }
                if exporting && header.len() == 2 {
                    self.tables.push(Token::gensym(Loc::zero()))
                }

                self.tables.push(num1);

                match token2 {
                    num2 @ tk!(TokenKind::Number(Number::Integer(_))) => {
                        self.tables.push(num2);
                        let funcref = self.lexer.next_token()?;
                        self.tables.push(funcref);
                    },
                    _ => self.tables.push(token2),
                }
                let rparen_table = self.lexer.next_token()?;
                self.tables.push(rparen_table);
            },
            data @ kw!(Keyword::FuncRef) => {
                for t in header.clone() { self.tables.push(t); }
                if header.len() == 2 {
                    self.tables.push(Token::gensym(Loc::zero()))
                }
                return self.rewrite_table_elem(&header, data, token2);
            },
            _ => {
                for t in header { self.tables.push(t); }
                self.tables.push(token1);
                self.tables.push(token2);
            },
        }

        Ok(())
    }
    
    fn rewrite_table_elem(&mut self, tokens: &Vec<Token>, token1: Token, token2: Token) -> Result<(), RewriteError> {
        let mut tokens_elem = vec![];
        tokens_elem.push(token1);

        self.elem.push(token2);                  
        let token_elem = self.lexer.next_token()?;
        self.elem.push(token_elem); 
        
        match tokens.len() {
            2 => self.elem.push(Token::gensym(Loc::zero())),
            3 => self.elem.push(tokens[2].clone()),
            _ => {},
        }

        self.elem.push(Token::left_paren(Loc::zero()));
        self.elem.push(Token::keyword(Keyword::Instr(Instr::I32Const(0)), Loc::zero()));
        self.elem.push(Token::number_u(0, Loc::zero()));
        self.elem.push(Token::right_paren(Loc::zero()));

        let mut n = 0;
        while let Ok(token) = self.lexer.next_token() {
            match token {
                tk!(TokenKind::RightParen) => {
                    self.elem.push(token);
                    break;
                },
                tk!(TokenKind::Empty) => {
                    self.elem.push(token);
                    break;
                },
                _ => {                                
                    self.elem.push(token);
                    n += 1;
                }
            }
        }
        let rparen_table = self.lexer.next_token()?;
        tokens_elem.push(rparen_table);

        self.tables.push(Token::number_u(n, Loc::zero()));
        self.tables.push(Token::number_u(n, Loc::zero()));
        
        for t in tokens_elem { self.tables.push(t); }

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
        r#"(module (table <#:gensym> 100 200 funcref) (export "expname1" (table <#:gensym>)))"#
    );
    assert_eq_rewrite(
        r#"(table $expid1 (export "expname2") 15 funcref)"#, 
        r#"(module (table $expid1 15 funcref) (export "expname2" (table $expid1)))"#
    );
    assert_eq_rewrite(
        r#"(table (export "e3") (export "e4") 32786 funcref)"#, 
        r#"(module (table <#:gensym> 32786 funcref) (export "e3" (table <#:gensym>)) (export "e4" (table <#:gensym>)))"#
    );
    assert_eq_rewrite(
        r#"(table $id (export "e5") (export "e6") 0 0 funcref)"#, 
        r#"(module (table $id 0 0 funcref) (export "e5" (table $id)) (export "e6" (table $id)))"#
    );
}

#[test]
fn test_rewrite_table_import_export() {
    assert_eq_rewrite(
        r#"(table $id (export "e3") (import "n1" "n2") 1234 funcref)"#, 
        r#"(module (import "n1" "n2" (table $id 1234 funcref)) (export "e3" (table $id)))"#
    );
    assert_eq_rewrite(
        r#"(table (export "e3") (import "n1" "n2") 4321 5678 funcref)"#, 
        r#"(module (import "n1" "n2" (table <#:gensym> 4321 5678 funcref)) (export "e3" (table <#:gensym>)))"#
    );
}

#[test]
fn test_rewrite_table_elem() {
    assert_eq_rewrite(
        r#"(module (table funcref (elem 0 1 2 3 100)))"#, 
        r#"(module (table <#:gensym> 5 5 funcref) (elem <#:gensym> (i32.const 0) 0 1 2 3 100))"#
    );
    assert_eq_rewrite(
        r#"(module (table $id funcref (elem 0 1 2 3 100)))"#, 
        r#"(module (table $id 5 5 funcref) (elem $id (i32.const 0) 0 1 2 3 100))"#
    );
    assert_eq_rewrite(
        r#"(module (table (export "n1") funcref (elem 1 2 4 8 16 32 64 128 256 512)))"#, 
        r#"(module (table <#:gensym> 10 10 funcref) (export "n1" (table <#:gensym>)) (elem <#:gensym> (i32.const 0) 1 2 4 8 16 32 64 128 256 512))"#
    );
    assert_eq_rewrite(
        r#"(module (table $id (export "n1") funcref (elem 1 2 4 8 16 32 64 128 256 512)))"#, 
        r#"(module (table $id 10 10 funcref) (export "n1" (table $id)) (elem $id (i32.const 0) 1 2 4 8 16 32 64 128 256 512))"#
    );
}