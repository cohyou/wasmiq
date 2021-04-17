use std::collections::VecDeque;
use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_instrs(&mut self, first: Vec<Token>) -> Result<(), RewriteError> {
        // label blocktype instrs
        let mut tokens = VecDeque::from(first);
        let mut token = if let Some(token) = tokens.pop_front() {
            token
        } else {
            self.lexer.next_token()?
        };
        
        loop {
            // println!("token loop: {:?}", token);
            match token {
                empty @ tk!(TokenKind::Empty) => {
                    self.ast.push(empty);
                    break;
                },
                if_ @ instr!(Instr::If(_, _, _)) => {
                    self.ast.push(if_);
                    self.rewrite_if()?;
                },
                tk!(TokenKind::LeftParen) => {
                    self.rewrite_folded_instrs(&mut tokens)?;
                },
                rparen @ tk!(TokenKind::RightParen) => {
                    self.ast.push(rparen);
                    break;
                },
                _ => self.ast.push(token),
            }

            if let Some(new_token) = tokens.pop_front() {
                token = new_token
            } else {
                if let Ok(new_token) = self.lexer.next_token() {
                    token = new_token
                } else {
                    break;
                }
            }
        }

        Ok(())
    }

    fn rewrite_folded_instrs(&mut self, first: &mut VecDeque<Token>) -> Result<(), RewriteError> {
        let token = if let Some(token) = first.pop_front() {
            token
        } else {
            self.lexer.next_token()?
        };
        
        match &token {
            
            instr_block @ instr!(Instr::Block(_, _)) => {
                self.ast.push(instr_block.clone());
                self.rewrite_folded_instrs_internal(first)?;
                self.ast.push(Token::keyword(Keyword::End, Loc::zero()));
            },
            instr_loop @ instr!(Instr::Loop(_, _)) => {
                self.ast.push(instr_loop.clone());
                self.rewrite_folded_instrs_internal(first)?;
                self.ast.push(Token::keyword(Keyword::End, Loc::zero()));
            },
            instr_if @ instr!(Instr::If(_, _, _)) => {
                self.ast.push(instr_if.clone());
                // (if label blocktype foldedinstr* (then instr*1) (else instr*2)? )
                // foldedinstr* ‘if’ label blocktype instr*1 ‘else’ (instr*2)? ‘end’
                self.rewrite_folded_if()?;
                self.ast.push(Token::keyword(Keyword::End, Loc::zero()));
            },
            instr @ instr!(_) => {
                // plaininstr
                self.rewrite_folded_instrs_internal(first)?;
                self.ast.push(instr.clone());
            },
            _ => unimplemented!(),
        }
        
        Ok(())
    }

    fn rewrite_folded_if(&mut self) -> Result<(), RewriteError> {
        unimplemented!()
    }

    fn rewrite_folded_instrs_internal(&mut self, first: &mut VecDeque<Token>) -> Result<(), RewriteError> {
        let mut token;
        loop {
            if let Some(new_token) = first.pop_front() {
                token = new_token
            } else {
                if let Ok(new_token) = self.lexer.next_token() {
                    token = new_token
                } else {
                    break;
                }
            }
            // println!("token folded: {:?}", token);
            match token {
                tk!(TokenKind::RightParen) => {                    
                    break;
                },
                tk!(TokenKind::LeftParen) => {
                    self.rewrite_folded_instrs(first)?;
                },
                tk!(TokenKind::Empty) => {
                    self.ast.push(token);
                    break;
                },
                instr!(_) => {
                    self.scan_one_instr(token)?;
                },
                _ => {
                    self.ast.push(token);
                },
            }
        }

        Ok(())
    }

    fn scan_one_instr(&mut self, token: Token) -> Result<(), RewriteError> {
        if let instr!(ref instr) = token {
            match instr {
                Instr::Block(_, _) |
                Instr::Loop(_, _) => {
                    unreachable!();
                },
    
                Instr::If(_, _, _) => {
                    unreachable!();
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
        } else {
            unreachable!();
        }

        Ok(())
    }
}

#[test]
#[ignore]
fn test_rewrite_instrs_folded() {
    assert_eq_rewrite(
        "(func (block nop i32.const 0 unreachable))", 
        "(module (func (type <#:gensym>) block nop i32.const 0 unreachable end))"
    );
    assert_eq_rewrite(
        "(func (loop nop i32.const 0 unreachable))", 
        "(module (func (type <#:gensym>) loop nop i32.const 0 unreachable end))"
    );
    assert_eq_rewrite(
        "(func (i32.add (local.get 0) (i32.const 2)) )", 
        "(module (func (type <#:gensym>) loop nop i32.const 0 unreachable end))"
    );
    print_tokens("(func i32.load offset=1234 align=45679)");
}
