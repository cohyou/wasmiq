mod ifelse;
mod folded;


use super::*;


impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_func(&mut self, lparen_func: Token, func: Token) -> Result<(), RewriteError> {
        let mut header = vec![lparen_func, func];
        let maybe_id = self.lexer.next_token()?;

        let named_id = if let tk!(TokenKind::Id(s)) = maybe_id.clone() {
            Some(Id::Named(s))
        } else {
            None
        };

        let token1 = self.scan_id(maybe_id, &mut header)?;
        let token2 = self.lexer.next_token()?;

        if let Some(_) = named_id {
            self.context.funcs.push(named_id);
        } else {
            self.set_context_id_func(&token1, &token2);
        }

        self.rewrite_func_first(header, token1, token2, false)
    }

    fn set_context_id_func(&mut self, token1: &Token, token2: &Token) {
        if let tk!(TokenKind::LeftParen) = token1 {
            if let kw!(Keyword::Export) = token2 {
                let new_gensym_index = self.next_symbol_index;
                self.context.funcs.push(Some(Id::Anonymous(new_gensym_index)));
            } else {
                self.context.funcs.push(None);
            }
        } else {
            match token2 {
                instr!(_) => self.context.funcs.push(None),
                tk!(TokenKind::RightParen) => self.context.funcs.push(None),
                _ => {},
            }
        }
    }

    fn rewrite_func_first(&mut self, header: Vec<Token>, token1: Token, token2: Token, exporting: bool) -> Result<(), RewriteError> {
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

                        let rparen = self.lexer.next_token()?;

                        let token1 = self.lexer.next_token()?;
                        let token2 = self.lexer.next_token()?;
                        self.rewrite_func_first_import(header, token1, token2, exporting)?;

                        self.imports.push(rparen);
                    },
                    export @ kw!(Keyword::Export) => {
                        self.exports.push(lparen);
                        self.exports.push(export);
                        let name = self.lexer.next_token()?;
                        self.exports.push(name);

                        self.push_header_export(header.clone(), exporting);

                        self.exports.push(Token::right_paren(Loc::zero()));
                        let rparen_func = self.lexer.next_token()?;
                        self.exports.push(rparen_func);

                        let token1 = self.lexer.next_token()?;
                        let token2 = self.lexer.next_token()?;
                        return self.rewrite_func_first(header, token1, token2, true);
                    },
                    tp @ kw!(Keyword::Type) => {
                        self.push_header(header.clone(), exporting);

                        let holding = self.scan_typeidx_holding(lparen, tp)?;
                        self.funcs.extend(holding);
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
                        self.funcs.extend(typeidx);
                        let instrs = self.rewrite_instrs(vec![lparen, instr])?;
                        self.funcs.extend(instrs);
                    },
                    tk!(TokenKind::Empty) => {},
                    _ => self.push_others_first(header, lparen, token2),
                }
            },
            instr @ instr!(_) => {
                self.push_header(header, exporting);
                let typeidx = self.add_typeidx();
                self.funcs.extend(typeidx);
                let instrs = self.rewrite_instrs(vec![instr, token2])?;
                self.funcs.extend(instrs);
            },
            rparen @ tk!(TokenKind::RightParen) => {
                self.push_header(header, exporting);
                let typeidx = self.add_typeidx();
                self.funcs.extend(typeidx);
                self.funcs.push(rparen);
            },
            tk!(TokenKind::Empty) => {},
            _ => self.push_others_first(header, token1, token2),
        }

        Ok(())
    }

    fn rewrite_func_first_import(&mut self, header: Vec<Token>, token1: Token, token2: Token, exporting: bool) -> Result<(), RewriteError> {
        match token1 {
            lparen @ tk!(TokenKind::LeftParen) => {
                match token2 {
                    tp @ kw!(Keyword::Type) => {
                        self.push_header_import(header.clone(), exporting);

                        let holding = self.scan_typeidx_holding(lparen, tp)?;
                        self.imports.extend(holding);
                        let token1 = self.lexer.next_token()?;
                        let token2 = self.lexer.next_token()?;
                        self.rewrite_func_rest_import(token1, token2)?;
                    },
                    param @ kw!(Keyword::Param) => {
                        self.rewrite_func_valtypes_first_import(header, lparen, param, exporting)?;
                    },
                    result @ kw!(Keyword::Result) => {
                        self.rewrite_func_valtypes_first_import(header, lparen, result, exporting)?;
                    },
                    _ => self.push_others_first_import(header, lparen, token2),
                }
            },
            rparen @ tk!(TokenKind::RightParen) => {
                self.push_header_import(header, exporting);
                let typeidx = self.add_typeidx();
                self.imports.extend(typeidx);
                self.imports.push(rparen);
            },
            _ => self.push_others_first_import(header, token1, token2),
        }

        Ok(())
    }

    fn rewrite_func_valtypes_first(&mut self, header: Vec<Token>, lparen: Token, valtype: Token, exporting: bool) -> Result<(), RewriteError> {
        self.push_header(header.clone(), exporting);
        let typeidx = self.add_typeidx();
        self.funcs.extend(typeidx);
        self.rewrite_func_valtypes(lparen, valtype)
    }

    fn rewrite_func_valtypes_first_import(&mut self, header: Vec<Token>, lparen: Token, valtype: Token, exporting: bool) -> Result<(), RewriteError> {
        self.push_header_import(header.clone(), exporting);
        let typeidx = self.add_typeidx();
        self.imports.extend(typeidx);
        self.rewrite_func_valtypes_import(lparen, valtype)
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
                        let instrs = self.rewrite_instrs(vec![lparen, instr])?;
                        self.funcs.extend(instrs);
                    },
                    tk!(TokenKind::Empty) => {},
                    _ => self.push_others_rest(lparen, token2),
                }
            },
            instr @ instr!(_) => {
                let instrs = self.rewrite_instrs(vec![instr, token2])?;
                self.funcs.extend(instrs);
            },
            rparen @ tk!(TokenKind::RightParen) => {
                self.funcs.push(rparen);
                self.precedings.push_back(token2);
            },
            _ => self.push_others_rest(token1, token2),
        }

        Ok(())
    }

    fn rewrite_func_rest_import(&mut self, token1: Token, token2: Token) -> Result<(), RewriteError> {

        match token1 {
            lparen @ tk!(TokenKind::LeftParen) => {
                match token2 {
                    param @ kw!(Keyword::Param) => {
                        self.rewrite_func_valtypes_rest_import(lparen, param)?;
                    },
                    result @ kw!(Keyword::Result) => {
                        self.rewrite_func_valtypes_rest_import(lparen, result)?;
                    },
                    _ => self.push_others_rest_import(lparen, token2),
                }
            },
            rparen @ tk!(TokenKind::RightParen) => {
                self.imports.push(rparen);
                self.precedings.push_back(token2);
            },
            _ => self.push_others_rest_import(token1, token2),
        }

        Ok(())
    }

    fn rewrite_func_valtypes_rest(&mut self, lparen: Token, valtype: Token) -> Result<(), RewriteError> {
        self.rewrite_func_valtypes(lparen, valtype)
    }

    fn rewrite_func_valtypes_rest_import(&mut self, lparen: Token, valtype: Token) -> Result<(), RewriteError> {
        self.rewrite_func_valtypes_import(lparen, valtype)
    }

    fn rewrite_func_valtypes(&mut self, lparen: Token, valtype: Token) -> Result<(), RewriteError> {
        self.funcs.push(lparen);
        self.funcs.push(valtype.clone());
        if let kw!(keyword) = valtype {
            let holding = self.rewrite_valtypes(keyword)?;
            self.funcs.extend(holding);
        }
        
        let token1 = self.lexer.next_token()?;
        let token2 = self.lexer.next_token()?;
        self.rewrite_func_rest(token1, token2)
    }

    fn rewrite_func_valtypes_import(&mut self, lparen: Token, valtype: Token) -> Result<(), RewriteError> {
        self.imports.push(lparen);
        self.imports.push(valtype.clone());
        if let kw!(keyword) = valtype {
            let holding = self.rewrite_valtypes(keyword)?;
            self.imports.extend(holding);
        }
        
        let token1 = self.lexer.next_token()?;
        let token2 = self.lexer.next_token()?;
        self.rewrite_func_rest_import(token1, token2)
    }

    fn push_header(&mut self, header: Vec<Token>, exporting: bool) {
        for t in header.clone() { self.funcs.push(t); }
        if header.len() == 2 {
            if exporting {
                self.funcs.push(Token::gensym(self.next_symbol_index - 1, Loc::zero()));
            }   
        }
    }

    fn push_header_import(&mut self, header: Vec<Token>, exporting: bool) {
        for t in header.clone() { self.imports.push(t); }
        if exporting && header.len() == 2 {
            self.imports.push(Token::gensym(self.next_symbol_index - 1, Loc::zero()));
        }
    }

    fn push_header_export(&mut self, header: Vec<Token>, exporting: bool) {
        for t in header.clone() { self.exports.push(t); }
        if header.len() == 2 {
            if exporting {
                self.exports.push(Token::gensym(self.next_symbol_index - 1, Loc::zero()));
            } else {
                let gensym = self.make_gensym();
                self.exports.push(gensym);
            }
        }
    }

    fn push_others_first(&mut self, header: Vec<Token>, token1: Token, token2: Token) {
        for t in header { self.funcs.push(t); }
        self.push_others(token1, token2);
    }

    fn push_others_first_import(&mut self, header: Vec<Token>, token1: Token, token2: Token) {
        for t in header { self.imports.push(t); }
        self.push_others_import(token1, token2);
    }

    fn push_others_rest(&mut self, token1: Token, token2: Token) {
        self.push_others(token1, token2);
    }

    fn push_others_rest_import(&mut self, token1: Token, token2: Token) {
        self.push_others_import(token1, token2);
    }

    fn push_others(&mut self, token1: Token, token2: Token) {
        self.funcs.push(token1);
        self.funcs.push(token2);
    }

    fn push_others_import(&mut self, token1: Token, token2: Token) {
        self.imports.push(token1);
        self.imports.push(token2);
    }
}

