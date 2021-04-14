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
                tk!(TokenKind::Empty) => {
                    self.ast.push(token.clone());
                    break;
                },
                instr!(Instr::If(_, _, _)) => {
                    self.ast.push(token.clone());
                    self.rewrite_if()?;
                },
                tk!(TokenKind::LeftParen) => {
                    self.rewrite_folded_instrs(&mut tokens)?;
                },
                tk!(TokenKind::RightParen) => {
                    self.ast.push(token.clone());
                    break;
                },
                _ => {
                    self.ast.push(token.clone());
                },
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
                _ => {
                    self.ast.push(token);
                },
            }
        }

        Ok(())
    }
}

#[test]
fn test_rewrite_instrs_folded() {
    assert_eq_rewrite(
        "(func (block nop i32.const 0 unreachable))", 
        "(module (func (type <#:gensym>) block nop i32.const 0 unreachable end))"
    );
    assert_eq_rewrite(
        "(func (loop nop i32.const 0 unreachable))", 
        "(module (func (type <#:gensym>) loop nop i32.const 0 unreachable end))"
    );
    // assert_eq_rewrite(
    //     "(func (i32.add (local.get 0) (i32.const 2)) )", 
    //     "(module (func (type <#:gensym>) loop nop i32.const 0 unreachable end))"
    // );
}
