use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_global(&mut self, token_global: Token) -> Result<(), RewriteError> {
        let mut tokens = vec![token_global];
        let token = self.lexer.next_token()?;
        let token1 = self.scan_id(token, &mut tokens)?;
        let token2 = self.lexer.next_token()?;

        if Rewriter::<R>::is_import_or_export(&token1, &token2) {
            let token_leftparen = token1.clone();
            self.rewrite_inline_export_import_internal(tokens, token_leftparen.clone(), token2)?;
            let token_rightparen = self.lexer.next_token()?;
            let token = self.lexer.next_token()?;
            match &token {
                token_type1 @ kw!(Keyword::ValType(_)) => {
                    self.ast.push(token_type1.clone());
                    let token_rightparen2 = self.lexer.next_token()?;
                    self.ast.push(token_rightparen2);
                },
                token_leftparen @ tk!(TokenKind::LeftParen) => {
                    self.ast.push(token_leftparen.clone());
                    let token_leftparen_mut = self.lexer.next_token()?;
                    self.ast.push(token_leftparen_mut);
                    let token_type = self.lexer.next_token()?;
                    self.ast.push(token_type);
                    let token_rightparen_mut = self.lexer.next_token()?;
                    self.ast.push(token_rightparen_mut);                  
                    let token_rightparen2 = self.lexer.next_token()?;
                    self.ast.push(token_rightparen2);
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
fn test_rewrite_global() {
    assert_eq_rewrite(
        "(global i32 end)", 
        "(module (global i32 end))"
    );
    assert_eq_rewrite(
        "(global i32 i64.const 8128 end)", 
        "(module (global i32 i64.const 8128 end))"
    );
    assert_eq_rewrite(
        "(global (mut f32) i64.const 8128 end)", 
        "(module (global (mut f32) i64.const 8128 end))"
    );
    assert_eq_rewrite(
        "(global $id1 i32 end)", 
        "(module (global $id1 i32 end))"
    );
    assert_eq_rewrite(
        "(global $id2 i32 i64.const 8128 end)", 
        "(module (global $id2 i32 i64.const 8128 end))"
    );
    assert_eq_rewrite(
        "(global $id3 (mut f32) i64.const 8128 end)", 
        "(module (global $id3 (mut f32) i64.const 8128 end))"
    );
}
    
#[test]
fn test_rewrite_global_import() {
    assert_eq_rewrite(
        r#"(global (import "n1" "n2") i32)"#, 
        r#"(module (import "n1" "n2" (global i32)))"#
    );
    assert_eq_rewrite(
        r#"(global (import "name1" "name2") (mut f64))"#,
        r#"(module (import "name1" "name2" (global (mut f64))))"#
    );
    assert_eq_rewrite(
        r#"(global $imp_global_const1 (import "name1" "name2") i64)"#, 
        r#"(module (import "name1" "name2" (global $imp_global_const1 i64)))"#
    );
    assert_eq_rewrite(
        r#"(global $imp_global_const2 (import "name1" "name2") (mut f32))"#, 
        r#"(module (import "name1" "name2" (global $imp_global_const2 (mut f32))))"#
    );
}

#[test]
fn test_rewrite_global_export() {
    assert_eq_rewrite(
        r#"(global (export "n1"))"#, 
        r#"(module (export "n1" (global <#:gensym>)))"#
    );
    assert_eq_rewrite(
        r#"(global $expid1 (export "expname2"))"#, 
        r#"(module (export "expname2" (global $expid1)))"#
    );
    assert_eq_rewrite(
        r#"(global (export "expname3") (export "expname4"))"#, 
        r#"(module (export "expname3" (global <#:gensym>)) (export "expname4" (global <#:gensym>)))"#
    );
    assert_eq_rewrite(
        r#"(global $expid2 (export "expname5") (export "expname6"))"#, 
        r#"(module (export "expname5" (global $expid2)) (export "expname6" (global $expid2)))"#
    );
}

#[test]
fn test_rewrite_global_import_export() {
    assert_eq_rewrite(
        r#"(global (export "expname3") (import "impname1" "impname2") i32)"#, 
        r#"(module (export "expname3" (global <#:gensym>)) (import "impname1" "impname2" (global i32)))"#
    );
    assert_eq_rewrite(
        r#"(global $expimpid (export "expname3") (import "impname1" "impname2") (mut f64))"#, 
        r#"(module (export "expname3" (global $expimpid)) (import "impname1" "impname2" (global $expimpid (mut f64))))"#
    );
}