impl<R> Rewriter<R> where R: Read + Seek {
    fn rewrite_blocktype_first(&mut self, token: Token) -> Result<(Vec<Token>, Vec<Token>), RewriteError> {
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
                        let (holding_blocktype, token2) = self.rewrite_blocktype_rest(token1)?;
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
                        let (holding_blocktype, token2) = self.rewrite_blocktype_rest(token1)?;
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

    fn rewrite_blocktype_rest(&mut self, token: Token) -> Result<(Vec<Token>, Token), RewriteError> {
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
                        let (holding_blocktype, token2) = self.rewrite_blocktype_rest(token1)?;
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
    assert_eq_rewrite("(func)", "(module (func (type <#:gensym(0)>)))");
    assert_eq_rewrite("(func nop)", "(module (func (type <#:gensym(0)>) nop))");
    assert_eq_rewrite("(func nop unreachable)", "(module (func (type <#:gensym(0)>) nop unreachable))");
    assert_eq_rewrite("(func $id)", "(module (func $id (type <#:gensym(0)>)))");
    assert_eq_rewrite("(func $id nop)", "(module (func $id (type <#:gensym(0)>) nop))");
    assert_eq_rewrite("(func $id nop unreachable)", "(module (func $id (type <#:gensym(0)>) nop unreachable))");
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
    assert_eq_rewrite("(func (param i32))", "(module (func (type <#:gensym(0)>) (param i32)))");
    assert_eq_rewrite("(func $id (param i32 f64))", "(module (func $id (type <#:gensym(0)>) (param i32) (param f64)))");
    assert_eq_rewrite("(func (type 0) (param i32))", "(module (func (type 0) (param i32)))");
    assert_eq_rewrite("(func (result i64))", "(module (func (type <#:gensym(0)>) (result i64)))");
    assert_eq_rewrite("(func $id (type 0) (result i64 f32) i64.const 100)", "(module (func $id (type 0) (result i64) (result f32) i64.const 100))");
    assert_eq_rewrite("(func (local f64 i32))", "(module (func (type <#:gensym(0)>) (local f64) (local i32)))");
    assert_eq_rewrite("(func $id (local f64))", "(module (func $id (type <#:gensym(0)>) (local f64)))");
    assert_eq_rewrite("(func (type 0) (local f64 i32) nop nop)", "(module (func (type 0) (local f64) (local i32) nop nop))");
}

#[test]
fn test_rewrite_func_normal4() {
    assert_eq_rewrite(
        "(func (param i32) (result f32))", 
        "(module (func (type <#:gensym(0)>) (param i32) (result f32)))"
    );
    assert_eq_rewrite(
        "(func (type 1) (param i32 i32) (result f32))", 
        "(module (func (type 1) (param i32) (param i32) (result f32)))"
    );
    assert_eq_rewrite(
        "(func $id (param i32 i32) (local i64 i64))", 
        "(module (func $id (type <#:gensym(0)>) (param i32) (param i32) (local i64) (local i64)))"
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
        "(module (func (type <#:gensym(0)>) (param f32) (param f32) (result f64) (local i32) (local f32) (local f32)))"
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
        "(module (func (type <#:gensym(0)>) (param $pr i32)))"
    );
    assert_eq_rewrite(
        "(func $id (local $lcl i32))", 
        "(module (func $id (type <#:gensym(0)>) (local $lcl i32)))"
    );
    assert_eq_rewrite(
        "(func $id (local $lcl i32) nop)", 
        "(module (func $id (type <#:gensym(0)>) (local $lcl i32) nop))"
    );
    assert_eq_rewrite(
        "(func (param i32 i32) (result i32) (local $l1 i64) (local i64 f64))", 
        "(module (func (type <#:gensym(0)>) (param i32) (param i32) (result i32) (local $l1 i64) (local i64) (local f64)))"
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
        r#"(module (import "n1" "n2" (func (type <#:gensym(0)>))))"#
    );
    assert_eq_rewrite(
        r#"(func (import "n1" "n2") (type 0))"#,
        r#"(module (import "n1" "n2" (func (type 0))))"#
    );
    assert_eq_rewrite(
        r#"(func $id (import "n1" "n2") (param i32 i64))"#, 
        r#"(module (import "n1" "n2" (func $id (type <#:gensym(0)>) (param i32) (param i64))))"#
    );
    assert_eq_rewrite(
        r#"(func $id (import "n1" "n2") (param i32 i32) (result i32))"#, 
        r#"(module (import "n1" "n2" (func $id (type <#:gensym(0)>) (param i32) (param i32) (result i32))))"#
    );
}

#[test]
fn test_rewrite_func_export() {
    assert_eq_rewrite(
        r#"(func (export "n1"))"#, 
        r#"(module (func <#:gensym(0)> (type <#:gensym(1)>)) (export "n1" (func <#:gensym(0)>)))"#
    );
    assert_eq_rewrite(
        r#"(func (export "main") nop)"#,
        r#"(module (func <#:gensym(0)> (type <#:gensym(1)>) nop) (export "main" (func <#:gensym(0)>)))"#
    );
    assert_eq_rewrite(
        r#"(func $id (export "e2") (type 1) nop)"#, 
        r#"(module (func $id (type 1) nop) (export "e2" (func $id)))"#
    );
    assert_eq_rewrite(
        r#"(func (export "e3") (export "e4") (param $p1 i64) (param i32 i32) (result f64))"#, 
        r#"(module (func <#:gensym(0)> (type <#:gensym(1)>) (param $p1 i64) (param i32) (param i32) (result f64)) (export "e3" (func <#:gensym(0)>)) (export "e4" (func <#:gensym(0)>)))"#
    );
    assert_eq_rewrite(
        r#"(func $id (export "e5") (export "e6") (type 256) (result f64))"#, 
        r#"(module (func $id (type 256) (result f64)) (export "e5" (func $id)) (export "e6" (func $id)))"#
    );
}

#[test]
fn test_rewrite_func_import_export() {
    assert_eq_rewrite(
        r#"(func (export "e3") (import "n1" "n2") (type 1))"#, 
        r#"(module (import "n1" "n2" (func <#:gensym(0)> (type 1))) (export "e3" (func <#:gensym(0)>)))"#
    );
    assert_eq_rewrite(
        r#"(func $id (export "e4") (import "n3" "n4") (param $p i32) (result i32))"#, 
        r#"(module (import "n3" "n4" (func $id (type <#:gensym(0)>) (param $p i32) (result i32))) (export "e4" (func $id)))"#
    );
}

#[test]
fn test_rewrite_func_export2() {
    assert_eq_rewrite(
        r#"(func (export "nop") (type 1))"#, 
        r#"(module (func <#:gensym(0)> (type 1)) (export "nop" (func <#:gensym(0)>)))"#
    );
}

