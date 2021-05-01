use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_global(&mut self, lparen_global: Token, global: Token) -> Result<(), RewriteError> {
        let mut header = vec![lparen_global, global];
        let maybe_id = self.lexer.next_token()?;
        if let tk!(TokenKind::Id(s)) = maybe_id.clone() {
            self.context.globals.push(Some(Id::Named(s)));
        }
        let token1 = self.scan_id(maybe_id, &mut header)?;
        let token2 = self.lexer.next_token()?;

        self.set_context_id_func_global(token1.clone(), token2.clone());

        self.rewrite_global_internal(header, token1, token2, false)
    }

    fn set_context_id_func_global(&mut self, token1: Token, token2: Token) {
        if let tk!(TokenKind::LeftParen) = token1 {
            if let kw!(Keyword::Export) = token2 {
                let new_gensym_index = self.next_symbol_index + 1;
                self.context.globals.push(Some(Id::Anonymous(new_gensym_index)));
            } else {
                self.context.globals.push(None);
            }
        } else {
            match token2 {
                kw!(Keyword::ValType(_)) => self.context.globals.push(None),
                _ => {},
            }
        }
    }

    fn rewrite_global_internal(&mut self, header: Vec<Token>, token1: Token, token2: Token, exporting: bool) -> Result<(), RewriteError> {
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
                            self.imports.push(Token::gensym(self.next_symbol_index - 1, Loc::zero()));
                        }

                        let rparen = self.lexer.next_token()?;

                        match self.lexer.next_token()? {
                            lparen @ tk!(TokenKind::LeftParen) => {
                                self.imports.push(lparen);
                                self.imports.push(self.lexer.next_token()?);
                                self.imports.push(self.lexer.next_token()?);
                                self.imports.push(self.lexer.next_token()?);
                            },
                            valtype @ kw!(Keyword::ValType(_)) => {
                                self.imports.push(valtype);
                            },
                            _ => {},
                        }
                        self.imports.push(rparen);
                        let rparen = self.lexer.next_token()?;
                        
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
                        return self.rewrite_global_internal(header, token1, token2, true);
                    },
                    mutable @ kw!(Keyword::Mutable) => {
                        for t in header.clone() { self.globals.push(t); }
                        if exporting && header.len() == 2 {
                            self.globals.push(Token::gensym(self.next_symbol_index - 1, Loc::zero()))
                        }
                        self.globals.push(lparen);
                        self.globals.push(mutable);
                        let valtype = self.lexer.next_token()?;
                        self.globals.push(valtype);
                        let rparen = self.lexer.next_token()?;
                        self.globals.push(rparen);

                        let token = self.lexer.next_token()?;
                        let instrs = self.rewrite_instrs(vec![token])?;
                        self.globals.extend(instrs);
                    },
                    _ => {
                        for t in header { self.globals.push(t); }

                        self.globals.push(lparen);
                        self.globals.push(token2);
                    },
                }
            },
            valtype @ kw!(Keyword::ValType(_)) => {
                for t in header.clone() { self.globals.push(t); }
                if exporting && header.len() == 2 {
                    self.globals.push(Token::gensym(self.next_symbol_index - 1, Loc::zero()))
                }
                self.globals.push(valtype);
                
                match token2 {
                    rparen @ tk!(TokenKind::RightParen) => self.globals.push(rparen),
                    _ => {
                        let instrs = self.rewrite_instrs(vec![token2])?;
                        self.globals.extend(instrs);
                    },
                }
            },
            _ => {
                for t in header { self.globals.push(t); }
                self.globals.push(token1);
                self.globals.push(token2);
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
        r#"(module (global <#:gensym(0)> i32) (export "n1" (global <#:gensym(0)>)))"#
    );
    assert_eq_rewrite(
        r#"(global $id (export "e2") (mut f64) nop)"#, 
        r#"(module (global $id (mut f64) nop) (export "e2" (global $id)))"#
    );
    assert_eq_rewrite(
        r#"(global (export "e3") (export "e4") i64 nop)"#, 
        r#"(module (global <#:gensym(0)> i64 nop) (export "e3" (global <#:gensym(0)>)) (export "e4" (global <#:gensym(0)>)))"#
    );
    assert_eq_rewrite(
        r#"(global $id (export "e5") (export "e6") (mut f32))"#, 
        r#"(module (global $id (mut f32)) (export "e5" (global $id)) (export "e6" (global $id)))"#
    );
}

#[test]
fn test_rewrite_global_import_export() {
    assert_eq_rewrite(
        r#"(global (export "e3") (import "n1" "n2") i32)"#, 
        r#"(module (import "n1" "n2" (global <#:gensym(0)> i32)) (export "e3" (global <#:gensym(0)>)))"#
    );
    assert_eq_rewrite(
        r#"(global $id (export "e3") (import "n1" "n2") (mut f64))"#, 
        r#"(module (import "n1" "n2" (global $id (mut f64))) (export "e3" (global $id)))"#
    );
}

#[test]
fn test_rewrite_global_single_instr() {
    assert_eq_rewrite(
        r#"(global i32 (i32.const 1))"#,
        r#"(module (global i32 i32.const 1))"#
    );
}