use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_import(&mut self, lparen_import: Token, import: Token) -> Result<(), RewriteError> {
        let mut tokens = vec![];
        tokens.push(lparen_import);
        tokens.push(import);

        let n1 = self.lexer.next_token()?;
        tokens.push(n1);
        let n2 = self.lexer.next_token()?;
        tokens.push(n2);
        let lparen_desc = self.lexer.next_token()?;
        tokens.push(lparen_desc);

        match self.lexer.next_token()? {
            func @ kw!(Keyword::Func) => {
                tokens.push(func);

                let t = self.lexer.next_token()?;

                let next = if let id @ tk!(TokenKind::Id(_)) = t {
                    tokens.push(id);
                    self.lexer.next_token()?
                } else {
                    t
                };

                match next {
                    lparen @ tk!(TokenKind::LeftParen) => {
                        match self.lexer.next_token()? {
                            type_ @ kw!(Keyword::Type) => {
                                tokens.push(lparen);
                                tokens.push(type_);
                                let typeidx = self.lexer.next_token()?;
                                tokens.push(typeidx);
                                let rparen_type = self.lexer.next_token()?;
                                tokens.push(rparen_type);
                                let token1 = self.lexer.next_token()?;
                                let token2 = self.lexer.next_token()?;
                                let typeuse = self.rewrite_import_typeuse(token1, token2)?;
                                tokens.extend(typeuse);
                            },
                            t @ _ => {
                                tokens.extend(self.make_type_gensym_tokens());
                                let typeuse = self.rewrite_import_typeuse(lparen, t)?;
                                tokens.extend(typeuse);
                            }
                        }
                    },
                    rparen @ tk!(TokenKind::RightParen) => {
                        tokens.extend(self.make_type_gensym_tokens());
                        tokens.push(rparen);
                    },
                    t @ _ => tokens.push(t),
                }
            },
            t @ _ => {
                tokens.push(t);
                let importdesc = self.scan_simple_list()?;
                tokens.extend(importdesc);
            },
        }

        self.imports.extend(tokens);

        Ok(())
    }

    fn rewrite_import_typeuse(&mut self, token1: Token, token2: Token) -> Result<Vec<Token>, RewriteError> {
        let mut tokens = vec![];

        match token1 {
            rparen_desc @ tk!(TokenKind::RightParen) => {
                tokens.push(rparen_desc);
                tokens.push(token2);
            },
            lparen @ tk!(TokenKind::LeftParen) => {
                tokens.push(lparen);

                match token2 {
                    param @ kw!(Keyword::Param) => {
                        tokens.push(param);
                        let params = self.rewrite_valtypes(Keyword::Param)?;
                        tokens.extend(params);

                        let token1 = self.lexer.next_token()?;
                        let token2 = self.lexer.next_token()?;
                        let ret = self.rewrite_import_typeuse(token1, token2)?;
                        tokens.extend(ret);
                        // match self.lexer.next_token()? {
                        //     rparen @ tk!(TokenKind::RightParen) => {
                        //         tokens.push(rparen);
                        //     },
                        //     lparen @ tk!(TokenKind::LeftParen) => {
                        //         tokens.push(lparen);
                        //         match self.lexer.next_token()? {
                        //             result @ kw!(Keyword::Result) => {
                        //                 tokens.push(result);
                        //                 let results = self.rewrite_valtypes(Keyword::Result)?;
                        //                 tokens.extend(results); 
                        //                 let rparen_desc = self.lexer.next_token()?;
                        //                 tokens.push(rparen_desc);
                        //                 let rparen_import = self.lexer.next_token()?;
                        //                 tokens.push(rparen_import);
                        //             }                    
                        //             t @ _ => tokens.push(t), 
                        //         }
                        //     },
                        //     t @ _ => tokens.push(t),
                        // }
                    },
                    result @ kw!(Keyword::Result) => {
                        tokens.push(result);
                        let results = self.rewrite_valtypes(Keyword::Result)?;
                        tokens.extend(results);

                        let token1 = self.lexer.next_token()?;
                        let token2 = self.lexer.next_token()?;
                        let ret = self.rewrite_import_typeuse_rest(token1, token2)?;
                        tokens.extend(ret);

                        // let rparen_desc = self.lexer.next_token()?;
                        // tokens.push(rparen_desc);
                        // let rparen_import = self.lexer.next_token()?;
                        // tokens.push(rparen_import);
                    },
                    t @ _ => tokens.push(t), 
                }
            },
            t @ _ => {
                tokens.push(t);
                tokens.push(token2);
            }, 
        }
    
        Ok(tokens)
    }

    fn rewrite_import_typeuse_rest(&mut self, token1: Token, token2: Token) -> Result<Vec<Token>, RewriteError> {
        let mut tokens = vec![];

        match token1 {
            rparen_desc @ tk!(TokenKind::RightParen) => {
                tokens.push(rparen_desc);
                tokens.push(token2);
            },
            lparen @ tk!(TokenKind::LeftParen) => {
                tokens.push(lparen);

                match token2 {
                    result @ kw!(Keyword::Result) => {
                        tokens.push(result);
                        let results = self.rewrite_valtypes(Keyword::Result)?;
                        tokens.extend(results);
                        let rparen_desc = self.lexer.next_token()?;
                        tokens.push(rparen_desc);
                        let rparen_import = self.lexer.next_token()?;
                        tokens.push(rparen_import);
                    },
                    t @ _ => tokens.push(t), 
                }
            },
            t @ _ => {
                tokens.push(t);
                tokens.push(token2);
            }, 
        }
    
        Ok(tokens)
    }
}

