use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_elem(&mut self) -> Result<(), RewriteError> {
        let token = self.lexer.next_token()?;
        
        match token {
            tk!(TokenKind::Empty) => {
                self.ast.push(token.clone());
            },
            tk!(TokenKind::Number(_)) => {
                self.ast.push(token.clone());
                self.rewrite_instr_for_offset()?;
            },
            _ => {
                self.ast.push(Token::number_u(0, Loc::zero()));
                self.ast.push(token.clone());
            },
        }

        Ok(())
    }

    pub fn rewrite_data(&mut self) -> Result<(), RewriteError> {
        let token = self.lexer.next_token()?;
        
        match token {
            tk!(TokenKind::Empty) => {
                self.ast.push(token.clone());
            },
            tk!(TokenKind::Number(_)) => {
                self.ast.push(token.clone());
                self.rewrite_instr_for_offset()?;
            },
            _ => {
                self.ast.push(Token::number_u(0, Loc::zero()));
                self.ast.push(token.clone());
            },
        }

        Ok(())
    }

    fn rewrite_instr_for_offset(&mut self) -> Result<(), RewriteError> {
        let token = self.lexer.next_token()?;
        match &token {
            instr!(instr) => {
                self.ast.push(Token::left_paren(Loc::zero()));
                self.ast.push(Token::keyword(Keyword::Offset, Loc::zero()));
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
                        self.ast.push(token.clone());
                        self.ast.push(token2.clone());
                    },

                    Instr::Load(_, _) |
                    Instr::ILoad8(_, _, _) |
                    Instr::ILoad16(_, _, _) |
                    Instr::I64Load32(_, _) |
                    Instr::Store(_, _) |
                    Instr::IStore8(_, _) |
                    Instr::IStore16(_, _) |
                    Instr::I64Store32(_) => {
                        self.ast.push(token.clone());
                        let token2 = self.lexer.next_token()?;
                        match token2 {
                            kw!(Keyword::MemArgOffset(_)) => {
                                self.ast.push(token2.clone());
                                let token3 = self.lexer.next_token()?;
                                match token3 {
                                    kw!(Keyword::MemArgAlign(_)) => {
                                        self.ast.push(token3.clone());
                                        let token4 = self.lexer.next_token()?;
                                        self.ast.push(token4.clone());
                                    }
                                    _ => self.ast.push(token3.clone()),
                                }
                            },
                            kw!(Keyword::MemArgAlign(_)) => {
                                self.ast.push(token2.clone());
                                let token3 = self.lexer.next_token()?;
                                self.ast.push(token3.clone());
                            },
                            _ => self.ast.push(token2.clone()),
                        }
                    },

                    Instr::Unreachable |
                    Instr::Nop |
                    Instr::Return |
                    Instr::Drop(_) |
                    Instr::Select(_) |
                    Instr::MemorySize |
                    Instr::MemoryGrow => {
                        self.ast.push(token.clone());
                    },
                    _ => { self.ast.push(token.clone()); }
                }
                self.ast.push(Token::right_paren(Loc::zero()));
            },
            _ => self.ast.push(token.clone()),
        }
        Ok(())
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
        "(elem 0 (offset f64.const 3.14))",
        "(module (elem 0 (offset f64.const 3.14)))"
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
