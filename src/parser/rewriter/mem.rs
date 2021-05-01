use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_memory(&mut self, lparen_global: Token, global: Token) -> Result<(), RewriteError> {
        let mut header = vec![lparen_global, global];
        let maybe_id = self.lexer.next_token()?;
        if let tk!(TokenKind::Id(s)) = maybe_id.clone() {
            self.context.mems.push(Some(Id::Named(s)));
        }
        let token1 = self.scan_id(maybe_id, &mut header)?;
        let token2 = self.lexer.next_token()?;

        self.set_context_id_func_memory(token1.clone(), token1.clone());

        self.rewrite_memory_internal(header, token1, token2, false)
    }

    fn set_context_id_func_memory(&mut self, token1: Token, token2: Token) {
        if let tk!(TokenKind::LeftParen) = token1 {
            if let kw!(Keyword::Export) = token2 {
                let new_gensym_index = self.next_symbol_index + 1;
                self.context.mems.push(Some(Id::Anonymous(new_gensym_index)));
            } else {
                self.context.mems.push(None);
            }
        } else {
            match token2 {
                tk!(TokenKind::Number(Number::Integer(_))) => self.context.mems.push(None),
                _ => {},
            }
        }
    }

    fn rewrite_memory_internal(&mut self, header: Vec<Token>, token1: Token, token2: Token, exporting: bool) -> Result<(), RewriteError> {
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
                            self.imports.push(Token::gensym(self.next_symbol_index - 1, Loc::zero()))
                        }

                        let rparen = self.lexer.next_token()?;

                        match self.lexer.next_token()? {
                            num1 @ tk!(TokenKind::Number(Number::Integer(_))) => {
                                self.imports.push(num1);
                                match self.lexer.next_token()? {
                                    num2 @ tk!(TokenKind::Number(Number::Integer(_))) => { 
                                        self.imports.push(num2);
                                        let rparen = self.lexer.next_token()?;
                                        self.imports.push(rparen);
                                    },
                                    rparen_memory @ _ => {
                                        // let rparen = self.lexer.next_token()?;
                                        self.imports.push(rparen_memory);
                                    },
                                }      
                            },
                            token @ _ => self.imports.push(token),
                        }
                        self.imports.push(rparen);
                    },
                    export @ kw!(Keyword::Export) => {
                        self.exports.push(lparen);
                        self.exports.push(export);
                        let name = self.lexer.next_token()?;
                        self.exports.push(name);

                        for t in header.clone() { self.exports.push(t); }
                        if header.len() == 2 {
                            if exporting {
                                self.exports.push(Token::gensym(self.next_symbol_index - 1, Loc::zero()));
                            } else {
                                let gensym = self.make_gensym();
                                self.exports.push(gensym);
                            }
                        }

                        self.exports.push(Token::right_paren(Loc::zero()));
                        let rparen_global = self.lexer.next_token()?;
                        self.exports.push(rparen_global);

                        let token1 = self.lexer.next_token()?;
                        let token2 = self.lexer.next_token()?;
                        return self.rewrite_memory_internal(header, token1, token2, true);
                    },
                    data @ kw!(Keyword::Data) => {
                        for t in header.clone() { self.mems.push(t); }
                        if header.len() == 2 {
                            if exporting {
                                self.mems.push(Token::gensym(self.next_symbol_index - 1, Loc::zero()));
                            } else {
                                let gensym = self.make_gensym();
                                self.mems.push(gensym);
                            }
                            
                        }
                        return self.rewrite_memory_data(&header, lparen, data);
                    },
                    _ => {
                        for t in header { self.mems.push(t); }
                        self.mems.push(lparen);
                        self.mems.push(token2);
                    },
                }
            },
            num1 @ tk!(TokenKind::Number(Number::Integer(_))) => {
                for t in header.clone() { self.mems.push(t); }
                if exporting && header.len() == 2 {
                    self.mems.push(Token::gensym(self.next_symbol_index - 1, Loc::zero()))
                }

                self.mems.push(num1);
                match token2 {
                    num2 @ tk!(TokenKind::Number(Number::Integer(_))) => {
                        self.mems.push(num2);
                        let rparen = self.lexer.next_token()?;
                        self.mems.push(rparen);
                    },
                    rparen @ _ => self.mems.push(rparen),
                }
            },
            _ => {
                for t in header { self.mems.push(t); }
                self.mems.push(token1);
                self.mems.push(token2);
            },
        }

        Ok(())
    }

    fn rewrite_memory_data(&mut self, tokens: &Vec<Token>, token1: Token, token2: Token) -> Result<(), RewriteError> {
        let mut tokens_data = vec![];

        tokens_data.push(Token::right_paren(Loc::zero()));

        self.data.push(token1);  
        self.data.push(token2);                  
        
        match tokens.len() {
            2 => self.data.push(Token::gensym(self.next_symbol_index - 1, Loc::zero())),
            3 => self.data.push(tokens[2].clone()),
            _ => {},
        }

        self.data.push(Token::left_paren(Loc::zero()));
        self.data.push(Token::keyword(Keyword::Instr(Instr::I32Const(0)), Loc::zero()));
        self.data.push(Token::number_u(0, Loc::zero()));
        self.data.push(Token::right_paren(Loc::zero()));

        let mut n = 0;
        while let Ok(token) = self.lexer.next_token() {
            match token {
                tk!(TokenKind::RightParen) => {
                    self.data.push(token);
                    break;
                },
                tk!(TokenKind::Empty) => {
                    self.data.push(token);
                    break;
                },
                tk!(TokenKind::String(ref s)) => {
                    self.data.push(token.clone());
                    n += s.len();
                }
                _ => {                                
                    self.data.push(token);
                    break;
                }
            }
        }

        let n = (n / (64*1024)) + 1;

        self.mems.push(Token::number_u(n, Loc::zero()));
        self.mems.push(Token::number_u(n, Loc::zero()));
        for t in tokens_data {
            self.mems.push(t);
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
        r#"(module (memory <#:gensym(0)> 5) (export "e1" (memory <#:gensym(0)>)))"#
    );
    assert_eq_rewrite(
        r#"(memory $id (export "e2") 50 100)"#, 
        r#"(module (memory $id 50 100) (export "e2" (memory $id)))"#
    );
    assert_eq_rewrite(
        r#"(memory (export "e3") (export "e4") 512)"#, 
        r#"(module (memory <#:gensym(0)> 512) (export "e3" (memory <#:gensym(0)>)) (export "e4" (memory <#:gensym(0)>)))"#
    );
    assert_eq_rewrite(
        r#"(memory $id2 (export "e5") (export "e6") 10000 20000)"#,
        r#"(module (memory $id2 10000 20000) (export "e5" (memory $id2)) (export "e6" (memory $id2)))"#
    );
}

