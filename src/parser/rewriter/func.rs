use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_func(&mut self, token_func: Token) -> Result<(), RewriteError> {
        let mut tokens = vec![token_func];
        let token = self.lexer.next_token()?;
        let token1 = self.scan_id(token, &mut tokens)?;
        let token2 = self.lexer.next_token()?;

        if Rewriter::<R>::is_import_or_export(&token1, &token2) {
            let token_leftparen = token1.clone();
            self.rewrite_inline_export_import(tokens, token_leftparen.clone(), token2.clone())?;
            let token_rightparen = self.lexer.next_token()?;
            if Rewriter::<R>::is_for_typeuse(&token2) {
                self.rewrite_typeuse(token_leftparen, token2)?;
            }
            self.ast.push(token_rightparen);
        } else {
            for t in &tokens { self.ast.push(t.clone()); }
            let (token1, token2) =
            if Rewriter::<R>::is_type_list(&token1, &token2) {
                self.scan_typeidx(token1.clone(), token2.clone())?;
                let token1 = self.lexer.next_token()?;
                let token2 = self.lexer.next_token()?;

                (token1, token2)
            } else {
                self.add_typeidx();
                (token1, token2)
            };

            if Rewriter::<R>::is_for_typeuse(&token2) {
                let (token1, token2) = self.rewrite_typeuse(token1.clone(), token2.clone())?;
                self.rewrite_func_body(token1, token2)?;
            } else {
                self.rewrite_func_body(token1, token2)?;
            }
        }

        Ok(())
    }

    fn scan_typeidx(&mut self, token1: Token, token2: Token) -> Result<(), RewriteError> {
        self.ast.push(token1);
        self.ast.push(token2.clone());
        let token_typeidx = self.lexer.next_token()?;
        self.ast.push(token_typeidx);
        let token_rightparen = self.lexer.next_token()?;
        self.ast.push(token_rightparen);
        Ok(())
    }

    fn rewrite_typeuse(&mut self, token_leftparen: Token, token_keyword: Token) -> Result<(Token, Token), RewriteError> {

        match &token_keyword {
            token_param @ kw!(Keyword::Param) => {                

                self.ast.push(token_leftparen);
                self.ast.push(token_param.clone());
                self.rewrite_param()?;

                let token = self.lexer.next_token()?;
                match token {
                    token_leftparen @ tk!(TokenKind::LeftParen) => {
                        
                        let token = self.lexer.next_token()?;
                        match &token {
                            token_result @ kw!(Keyword::Result) => {
                                self.ast.push(token_leftparen);
                                self.ast.push(token_result.clone());
                                self.rewrite_result()?;

                                let token1 = self.lexer.next_token()?;
                                let token2 = self.lexer.next_token()?;
                                return Ok((token1, token2));
                            },
                            _ => {
                                return Ok((token_leftparen.clone(), token.clone()));
                            },
                        }
                    },
                    _ => {
                        self.ast.push(token.clone());
                        let token1 = self.lexer.next_token()?;
                        let token2 = self.lexer.next_token()?;
                        return Ok((token1, token2));
                    },
                }
            },
            token_result @ kw!(Keyword::Result) => {

                self.ast.push(token_leftparen);
                self.ast.push(token_result.clone());
                self.rewrite_result()?;
                let token1 = self.lexer.next_token()?;
                let token2 = self.lexer.next_token()?;
                return Ok((token1, token2));
            },
            _ => {
                return Ok((token_leftparen.clone(), token_keyword.clone()));
            },
        }
    }

    fn rewrite_func_body(&mut self, token1: Token, token2: Token) -> Result<(), RewriteError> {
        match &token1 {
            tk!(TokenKind::LeftParen) => {
                match &token2 {
                    token_local @ kw!(Keyword::Local) => {
                        self.ast.push(token1);
                        self.ast.push(token_local.clone());
                        self.rewrite_local()?;
                    },
                    token_instr @ instr!(_) => {
                        self.rewrite_instrs(vec![token1.clone(), token_instr.clone()])?;
                    },
                    _ => {},
                }
            },
            token_instr @ instr!(_) => {
                self.rewrite_instrs(vec![token_instr.clone(), token2.clone()])?;
            },
            tk!(TokenKind::RightParen) => {
                self.ast.push(token1.clone());
                self.ast.push(token2.clone());
                return Ok(());
            },
            _ => {},
        }

        Ok(())
    }

    fn is_for_typeuse(token: &Token) -> bool {
        token.value == TokenKind::Keyword(Keyword::Type) ||
        token.value == TokenKind::Keyword(Keyword::Param) ||
        token.value == TokenKind::Keyword(Keyword::Result)
    }

    fn is_type_list(token1: &Token, token2: &Token) -> bool {
        token1.value == TokenKind::LeftParen &&
        token2.value == TokenKind::Keyword(Keyword::Type)
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
    assert_eq_rewrite("(func nop unreachable)", "(module (func (type <#:gensym>) nop unreachable))");
    assert_eq_rewrite("(func $id)", "(module (func $id (type <#:gensym>)))");
    assert_eq_rewrite("(func $id nop)", "(module (func $id (type <#:gensym>) nop))");
    assert_eq_rewrite("(func $id nop unreachable)", "(module (func $id (type <#:gensym>) nop unreachable))");
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
        "(func (type 0) (param $pr i32))",
        "(module (func (type 0) (param $pr i32)))"
    );
    assert_eq_rewrite(
        "(func (param $pr i32))",
        "(module (func (type <#:gensym>) (param $pr i32)))"
    );
    assert_eq_rewrite(
        "(func $id (local $lcl i32))", 
        "(module (func $id (type <#:gensym>) (local $lcl i32)))"
    );
    assert_eq_rewrite(
        "(func $id (local $lcl i32) nop)", 
        "(module (func $id (type <#:gensym>) (local $lcl i32) nop))"
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

