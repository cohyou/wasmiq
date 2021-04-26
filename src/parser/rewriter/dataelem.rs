use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_elem(&mut self, lparen_elem: Token, elem: Token) -> Result<(), RewriteError> {
        self.elem.push(lparen_elem);
        self.elem.push(elem);

        let token = self.lexer.next_token()?;
        
        // tableidx
        let (tokens, token) = self.scan_idx(token)?;
        self.elem.extend(tokens);

        // offset
        let (tokens, token) = self.rewrite_offset(token)?;
        self.elem.extend(tokens);

        // vec(funcidx)
        let func_indices = self.scan_vec(token)?;
        self.elem.extend(func_indices);

        Ok(())
    }

    pub fn rewrite_data(&mut self, lparen_data: Token, data: Token) -> Result<(), RewriteError> {
        self.data.push(lparen_data);
        self.data.push(data);

        let token = self.lexer.next_token()?;
        
        // memidx
        let (tokens, token) = self.scan_idx(token)?;
        self.data.extend(tokens);

        // offset
        let (tokens, token) = self.rewrite_offset(token)?;
        self.data.extend(tokens);

        // vec(funcidx)
        let func_indices = self.scan_vec(token)?;
        self.data.extend(func_indices);

        Ok(())
    }

    fn scan_idx(&mut self, token: Token) -> Result<(Vec<Token>, Token), RewriteError> {
        let mut tokens = vec![];

        let token = match token {
            num @ tk!(TokenKind::Number(_)) => {
                tokens.push(num);
                self.lexer.next_token()?
            },
            id @ tk!(TokenKind::Id(_)) => {
                tokens.push(id);
                self.lexer.next_token()?
            },
            lparen @ tk!(TokenKind::LeftParen) => {
                tokens.push(Token::number_u(0, Loc::zero()));
                lparen
            },
            t @ _ => t,
        };

        Ok((tokens, token))
    }

    fn rewrite_offset(&mut self, token: Token) -> Result<(Vec<Token>, Token), RewriteError> {
        let mut tokens = vec![];

        let token = match token {
            lparen @ tk!(TokenKind::LeftParen) => {
                match self.lexer.next_token()? {
                    offset @ kw!(Keyword::Offset) => {
                        tokens.push(lparen);
                        tokens.push(offset);
                        let tokens_instr = self.rewrite_instrs(vec![])?;
                        tokens.extend(tokens_instr);
                        self.lexer.next_token()?
                    },
                    instr @ instr!(_) => {
                        tokens.push(lparen);
                        tokens.push(Token::keyword(Keyword::Offset, Loc::zero()));
                        let tokens_instr = self.rewrite_instr_for_offset(instr)?;
                        tokens.extend(tokens_instr);
                        let rparen = self.lexer.next_token()?;
                        tokens.push(rparen);
                        self.lexer.next_token()?
                    },
                    t @ _ => t,
                }
            },
            instr @ instr!(_) => {
                tokens.push(Token::left_paren(Loc::zero()));
                tokens.push(Token::keyword(Keyword::Offset, Loc::zero()));
                let tokens_instr = self.rewrite_instr_for_offset(instr)?;
                tokens.extend(tokens_instr);
                tokens.push(Token::right_paren(Loc::zero()));
                self.lexer.next_token()?
            },
            t @ _ => t,
        };

        Ok((tokens, token))
    }
}

#[test]
fn test_rewrite_elem() {
    assert_eq_rewrite(
        "(elem 0 (offset f32.const 12.3 i32.const 0) 2 3)",
        "(module (elem 0 (offset f32.const 12.3 i32.const 0) 2 3))"
    );
    assert_eq_rewrite(
        "(elem 0 i64.const 867 6 78)",
        "(module (elem 0 (offset i64.const 867) 6 78))"
    );
    assert_eq_rewrite(
        "(elem $id i32.const 64 200 42784)",
        "(module (elem $id (offset i32.const 64) 200 42784))"
    );
    assert_eq_rewrite(
        "(elem (offset f64.const 3.14) 32 44 33)",
        "(module (elem 0 (offset f64.const 3.14) 32 44 33))"
    );
    assert_eq_rewrite(
        "(elem (i32.const 0) $f)",
        "(module (elem 0 (offset i32.const 0) $f))"
    );
}

