use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_memory(&mut self, lparen_global: Token, global: Token) -> Result<(), RewriteError> {
        let mut header = vec![lparen_global, global];
        let maybe_id = self.lexer.next_token()?;
        let token1 = self.scan_id(maybe_id, &mut header)?;
        let token2 = self.lexer.next_token()?;
        self.rewrite_memory_internal(header, token1, token2, false)
    }

    fn rewrite_memory_internal(&mut self, header: Vec<Token>, token1: Token, token2: Token, exporting: bool) -> Result<(), RewriteError> {
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
                        return self.rewrite_memory_internal(header, token1, token2, true);
                    },
                    data @ kw!(Keyword::Data) => {
                        for t in header.clone() { self.ast.push(t); }
                        if header.len() == 2 {
                            self.ast.push(Token::gensym(Loc::zero()))
                        }
                        return self.rewrite_memory_data(&header, lparen, data);
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
            _ => {
                for t in header { self.ast.push(t); }
                self.ast.push(token1);
                self.ast.push(token2);
            },
        }

        Ok(())
    }

    fn rewrite_memory_data(&mut self, tokens: &Vec<Token>, token1: Token, token2: Token) -> Result<(), RewriteError> {
        let mut tokens_data = vec![];

        tokens_data.push(Token::right_paren(Loc::zero()));

        tokens_data.push(token1);  
        tokens_data.push(token2);                  
        
        match tokens.len() {
            2 => tokens_data.push(Token::gensym(Loc::zero())),
            3 => tokens_data.push(tokens[2].clone()),
            _ => {},
        }

        tokens_data.push(Token::left_paren(Loc::zero()));
        tokens_data.push(Token::keyword(Keyword::Instr(Instr::I32Const(0)), Loc::zero()));
        tokens_data.push(Token::number_u(0, Loc::zero()));
        tokens_data.push(Token::right_paren(Loc::zero()));

        let mut n = 0;
        while let Ok(token) = self.lexer.next_token() {
            match token {
                tk!(TokenKind::RightParen) => {
                    tokens_data.push(token);
                    break;
                },
                tk!(TokenKind::Empty) => {
                    tokens_data.push(token);
                    break;
                },
                tk!(TokenKind::String(ref s)) => {
                    tokens_data.push(token.clone());
                    n += s.len();
                }
                _ => {                                
                    tokens_data.push(token);
                    break;
                }
            }
        }

        let n = (n / (64*1024)) + 1;

        self.ast.push(Token::number_u(n, Loc::zero()));
        self.ast.push(Token::number_u(n, Loc::zero()));
        for t in tokens_data {
            self.ast.push(t);
        }

        Ok(())
    }
}

#[test]
fn test_rewrite_mem_normal() {
    assert_eq_rewrite("(memory 1)", "(module (memory 1))");
    assert_eq_rewrite("(memory 10 100)", "(module (memory 10 100))");
    assert_eq_rewrite("(memory $id1 1000)", "(module (memory $id1 1000))");
    assert_eq_rewrite("(memory $id2 10000 20000)", "(module (memory $id2 10000 20000))");
}

#[test]
fn test_rewrite_mem_import() {
    assert_eq_rewrite(
        r#"(memory (import "name1" "name2") 2)"#, 
        r#"(module (import "name1" "name2" (memory 2)))"#
    );
    assert_eq_rewrite(
        r#"(memory (import "name3" "name4") 20 40)"#, 
        r#"(module (import "name3" "name4" (memory 20 40)))"#
    );
    assert_eq_rewrite(
        r#"(memory $imp_mem1 (import "name1" "name2") 2)"#, 
        r#"(module (import "name1" "name2" (memory $imp_mem1 2)))"#
    );
    assert_eq_rewrite(
        r#"(memory $imp_mem2 (import "name3" "name4") 20 40)"#, 
        r#"(module (import "name3" "name4" (memory $imp_mem2 20 40)))"#
    );
}

#[test]
fn test_rewrite_mem_export() {
    assert_eq_rewrite(
        r#"(memory (export "e1") 5)"#, 
        r#"(module (export "e1" (memory <#:gensym>)) (memory <#:gensym> 5))"#
    );
    assert_eq_rewrite(
        r#"(memory $id (export "e2") 50 100)"#, 
        r#"(module (export "e2" (memory $id)) (memory $id 50 100))"#
    );
    assert_eq_rewrite(
        r#"(memory (export "e3") (export "e4") 512)"#, 
        r#"(module (export "e3" (memory <#:gensym>)) (export "e4" (memory <#:gensym>)) (memory <#:gensym> 512))"#
    );
    assert_eq_rewrite(
        r#"(memory $id2 (export "e5") (export "e6") 10000 20000)"#, 
        r#"(module (export "e5" (memory $id2)) (export "e6" (memory $id2)) (memory $id2 10000 20000))"#
    );
}

#[test]
fn test_rewrite_mem_import_export() {
    assert_eq_rewrite(
        r#"(memory (export "e3") (import "n1" "n2") 1234)"#, 
        r#"(module (export "e3" (memory <#:gensym>)) (import "n1" "n2" (memory <#:gensym> 1234)))"#
    );
    assert_eq_rewrite(
        r#"(memory $id (export "e3") (import "n1" "n2") 4321 5678)"#, 
        r#"(module (export "e3" (memory $id)) (import "n1" "n2" (memory $id 4321 5678)))"#
    );
}

#[test]
fn test_rewrite_table_data() {
    assert_eq_rewrite(
        r#"(module (memory (data "abcd" "wowow" "wasmiq")))"#, 
        r#"(module (memory <#:gensym> 1 1) (data <#:gensym> (i32.const 0) "abcd" "wowow" "wasmiq")))"#
    );
    assert_eq_rewrite(
        r#"(module (memory $id (data "abcd" "wowow" "wasmiq")))"#, 
        r#"(module (memory $id 1 1) (data $id (i32.const 0) "abcd" "wowow" "wasmiq")))"#
    );
    assert_eq_rewrite(
        r#"(module (memory (export "n1") (data "abcd" "wowow" "wasmiq")))"#, 
        r#"(module (export "n1" (memory <#:gensym>)) (memory <#:gensym> 1 1) (data <#:gensym> (i32.const 0) "abcd" "wowow" "wasmiq")))"#
    );
    assert_eq_rewrite(
        r#"(module (memory $id (export "n1") (data "abcd" "wowow" "wasmiq")))"#, 
        r#"(module (export "n1" (memory $id)) (memory $id 1 1) (data $id (i32.const 0) "abcd" "wowow" "wasmiq")))"#
    );
}