#[test]
fn test_import1() {
    assert_eq_rewrite(
        r#"(import "mod" "nm" (func))"#, 
        r#"(module (import "mod" "nm" (func (type <#:gensym(0)>))))"#
    );
    assert_eq_rewrite(
        r#"(import "mod" "nm" (func $id))"#, 
        r#"(module (import "mod" "nm" (func $id (type <#:gensym(0)>))))"#
    );
    assert_eq_rewrite(
        r#"(import "mod" "nm" (func (type 0)))"#,
        r#"(module (import "mod" "nm" (func (type 0))))"#
    );
    assert_eq_rewrite(
        r#"(import "mod" "nm" (func $id (type 0)))"#,
        r#"(module (import "mod" "nm" (func $id (type 0))))"#
    );
}

#[test]
fn test_import2() {
    assert_eq_rewrite(
        r#"(import "mod" "nm" (func (param i32)))"#,
        r#"(module (import "mod" "nm" (func (type <#:gensym(0)>) (param i32))))"#
    );
    assert_eq_rewrite(
        r#"(import "mod" "nm" (func $id (param i32)))"#,
        r#"(module (import "mod" "nm" (func $id (type <#:gensym(0)>) (param i32))))"#
    );
    assert_eq_rewrite(
        r#"(import "mod" "nm" (func (result i64)))"#,
        r#"(module (import "mod" "nm" (func (type <#:gensym(0)>) (result i64))))"#
    );
    assert_eq_rewrite(
        r#"(import "mod" "nm" (func $id (result i64)))"#,
        r#"(module (import "mod" "nm" (func $id (type <#:gensym(0)>) (result i64))))"#
    );
}

#[test]
fn test_import3() {
    assert_eq_rewrite(
        r#"(import "mod" "nm" (func $id (param f32) (result i64)))"#,
        r#"(module (import "mod" "nm" (func $id (type <#:gensym(0)>) (param f32) (result i64))))"#
    );
    assert_eq_rewrite(
        r#"(import "mod" "nm" (func (param f32) (result i64)))"#,
        r#"(module (import "mod" "nm" (func (type <#:gensym(0)>) (param f32) (result i64))))"#
    );
    assert_eq_rewrite(
        r#"(import "mod" "nm" (func $id (param f32) (result i64)))"#,
        r#"(module (import "mod" "nm" (func $id (type <#:gensym(0)>) (param f32) (result i64))))"#
    );
}

#[test]
fn test_import4() {
    assert_eq_rewrite(
        r#"(import "mod" "nm" (func (param f32 f64) (result i64)))"#,
        r#"(module (import "mod" "nm" (func (type <#:gensym(0)>) (param f32) (param f64) (result i64))))"#
    );
    assert_eq_rewrite(
        r#"(import "mod" "nm" (func $fn (type 0) (param $pr f32) (result i32 i64)))"#,
        r#"(module (import "mod" "nm" (func $fn (type 0) (param $pr f32) (result i32) (result i64))))"#
    );
    assert_eq_rewrite(
        r#"(import "mod" "nm" (func $fn (param $pr f32) (param f64 i32) (result i32 i64)))"#,
        r#"(module (import "mod" "nm" (func $fn (type <#:gensym(0)>) (param $pr f32) (param f64) (param i32) (result i32) (result i64))))"#
    );
}