#[test]
fn test_rewrite_data() {
    assert_eq_rewrite(
        r#"(data 0 (offset i32.const 0 i64.const 1) "test" "string")"#, 
        r#"(module (data 0 (offset i32.const 0 i64.const 1) "test" "string"))"#
    );
    assert_eq_rewrite(
        r#"(data 0 i32.const 0 "test")"#, 
        r#"(module (data 0 (offset i32.const 0) "test"))"#
    );
    assert_eq_rewrite(
        r#"(data (offset f64.const 1.0) "string")"#,
        r#"(module (data 0 (offset f64.const 1) "string"))"#
    );
    assert_eq_rewrite(
        r#"(data (i32.const 0) "データ")"#,
        r#"(module (data 0 (offset i32.const 0) "データ"))"#
    );
}

impl<R> Rewriter<R> where R: Read + Seek {
    fn scan_vec(&mut self, token: Token) -> Result<Vec<Token>, RewriteError> {
        let mut result = vec![];
        let mut current = token;
        loop {
            match current {
                rparen @ tk!(TokenKind::RightParen) => {
                    result.push(rparen);
                    break;
                },
                empty @ tk!(TokenKind::Empty) => {
                    result.push(empty);
                    break;
                },
                t @ _ => result.push(t),
            }
            current = if let Ok(token) = self.lexer.next_token() {
                token
            } else {
                break;
            }
        }

        Ok(result)
    }

    fn rewrite_instr_for_offset(&mut self, token: Token) -> Result<Vec<Token>, RewriteError> {
        let mut result = vec![];

        match &token {
            instr!(instr) => {
                match instr {
                    Instr::Block(_, _) |
                    Instr::Loop(_, _) => {
                        unimplemented!();
                    },

                    Instr::If(_, _, _) => {
                        unimplemented!();
                    },

                    Instr::BrTable(_, _) => {
                        unimplemented!();
                    },

                    Instr::Br(_) |
                    Instr::BrIf(_) |
                    Instr::Call(_) |
                    Instr::CallIndirect(_) |
                    Instr::LocalGet(_) |
                    Instr::LocalSet(_) |
                    Instr::LocalTee(_) |
                    Instr::GlobalGet(_) |
                    Instr::GlobalSet(_) |
                    Instr::I32Const(_) |
                    Instr::I64Const(_) |
                    Instr::F32Const(_) |
                    Instr::F64Const(_) => {
                        let token2 = self.lexer.next_token()?;
                        result.push(token.clone());
                        result.push(token2.clone());
                    },

                    Instr::Load(_, _) |
                    Instr::ILoad8(_, _, _) |
                    Instr::ILoad16(_, _, _) |
                    Instr::I64Load32(_, _) |
                    Instr::Store(_, _) |
                    Instr::IStore8(_, _) |
                    Instr::IStore16(_, _) |
                    Instr::I64Store32(_) => {
                        result.push(token.clone());
                        let token2 = self.lexer.next_token()?;
                        match token2 {
                            kw!(Keyword::MemArgOffset(_)) => {
                                result.push(token2.clone());
                                let token3 = self.lexer.next_token()?;
                                match token3 {
                                    kw!(Keyword::MemArgAlign(_)) => {
                                        result.push(token3.clone());
                                        let token4 = self.lexer.next_token()?;
                                        result.push(token4.clone());
                                    }
                                    _ => result.push(token3.clone()),
                                }
                            },
                            kw!(Keyword::MemArgAlign(_)) => {
                                result.push(token2.clone());
                                let token3 = self.lexer.next_token()?;
                                result.push(token3.clone());
                            },
                            _ => result.push(token2.clone()),
                        }
                    },

                    Instr::Unreachable |
                    Instr::Nop |
                    Instr::Return |
                    Instr::Drop(_) |
                    Instr::Select(_) |
                    Instr::MemorySize |
                    Instr::MemoryGrow => {
                        result.push(token.clone());
                    },
                    _ => { result.push(token.clone()); }
                }
            },
            _ => result.push(token.clone()),
        }

        Ok(result)
    }
}