#[test]
fn test_rewrite_mem_import_export() {
    assert_eq_rewrite(
        r#"(memory (export "e3") (import "n1" "n2") 1234)"#, 
        r#"(module (import "n1" "n2" (memory <#:gensym(0)> 1234)) (export "e3" (memory <#:gensym(0)>)))"#
    );
    assert_eq_rewrite(
        r#"(memory $id (export "e3") (import "n1" "n2") 4321 5678)"#, 
        r#"(module (import "n1" "n2" (memory $id 4321 5678)) (export "e3" (memory $id)))"#
    );
}

#[test]
fn test_rewrite_mem_data() {
    assert_eq_rewrite(
        r#"(module (memory (data "abcd" "wowow" "wasmiq")))"#, 
        r#"(module (memory <#:gensym(0)> 1 1) (data <#:gensym(0)> (i32.const 0) "abcd" "wowow" "wasmiq"))"#
    );
    assert_eq_rewrite(
        r#"(module (memory $id (data "abcd" "wowow" "wasmiq")))"#, 
        r#"(module (memory $id 1 1) (data $id (i32.const 0) "abcd" "wowow" "wasmiq"))"#
    );
    assert_eq_rewrite(
        r#"(module (memory (export "n1") (data "abcd" "wowow" "wasmiq")))"#, 
        r#"(module (memory <#:gensym(0)> 1 1) (export "n1" (memory <#:gensym(0)>)) (data <#:gensym(0)> (i32.const 0) "abcd" "wowow" "wasmiq"))"#
    );
    assert_eq_rewrite(
        r#"(module (memory $id (export "n1") (data "abcd" "wowow" "wasmiq")))"#, 
        r#"(module (memory $id 1 1) (export "n1" (memory $id)) (data $id (i32.const 0) "abcd" "wowow" "wasmiq"))"#
    );
}