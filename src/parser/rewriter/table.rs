use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_table(&mut self, token_table: Token) -> Result<(), RewriteError> {
        let mut tokens = vec![token_table];
        let token = self.lexer.next_token()?;
        let token1 = self.scan_id(token, &mut tokens)?;
        let token2 = self.lexer.next_token()?;

        if Rewriter::<R>::is_import_or_export(&token1, &token2) {
            let token_leftparen = token1.clone();
            self.rewrite_inline_export_import(tokens.clone(), token_leftparen.clone(), token2.clone())?;
            let token_rightparen = self.lexer.next_token()?;
            let token = self.lexer.next_token()?;
            match &token {
                token_num1 @ tk!(TokenKind::Number(Number::Integer(_))) => {
                    self.ast.push(token_num1.clone());
                    let token = self.lexer.next_token()?;
                    match token {
                        token_num2 @ tk!(TokenKind::Number(Number::Integer(_))) => {
                            self.ast.push(token_num2.clone());
                            let token_funcref = self.lexer.next_token()?;
                            self.ast.push(token_funcref);
                        },
                        _ => {
                            self.ast.push(token);
                        },
                    }
                },
                // kw!(Keyword::FuncRef) => {
                //     self.rewrite_table_elem(&tokens, token1, token2)?;
                // },
                _ => {
                    self.ast.push(token.clone());
                },
            }
            self.ast.push(token_rightparen);
        } else {
            for t in &tokens { self.ast.push(t.clone()); }
            match &token1 {
                kw!(Keyword::FuncRef) => {                    
                    self.rewrite_table_elem(&tokens, token1, token2)?;
                },
                _ => {
                    self.ast.push(token1);
                    self.ast.push(token2);        
                },
            }
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
        
        if tokens.len() == 2 {
            tokens_elem.push(tokens[1].clone());
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
        r#"(table $expid1 (export "expname2"))"#, 
        r#"(module (export "expname2" (table $expid1)))"#
    );
    assert_eq_rewrite(
        r#"(table (export "expname3") (export "expname4"))"#, 
        r#"(module (export "expname3" (table <#:gensym>)) (export "expname4" (table <#:gensym>)))"#
    );
    assert_eq_rewrite(
        r#"(table $expid2 (export "expname5") (export "expname6"))"#, 
        r#"(module (export "expname5" (table $expid2)) (export "expname6" (table $expid2)))"#
    );
}

#[test]
fn test_rewrite_table_import_export() {
    assert_eq_rewrite(
        r#"(table $expimpid (export "expname3") (import "impname1" "impname2") 1234 funcref)"#, 
        r#"(module (export "expname3" (table $expimpid)) (import "impname1" "impname2" (table $expimpid 1234 funcref)))"#
    );
    assert_eq_rewrite(
        r#"(table (export "expname3") (import "impname1" "impname2") 4321 5678 funcref)"#, 
        r#"(module (export "expname3" (table <#:gensym>)) (import "impname1" "impname2" (table 4321 5678 funcref)))"#
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
    // assert_eq_rewrite(
    //     r#"(module (table (export "n1") funcref (elem 1 2 4 8 16 32 64 128 256 512)))"#, 
    //     r#"(module (export "n1" (table <#:gensym>)) (table 11 11 funcref) (elem (i32.const 0) 1 2 4 8 16 32 64 128 256 512)))"#
    // );
    // assert_eq_rewrite(
    //     r#"(module (table $id (export "n1") funcref (elem 1 2 4 8 16 32 64 128 256 512)))"#, 
    //     r#"(module (export "n1" (table $id )) (table $id 11 11 funcref) (elem $id (i32.const 0) 1 2 4 8 16 32 64 128 256 512)))"#
    // );
}