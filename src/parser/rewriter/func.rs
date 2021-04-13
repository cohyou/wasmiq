use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_func(&mut self, token_func: Token) -> Result<(), RewriteError> {
        let mut tokens = vec![];
        let token = self.lexer.next_token()?;
        tokens.push(token_func);
        let token = match token {
            token_id @ tk!(TokenKind::Id(_)) => {
                tokens.push(token_id.clone());
                self.lexer.next_token()?
            },
            _ => token,
        };

        match token {
            token_rightparen @ tk!(TokenKind::RightParen) => {
                for t in &tokens { self.ast.push(t.clone()); }
                self.add_typeidx();
                self.ast.push(token_rightparen);
                return Ok(());
            },
            token_instr @ instr!(_) => {
                for t in &tokens { self.ast.push(t.clone()); }
                self.add_typeidx();
                self.ast.push(token_instr);
                return Ok(());
            },
            token_leftparen @ tk!(TokenKind::LeftParen) => {
                let token = self.lexer.next_token()?;
                match &token {
                    kw!(Keyword::Import) => {
                        self.rewrite_inline_export_import_internal(tokens.clone(), token_leftparen.clone())?;
                    },
                    kw!(Keyword::Export) => {
                        self.rewrite_inline_export_import_internal(tokens.clone(), token_leftparen.clone())?;
                    },
                    _ => {},
                }
                let token_keyword = match &token {
                    token_type @ kw!(Keyword::Type) => {
                        for t in &tokens { self.ast.push(t.clone()); }
                        self.ast.push(token_leftparen);
                        self.ast.push(token_type.clone());
                        let token_typeidx = self.lexer.next_token()?;
                        self.ast.push(token_typeidx);
                        let token_rightparen = self.lexer.next_token()?;
                        self.ast.push(token_rightparen);


                        let token_leftparen = self.lexer.next_token()?;
                        self.ast.push(token_leftparen);

                        let token = self.lexer.next_token()?;
                        match &token {
                            token_param @ kw!(Keyword::Param) => {
                                self.ast.push(token_param.clone());
                            },
                            token_result @ kw!(Keyword::Result) => {
                                self.ast.push(token_result.clone());
                            },
                            token_local @ kw!(Keyword::Local) => {
                                self.ast.push(token_local.clone());
                            },
                            _ => {
                                self.ast.push(token.clone());
                            },
                        }
                        token.clone()
                    },
                    token_param @ kw!(Keyword::Param) => {
                        for t in &tokens { self.ast.push(t.clone()); }
                        
                        self.ast.push(Token::left_paren(Loc::zero()));
                        self.ast.push(Token::keyword(Keyword::Type, Loc::zero()));
                        self.ast.push(Token::gensym(Loc::zero()));
                        self.ast.push(Token::right_paren(Loc::zero()));

                        self.ast.push(token_leftparen);
                        self.ast.push(token_param.clone());
                        token_param.clone()
                    },
                    token_result @ kw!(Keyword::Result) => {
                        for t in &tokens { self.ast.push(t.clone()); }
                        
                        self.ast.push(Token::left_paren(Loc::zero()));
                        self.ast.push(Token::keyword(Keyword::Type, Loc::zero()));
                        self.ast.push(Token::gensym(Loc::zero()));
                        self.ast.push(Token::right_paren(Loc::zero()));

                        self.ast.push(token_leftparen);
                        self.ast.push(token_result.clone());
                        token_result.clone()
                    },
                    token_local @ kw!(Keyword::Local) => {
                        for t in &tokens { self.ast.push(t.clone()); }
                        
                        self.ast.push(Token::left_paren(Loc::zero()));
                        self.ast.push(Token::keyword(Keyword::Type, Loc::zero()));
                        self.ast.push(Token::gensym(Loc::zero()));
                        self.ast.push(Token::right_paren(Loc::zero()));

                        self.ast.push(token_leftparen);
                        self.ast.push(token_local.clone());
                        token_local.clone()
                    },
                    _ => {
                        self.ast.push(token_leftparen.clone());
                        token.clone()
                    },
                };

                match &token_keyword {
                    kw!(Keyword::Param) => self.rewrite_param()?,
                    _ => {},
                }
                match &token_keyword {
                    kw!(Keyword::Result) => self.rewrite_result()?,
                    _ => {},
                }
                match &token_keyword {
                    kw!(Keyword::Local) => self.rewrite_local()?,
                    _ => {},
                }
            },
            _ => {
                for t in &tokens { self.ast.push(t.clone()); }
                return Ok(());
            },
        }

        Ok(())
    }

    fn add_typeidx(&mut self) {
        self.ast.push(Token::left_paren(Loc::zero()));
        self.ast.push(Token::keyword(Keyword::Type, Loc::zero()));
        self.ast.push(Token::gensym(Loc::zero()));
        self.ast.push(Token::right_paren(Loc::zero()));
    }
}

#[test]
fn test_rewrite_func_normal1() {
    assert_eq_rewrite("(func)", "(module (func (type <#:gensym>)))");
    assert_eq_rewrite("(func nop)", "(module (func (type <#:gensym>) nop))");
    assert_eq_rewrite("(func nop nop)", "(module (func (type <#:gensym>) nop nop))");
    assert_eq_rewrite("(func $id)", "(module (func $id (type <#:gensym>)))");
    assert_eq_rewrite("(func $id nop)", "(module (func $id (type <#:gensym>) nop))");
    assert_eq_rewrite("(func $id nop nop)", "(module (func $id (type <#:gensym>) nop nop))");
}

