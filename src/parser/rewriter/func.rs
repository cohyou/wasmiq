mod ifelse;
mod folded;


use super::*;


impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_func(&mut self, lparen_global: Token, global: Token) -> Result<(), RewriteError> {
        let mut header = vec![lparen_global, global];
        let maybe_id = self.lexer.next_token()?;
        let token1 = self.scan_id(maybe_id, &mut header)?;
        let token2 = self.lexer.next_token()?;

        self.rewrite_func_first(header, token1, token2, false)
    }

    fn rewrite_func_first(&mut self, header: Vec<Token>, token1: Token, token2: Token, exporting: bool) -> Result<(), RewriteError> {
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

                        let rparen = self.lexer.next_token()?;

                        let token1 = self.lexer.next_token()?;
                        let token2 = self.lexer.next_token()?;
                        self.rewrite_func_first(header, token1, token2, exporting)?;

                        self.ast.push(rparen);
                    },
                    export @ kw!(Keyword::Export) => {
                        self.ast.push(lparen);
                        self.ast.push(export);
                        let name = self.lexer.next_token()?;
                        self.ast.push(name);

                        self.push_header(header.clone(), true);

                        self.ast.push(Token::right_paren(Loc::zero()));
                        let rparen_func = self.lexer.next_token()?;
                        self.ast.push(rparen_func);

                        let token1 = self.lexer.next_token()?;
                        let token2 = self.lexer.next_token()?;
                        return self.rewrite_func_first(header, token1, token2, true);
                    },
                    tp @ kw!(Keyword::Type) => {
                        self.push_header(header.clone(), false);

                        self.scan_typeidx(lparen, tp)?;
                        let token1 = self.lexer.next_token()?;
                        let token2 = self.lexer.next_token()?;
                        return self.rewrite_func_rest(token1, token2);
                    },
                    param @ kw!(Keyword::Param) => {
                        self.rewrite_func_valtypes_first(header, lparen, param, exporting)?;
                    },
                    result @ kw!(Keyword::Result) => {
                        self.rewrite_func_valtypes_first(header, lparen, result, exporting)?;
                    },
                    local @ kw!(Keyword::Local) => {
                        self.rewrite_func_valtypes_first(header, lparen, local, exporting)?;
                    },
                    instr @ instr!(_) => {
                        self.push_header(header, exporting);
                        let typeidx = self.add_typeidx();
                        self.ast.extend(typeidx);
                        self.rewrite_instrs(vec![lparen, instr])?;
                    },
                    _ => self.push_others_first(header, lparen, token2),
                }
            },
            instr @ instr!(_) => {
                self.push_header(header, exporting);
                let typeidx = self.add_typeidx();
                self.ast.extend(typeidx);
                self.rewrite_instrs(vec![instr, token2])?;
            },
            rparen @ tk!(TokenKind::RightParen) => {
                self.push_header(header, exporting);
                let typeidx = self.add_typeidx();
                self.ast.extend(typeidx);
                self.ast.push(rparen);
            },
            _ => self.push_others_first(header, token1, token2),
        }

        Ok(())
    }

    fn rewrite_func_valtypes_first(&mut self, header: Vec<Token>, lparen: Token, valtype: Token, exporting: bool) -> Result<(), RewriteError> {
        self.push_header(header.clone(), exporting);
        let typeidx = self.add_typeidx();
        self.ast.extend(typeidx);
        self.rewrite_func_valtypes(lparen, valtype)
    }

    fn rewrite_func_rest(&mut self, token1: Token, token2: Token) -> Result<(), RewriteError> {

        match token1 {
            lparen @ tk!(TokenKind::LeftParen) => {
                match token2 {
                    param @ kw!(Keyword::Param) => {
                        self.rewrite_func_valtypes_rest(lparen, param)?;
                    },
                    result @ kw!(Keyword::Result) => {
                        self.rewrite_func_valtypes_rest(lparen, result)?;
                    },
                    local @ kw!(Keyword::Local) => {
                        self.rewrite_func_valtypes_rest(lparen, local)?;
                    },
                    instr @ instr!(_) => {
                        self.rewrite_instrs(vec![lparen, instr])?;
                    },
                    _ => self.push_others_rest(lparen, token2),
                }
            },
            instr @ instr!(_) => {
                self.rewrite_instrs(vec![instr, token2])?;
            },
            _ => self.push_others_rest(token1, token2),
        }

        Ok(())
    }

    fn rewrite_func_valtypes_rest(&mut self, lparen: Token, valtype: Token) -> Result<(), RewriteError> {
        self.rewrite_func_valtypes(lparen, valtype)
    }

    fn rewrite_func_valtypes(&mut self, lparen: Token, valtype: Token) -> Result<(), RewriteError> {
        self.ast.push(lparen);
        self.ast.push(valtype.clone());
        if let kw!(keyword) = valtype {
            let holding = self.rewrite_valtypes(keyword)?;
            self.ast.extend(holding);
        }
        
        let token1 = self.lexer.next_token()?;
        let token2 = self.lexer.next_token()?;
        self.rewrite_func_rest(token1, token2)
    }

    fn push_header(&mut self, header: Vec<Token>, exporting: bool) {
        for t in header.clone() { self.ast.push(t); }
        if exporting && header.len() == 2 {
            self.ast.push(Token::gensym(Loc::zero()))
        }
    }

    fn push_others_first(&mut self, header: Vec<Token>, token1: Token, token2: Token) {
        for t in header { self.ast.push(t); }
        self.push_others(token1, token2);
    }

    fn push_others_rest(&mut self, token1: Token, token2: Token) {
        self.push_others(token1, token2);
    }

    fn push_others(&mut self, token1: Token, token2: Token) {
        self.ast.push(token1);
        self.ast.push(token2);
    }
}

