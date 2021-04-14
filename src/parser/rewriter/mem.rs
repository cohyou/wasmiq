use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_memory(&mut self, token_mem: Token) -> Result<(), RewriteError> {
        let mut tokens = vec![token_mem];
        let token = self.lexer.next_token()?;
        let token1 = self.scan_id(token, &mut tokens)?;
        let token2 = self.lexer.next_token()?;

        if Rewriter::<R>::is_import_or_export(&token1, &token2) {
            let token_leftparen = token1.clone();
            self.rewrite_inline_export_import(tokens, token_leftparen.clone(), token2)?;
            let token_rightparen = self.lexer.next_token()?;
            let token = self.lexer.next_token()?;
            match &token {
                token_num1 @ tk!(TokenKind::Number(Number::Integer(_))) => {
                    self.ast.push(token_num1.clone());
                    let token = self.lexer.next_token()?;
                    match token {
                        token_num2 @ tk!(TokenKind::Number(Number::Integer(_))) => {
                            self.ast.push(token_num2.clone());
                            let token_rightparen2 = self.lexer.next_token()?;
                            self.ast.push(token_rightparen2);
                        },
                        _ => {
                            self.ast.push(token);
                        },
                    }
                },
                _ => {
                    self.ast.push(token.clone());
                },
            }
            self.ast.push(token_rightparen);
        } else {
            for t in &tokens { self.ast.push(t.clone()); }
            if let tk!(TokenKind::LeftParen) = token1 {
                match &token2 {
                    kw!(Keyword::Data) => {
                        self.rewrite_memory_data(&tokens, token1, token2)?;
                    },
                    _ => {
                        self.ast.push(token1);
                        self.ast.push(token2);        
                    },
                }
            } else {
                self.ast.push(token1);
                self.ast.push(token2);
            }
        }

        Ok(())
    }

    fn rewrite_memory_data(&mut self, tokens: &Vec<Token>, token1: Token, token2: Token) -> Result<(), RewriteError> {
        let mut tokens_data = vec![];
        // tokens_data.push(token1);
        tokens_data.push(Token::right_paren(Loc::zero()));

        tokens_data.push(token1);  
        tokens_data.push(token2);                  
        // let token_elem = self.lexer.next_token()?;
        // tokens_data.push(token_elem); 
        
        if tokens.len() == 2 {
            tokens_data.push(tokens[1].clone());
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
        r#"(memory (export "expname1"))"#, 
        r#"(module (export "expname1" (memory <#:gensym>)))"#
    );
    assert_eq_rewrite(
        r#"(memory $expid1 (export "expname2"))"#, 
        r#"(module (export "expname2" (memory $expid1)))"#
    );
    assert_eq_rewrite(
        r#"(memory (export "expname3") (export "expname4"))"#, 
        r#"(module (export "expname3" (memory <#:gensym>)) (export "expname4" (memory <#:gensym>)))"#
    );
    assert_eq_rewrite(
        r#"(memory $expid2 (export "expname5") (export "expname6"))"#, 
        r#"(module (export "expname5" (memory $expid2)) (export "expname6" (memory $expid2)))"#
    );
}

#[test]
fn test_rewrite_mem_import_export() {
    assert_eq_rewrite(
        r#"(memory (export "expname3") (import "impname1" "impname2") 1234)"#, 
        r#"(module (export "expname3" (memory <#:gensym>)) (import "impname1" "impname2" (memory 1234)))"#
    );
    assert_eq_rewrite(
        r#"(memory $expimpid (export "expname3") (import "impname1" "impname2") 4321 5678)"#, 
        r#"(module (export "expname3" (memory $expimpid)) (import "impname1" "impname2" (memory $expimpid 4321 5678)))"#
    );
}

#[test]
fn test_rewrite_table_elem() {
    assert_eq_rewrite(
        r#"(module (memory (data "abcd" "wowow" "wasmiq")))"#, 
        r#"(module (memory <#:gensym> 1 1) (data <#:gensym> (i32.const 0) "abcd" "wowow" "wasmiq")))"#
    );
    assert_eq_rewrite(
        r#"(module (memory $id (data "abcd" "wowow" "wasmiq")))"#, 
        r#"(module (memory $id 1 1) (data $id (i32.const 0) "abcd" "wowow" "wasmiq")))"#
    );
    // assert_eq_rewrite(
    //     r#"(module (memory (export "n1") (data "abcd" "wowow" "wasmiq")))"#, 
    //     r#"(module (export "n1" (memory <#:gensym>)) (memory <#:gensym> 1 1) (data <#:gensym> (i32.const 0) "abcd" "wowow" "wasmiq")))"#
    // );
    // assert_eq_rewrite(
    //     r#"(module (memory $id (export "n1") (data "abcd" "wowow" "wasmiq")))"#, 
    //     r#"(module (export "n1" (memory $id)) (memory $id 1 1) (data $id (i32.const 0) "abcd" "wowow" "wasmiq")))"#
    // );
}