#[test]
fn test_rewrite_func_normal2() {
    assert_eq_rewrite("(func (type 0))", "(module (func (type 0)))");
    assert_eq_rewrite("(func (type $tp1))", "(module (func (type $tp1)))");
    assert_eq_rewrite("(func (type 0) nop)", "(module (func (type 0) nop))");
    assert_eq_rewrite("(func (type $tp1) nop)", "(module (func (type $tp1) nop))");
    assert_eq_rewrite("(func $id (type 0))", "(module (func $id (type 0)))");
    assert_eq_rewrite("(func $id (type $tp1))", "(module (func $id (type $tp1)))");
    assert_eq_rewrite("(func $id (type 0) nop)", "(module (func $id (type 0) nop))");
    assert_eq_rewrite("(func $id (type $tp1) nop)", "(module (func $id (type $tp1) nop))");
}

#[test]
fn test_rewrite_func_normal3() {
    assert_eq_rewrite("(func (param i32))", "(module (func (type <#:gensym>) (param i32)))");
    assert_eq_rewrite("(func $id (param i32 f64))", "(module (func $id (type <#:gensym>) (param i32) (param f64)))");
    assert_eq_rewrite("(func (type 0) (param i32))", "(module (func (type 0) (param i32)))");
    assert_eq_rewrite("(func (result i64))", "(module (func (type <#:gensym>) (result i64)))");
    assert_eq_rewrite("(func $id (type 0) (result i64 f32) i64.const 100)", "(module (func $id (type 0) (result i64) (result f32) i64.const 100))");
    assert_eq_rewrite("(func (local f64 i32))", "(module (func (type <#:gensym>) (local f64) (local i32)))");
    assert_eq_rewrite("(func $id (local f64))", "(module (func $id (type <#:gensym>) (local f64)))");
    assert_eq_rewrite("(func (type 0) (local f64 i32) nop nop)", "(module (func (type 0) (local f64) (local i32) nop nop))");
}

#[test]
fn test_rewrite_func_normal4() {
    assert_eq_rewrite(
        "(func (param i32) (result f32))", 
        "(module (func (type <#:gensym>) (param i32) (result f32)))"
    );
    assert_eq_rewrite(
        "(func (type 1) (param i32 i32) (result f32))", 
        "(module (func (type 1) (param i32) (param i32) (result f32)))"
    );
    assert_eq_rewrite(
        "(func $id (param i32 i32) (local i64 i64))", 
        "(module (func $id (type <#:gensym>) (param i32) (param i32) (local i64) (local i64)))"
    );
    assert_eq_rewrite(
        "(func $id (type 10) (result i32) (local i64 i64))",
        "(module (func $id (type 10) (result i32) (local i64) (local i64)))"
    );
    assert_eq_rewrite(
        "(func $id (type 10) (param f32 f32) (result f64) (local i32 f32 f32))",
        "(module (func $id (type 10) (param f32) (param f32) (result f64) (local i32) (local f32) (local f32)))"
    );
    assert_eq_rewrite(
        "(func (param f32 f32) (result f64) (local i32 f32 f32))",
        "(module (func (type <#:gensym>) (param f32) (param f32) (result f64) (local i32) (local f32) (local f32)))"
    );
}

#[test]
fn test_rewrite_func_normal5() {
    assert_eq_rewrite(
        "(func (param $pr i32))", 
        "(module (func (type <#:gensym>) (param $pr i32)))"
    );
    assert_eq_rewrite(
        "(func $id (local $lcl i32))", 
        "(module (func $id (type <#:gensym>) (local $lcl i32)))"
    );
    assert_eq_rewrite(
        "(func (param i32 i32) (result i32) (local $l1 i64) (local i64 f64))", 
        "(module (func (type <#:gensym>) (param i32) (param i32) (result i32) (local $l1 i64) (local i64) (local f64)))"
    );
    assert_eq_rewrite(
        "(func $id (type 0) (param $pr i32) (local f32 f32))", 
        "(module (func $id (type 0) (param $pr i32) (local f32) (local f32)))"
    );
}

impl<R> Rewriter<R> where R: Read + Seek {
    fn rewrite_instrs(&mut self) -> Result<(), RewriteError> {
        while let Ok(token) = self.lexer.next_token() {
            match token {
                tk!(TokenKind::Empty) => {
                    self.ast.push(token.clone());
                    break;
                },
                instr!(Instr::If(_, _, _)) => {
                    self.ast.push(token.clone());
                    self.rewrite_if()?;
                },
                tk!(TokenKind::LeftParen) => {
                    self.ast.push(token.clone());
                    self.rewrite_folded_instrs()?;
                },
                _ => {
                    self.ast.push(token.clone());
                },
            }
        }
        Ok(())
    }

    fn rewrite_folded_instrs(&mut self) -> Result<(), RewriteError> {
        // plaininstr
        // block
        // loop
        // if
        unimplemented!()
    }
}

impl<R> Rewriter<R> where R: Read + Seek {
    fn rewrite_if(&mut self) -> Result<(), RewriteError> {
        let mut else_exists = false;
        while let Ok(token) = self.lexer.next_token() {
            match token {
                tk!(TokenKind::Empty) => {
                    self.ast.push(token.clone());
                    break;
                },
                kw!(Keyword::End) => {
                    if else_exists {
                        self.ast.push(Token::keyword(Keyword::Else, Loc::zero()));
                    }
                    self.ast.push(token.clone());
                    break;
                },
                kw!(Keyword::Else) => {
                    self.ast.push(token.clone());
                    else_exists = true;
                },
                _ => {
                    self.ast.push(token.clone());
                },
            }
        }
        Ok(())
    }
}

#[test]
#[ignore]
fn test_rewrite_if() {
    assert_eq_rewrite("(func (type 0) i32.const 0 if nop else end)", "");
}
