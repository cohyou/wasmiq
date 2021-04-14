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
                    self.ast.push(Token::keyword(Keyword::Instr(Instr::Unreachable), Loc::zero()));
                    self.ast.push(token.clone());
                    self.rewrite_folded_instrs()?;
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

    fn rewrite_folded_instrs(&mut self) -> Result<(), RewriteError> {
        // plaininstr
        // block
        // loop
        // if
        unimplemented!()
    }
}

#[test]
fn test_rewrite_instrs1() {
    assert_eq_rewrite("(func nop)", "(module (func (type <#:gensym>) nop))");
}

#[test]
fn test_rewrite_func_import() {
    assert_eq_rewrite(
        r#"(func (import "n1" "n2"))"#, 
        r#"(module (import "n1" "n2" (func)))"#
    );
}