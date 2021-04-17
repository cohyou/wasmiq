use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_global(&mut self, lparen_global: Token, global: Token) -> Result<(), RewriteError> {
        let mut header = vec![lparen_global, global];
        let maybe_id = self.lexer.next_token()?;
        let token1 = self.scan_id(maybe_id, &mut header)?;
        let token2 = self.lexer.next_token()?;
        self.rewrite_global_internal(header, token1, token2, false)
    }

    fn rewrite_global_internal(&mut self, header: Vec<Token>, token1: Token, token2: Token, exporting: bool) -> Result<(), RewriteError> {
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
                            lparen @ tk!(TokenKind::LeftParen) => {
                                self.ast.push(lparen);
                                self.ast.push(self.lexer.next_token()?);
                                self.ast.push(self.lexer.next_token()?);
                                self.ast.push(self.lexer.next_token()?);
                            },
                            valtype @ kw!(Keyword::ValType(_)) => {
                                self.ast.push(valtype);
                            },
                            _ => {},
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
                        return self.rewrite_global_internal(header, token1, token2, true);
                    },
                    mutable @ kw!(Keyword::Mutable) => {
                        for t in header.clone() { self.ast.push(t); }
                        if exporting && header.len() == 2 {
                            self.ast.push(Token::gensym(Loc::zero()))
                        }
                        self.ast.push(lparen.clone());
                        self.ast.push(mutable);
                        let valtype = self.lexer.next_token()?;
                        self.ast.push(valtype);
                        let rparen = self.lexer.next_token()?;
                        self.ast.push(rparen);
                    },
                    _ => {
                        for t in header { self.ast.push(t); }

                        self.ast.push(lparen);
                        self.ast.push(token2);
                    },
                }
            },
            valtype @ kw!(Keyword::ValType(_)) => {
                for t in header.clone() { self.ast.push(t); }
                if exporting && header.len() == 2 {
                    self.ast.push(Token::gensym(Loc::zero()))
                }
                self.ast.push(valtype);
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
}

#[test]
fn test_rewrite_global() {
    assert_eq_rewrite(
        "(module (global i32))", 
        "(module (global i32))"
    );
    assert_eq_rewrite(
        "(global i32 nop)", 
        "(module (global i32 nop))"
    );
    assert_eq_rewrite(
        "(global i32 i64.const 8128)", 
        "(module (global i32 i64.const 8128))"
    );
    assert_eq_rewrite(
        "(global (mut f32) i64.const 8128)", 
        "(module (global (mut f32) i64.const 8128))"
    );
    assert_eq_rewrite(
        "(global $id1 i32)", 
        "(module (global $id1 i32))"
    );
    assert_eq_rewrite(
        "(global $id2 i32 i64.const 8128)", 
        "(module (global $id2 i32 i64.const 8128))"
    );
    assert_eq_rewrite(
        "(global $id3 (mut f32) i64.const 8128)", 
        "(module (global $id3 (mut f32) i64.const 8128))"
    );
}
    
#[test]
fn test_rewrite_global_import() {
    assert_eq_rewrite(
        r#"(global (import "n1" "n2") i32)"#, 
        r#"(module (import "n1" "n2" (global i32)))"#
    );
    assert_eq_rewrite(
        r#"(global (import "n1" "n2") (mut f64))"#,
        r#"(module (import "n1" "n2" (global (mut f64))))"#
    );
    assert_eq_rewrite(
        r#"(global $imp_global_const1 (import "n1" "n2") i64)"#, 
        r#"(module (import "n1" "n2" (global $imp_global_const1 i64)))"#
    );
    assert_eq_rewrite(
        r#"(global $imp_global_const2 (import "n1" "n2") (mut f32))"#, 
        r#"(module (import "n1" "n2" (global $imp_global_const2 (mut f32))))"#
    );
}

#[test]
fn test_rewrite_global_export() {
    assert_eq_rewrite(
        r#"(global (export "n1") i32)"#, 
        r#"(module (export "n1" (global <#:gensym>)) (global <#:gensym> i32))"#
    );
    assert_eq_rewrite(
        r#"(global $id (export "e2") (mut f64) nop)"#, 
        r#"(module (export "e2" (global $id)) (global $id (mut f64) nop))"#
    );
    assert_eq_rewrite(
        r#"(global (export "e3") (export "e4") i64 nop)"#, 
        r#"(module (export "e3" (global <#:gensym>)) (export "e4" (global <#:gensym>)) (global <#:gensym> i64 nop))"#
    );
    assert_eq_rewrite(
        r#"(global $id (export "e5") (export "e6") (mut f32))"#, 
        r#"(module (export "e5" (global $id)) (export "e6" (global $id)) (global $id (mut f32)))"#
    );
}

#[test]
fn test_rewrite_global_import_export() {
    assert_eq_rewrite(
        r#"(global (export "e3") (import "n1" "n2") i32)"#, 
        r#"(module (export "e3" (global <#:gensym>)) (import "n1" "n2" (global <#:gensym> i32)))"#
    );
    assert_eq_rewrite(
        r#"(global $id (export "e3") (import "n1" "n2") (mut f64))"#, 
        r#"(module (export "e3" (global $id)) (import "n1" "n2" (global $id (mut f64))))"#
    );
}