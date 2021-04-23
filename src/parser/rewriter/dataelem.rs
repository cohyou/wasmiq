use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_elem(&mut self, lparen_elem: Token, elem: Token) -> Result<(), RewriteError> {
        self.elem.push(lparen_elem);
        self.elem.push(elem);

        let token = self.lexer.next_token()?;
        
        match token {
            num @ tk!(TokenKind::Number(_)) => {
                self.elem.push(num);
                let offset = self.scan_offset()?;
                self.elem.extend(offset);
            },
            id @ tk!(TokenKind::Id(_)) => {
                self.elem.push(id);
                let offset = self.scan_offset()?;
                self.elem.extend(offset);
            },
            lparen @ tk!(TokenKind::LeftParen) => {
                self.elem.push(Token::number_u(0, Loc::zero()));
                self.elem.push(lparen);
                let offset = self.scan_offset()?;
                self.elem.extend(offset);
            },
            t @ _ => self.elem.push(t),
        }

        Ok(())
    }

    pub fn rewrite_data(&mut self, lparen_data: Token, data: Token) -> Result<(), RewriteError> {
        self.data.push(lparen_data);
        self.data.push(data);

        let token = self.lexer.next_token()?;
        
        match token {
            num @ tk!(TokenKind::Number(_)) => {
                self.data.push(num);
                let offset = self.scan_offset()?;
                self.data.extend(offset);
            },
            id @ tk!(TokenKind::Id(_)) => {
                self.data.push(id);
                let offset = self.scan_offset()?;
                self.data.extend(offset);
            },
            lparen @ tk!(TokenKind::LeftParen) => {
                self.data.push(Token::number_u(0, Loc::zero()));
                self.data.push(lparen);
                let offset = self.scan_offset()?;
                self.data.extend(offset);
            },
            t @ _ => self.data.push(t),
        }

        Ok(())
    }

    fn scan_offset(&mut self) -> Result<Vec<Token>, RewriteError> {
        let mut result = vec![];

        match self.lexer.next_token()? {
            lparen @ tk!(TokenKind::LeftParen) => {
                result.push(lparen);
                let offset = self.lexer.next_token()?;
                result.push(offset);
                let instrs = self.rewrite_instrs(vec![])?;
                result.extend(instrs);
        
                let func_indices = self.scan_vec()?;
                result.extend(func_indices);
            },
            offset @ kw!(Keyword::Offset) => {
                result.push(offset);
                let instrs = self.rewrite_instrs(vec![])?;
                result.extend(instrs);
        
                let func_indices = self.scan_vec()?;
                result.extend(func_indices);
            },
            instr @ instr!(_) => {
                result.push(Token::left_paren(Loc::zero()));
                result.push(Token::keyword(Keyword::Offset, Loc::zero()));
                let instr = self.rewrite_instr_for_offset(instr)?;
                result.extend(instr);
                result.push(Token::right_paren(Loc::zero()));

                let func_indices = self.scan_vec()?;
                result.extend(func_indices);
            },
            t @ _ => result.push(t),
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

    fn scan_vec(&mut self) -> Result<Vec<Token>, RewriteError> {
        let mut result = vec![];
    
        while let Ok(token) = self.lexer.next_token() {
            match token {
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
        }

        Ok(result)
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
}