impl<R> Rewriter<R> where R: Read + Seek {
    fn rewrite_blocktype_if_first(&mut self, token: Token) -> Result<(Vec<Token>, Vec<Token>), RewriteError> {
        let mut holding = vec![];
        let token = match token {
            lparen @ tk!(TokenKind::LeftParen) => {

                match self.lexer.next_token()? {
                    result @ kw!(Keyword::Result) => {
                        holding.push(lparen);
                        holding.push(result);

                        let holding_result = self.rewrite_valtypes(Keyword::Result)?;
                        holding.extend(holding_result);
                    },
                    tp @ kw!(Keyword::Type) => {
                        let holding_typeidx = self.scan_typeidx_holding(lparen.clone(), tp)?;
                        holding.extend(holding_typeidx);

                        let token1 = self.lexer.next_token()?;
                        let (holding_blocktype, token2) = self.rewrite_blocktype_if_rest(token1)?;
                        holding.extend(holding_blocktype);
                        
                        return Ok((holding, vec![token2.clone()]));
                    },
                    param @ kw!(Keyword::Param) => {
                        let holding_typeidx = self.add_typeidx();
                        holding.extend(holding_typeidx);

                        holding.push(lparen.clone());
                        holding.push(param);

                        let holding_param = self.rewrite_valtypes(Keyword::Param)?;
                        holding.extend(holding_param);

                        let token1 = self.lexer.next_token()?;
                        let (holding_blocktype, token2) = self.rewrite_blocktype_if_rest(token1)?;
                        holding.extend(holding_blocktype);

                        return Ok((holding, vec![token2]));
                    },
                    instr @ instr!(_) => return Ok((holding, vec![lparen, instr])),
                    t @ _ => return Ok((holding, vec![lparen, t])),
                }
                self.lexer.next_token()?
            },
            _ => token,
        };

        Ok((holding, vec![token]))
    }

    fn rewrite_blocktype_if_rest(&mut self, token: Token) -> Result<(Vec<Token>, Token), RewriteError> {
        let mut holding = vec![];

        let token = match token {
            lparen @ tk!(TokenKind::LeftParen) => {
                
                let token = match self.lexer.next_token()? {
                    param @ kw!(Keyword::Param) => {
                        holding.push(lparen.clone());
                        holding.push(param);
                        let holding_param = self.rewrite_valtypes(Keyword::Param)?;
                        holding.extend(holding_param);

                        let token1 = self.lexer.next_token()?;
                        let (holding_blocktype, token2) = self.rewrite_blocktype_if_rest(token1)?;
                        holding.extend(holding_blocktype);

                        return Ok((holding, token2));
                    },
                    t @ _ => t,
                };

                let token = match token {
                    result @ kw!(Keyword::Result) => {
                        holding.push(lparen);
                        holding.push(result);
                        let holding_result = self.rewrite_valtypes(Keyword::Result)?;
                        holding.extend(holding_result);
                        self.lexer.next_token()?
                    },
                    _ => token,
                };
                token
            },
            _ => token,
        };

        Ok((holding, token))
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

#[test]
fn test_rewrite_func_import() {
    assert_eq_rewrite(
        r#"(func (import "n1" "n2"))"#, 
        r#"(module (import "n1" "n2" (func (type <#:gensym>))))"#
    );
    assert_eq_rewrite(
        r#"(func (import "n1" "n2") (type 0))"#,
        r#"(module (import "n1" "n2" (func (type 0))))"#
    );
    assert_eq_rewrite(
        r#"(func $id (import "n1" "n2") (param i32 i64))"#, 
        r#"(module (import "n1" "n2" (func $id (type <#:gensym>) (param i32) (param i64))))"#
    );
    assert_eq_rewrite(
        r#"(func $id (import "n1" "n2") (param i32 i32) (result i32))"#, 
        r#"(module (import "n1" "n2" (func $id (type <#:gensym>) (param i32) (param i32) (result i32))))"#
    );
}

#[test]
fn test_rewrite_func_export() {
    assert_eq_rewrite(
        r#"(func (export "n1"))"#, 
        r#"(module (export "n1" (func <#:gensym>)) (func <#:gensym> (type <#:gensym>)))"#
    );
    assert_eq_rewrite(
        r#"(func $id (export "e2") (type 1) nop)"#, 
        r#"(module (export "e2" (func $id)) (func $id (type 1) nop))"#
    );
    assert_eq_rewrite(
        r#"(func (export "e3") (export "e4") (param $p1 i64) (param i32 i32) (result i32))"#, 
        r#"(module (export "e3" (func <#:gensym>)) (export "e4" (func <#:gensym>)) (func <#:gensym> (type <#:gensym>) (param $p1 i64) (param i32) (param i32) (result i32)))"#
    );
    assert_eq_rewrite(
        r#"(func $id (export "e5") (export "e6") (type 256) (result f64))"#, 
        r#"(module (export "e5" (func $id)) (export "e6" (func $id)) (func $id (type 256) (result f64)))"#
    );
}

#[test]
fn test_rewrite_global_import_export() {
    assert_eq_rewrite(
        r#"(func (export "e3") (import "n1" "n2") (type 1))"#, 
        r#"(module (export "e3" (func <#:gensym>)) (import "n1" "n2" (func (type 1))))"#
    );
    assert_eq_rewrite(
        r#"(func $id (export "e3") (import "n1" "n2") (param $p i32) (result i32))"#, 
        r#"(module (export "e3" (func $id)) (import "n1" "n2" (func $id (type <#:gensym>) (param $p i32) (result i32))))"#
    );
}