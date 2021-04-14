use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_memory(&mut self, token_mem: Token) -> Result<(), RewriteError> {
        let mut tokens = vec![token_mem];
        let token = self.lexer.next_token()?;
        let token1 = self.scan_id(token, &mut tokens)?;
        let token2 = self.lexer.next_token()?;

        if Rewriter::<R>::is_import_or_export(&token1, &token2) {
            let token_leftparen = token1.clone();
            self.rewrite_inline_export_import_internal(tokens, token_leftparen.clone(), token2)?;
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
            self.ast.push(token1);
            self.ast.push(token2);
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