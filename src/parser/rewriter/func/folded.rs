use std::collections::VecDeque;
use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_instrs(&mut self, first: Vec<Token>) -> Result<(), RewriteError> {
        let mut tokens = VecDeque::from(first);
        let mut token = if let Some(token) = tokens.pop_front() {
            token
        } else {
            self.lexer.next_token()?
        };

        loop {
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

        match token {
            instr_block @ instr!(Instr::Block(_, _)) => {
                self.rewrite_folded_block(instr_block, first)?;
            },
            instr_loop @ instr!(Instr::Loop(_, _)) => {
                self.rewrite_folded_loop(instr_loop, first)?;
            },
            instr_if @ instr!(Instr::If(_, _, _)) => {
                self.rewrite_folded_if(instr_if)?;
            },
            instr!(_) => {
                let mut new_first = vec![token];
                for f in first { new_first.push(f.clone()); }

                self.rewrite_folded_instrs_internal(&mut VecDeque::from(new_first))?;
            },
            tk!(TokenKind::LeftParen) => {
                self.rewrite_folded_instrs(first)?;
            },
            _ => {
                panic!("{:?} (rewrite_folded_instrs)", token);
            },
        }
        
        Ok(())
    }

    fn rewrite_folded_block(&mut self, instr_block: Token, first: &mut VecDeque<Token>) -> Result<(), RewriteError> {
        self.ast.push(instr_block);
        let (label, token) = self.scan_label()?;
        self.ast.extend(label);
        
        let (holding_if, tokens) = self.rewrite_blocktype_if_first(token)?;
        self.ast.extend(holding_if);
        first.extend(tokens);

        self.rewrite_folded_block_loop_instrs(first)?;
        self.ast.push(Token::keyword(Keyword::End, Loc::zero()));
        Ok(())
    }

    fn rewrite_folded_loop(&mut self, instr_loop: Token, first: &mut VecDeque<Token>) -> Result<(), RewriteError> {
        self.ast.push(instr_loop);
        let (label, token) = self.scan_label()?;
        self.ast.extend(label);
        
        let (holding_if, tokens) = self.rewrite_blocktype_if_first(token)?;
        self.ast.extend(holding_if);

        first.extend(tokens);
        self.rewrite_folded_block_loop_instrs(first)?;

        self.ast.push(Token::keyword(Keyword::End, Loc::zero()));
        Ok(())
    }

    fn rewrite_folded_if(&mut self, instr_if: Token) -> Result<(), RewriteError> {
        let mut holding = vec![instr_if];
        let (label, token) = self.scan_label()?;
        holding.extend(label);
        
        let (holding_if, tokens) = self.rewrite_blocktype_if_first(token)?;
        holding.extend(holding_if);

        let mut tokens = VecDeque::from(tokens);
        self.rewrite_folded_instrs(&mut tokens)?;


        self.ast.extend(holding);

        let _lparen = self.lexer.next_token()?;
        let _then = self.lexer.next_token()?;

        self.rewrite_folded_instrs_internal(&mut VecDeque::new())?;

        let elsezero = Token::keyword(Keyword::Else, Loc::zero());
        match self.lexer.next_token()? {
            tk!(TokenKind::LeftParen) => {
                match self.lexer.next_token()? {
                    else_ @ kw!(Keyword::Else) => {
                        self.ast.push(else_);
                        self.rewrite_folded_instrs_internal(&mut VecDeque::new())?;
                        let _rparen = self.lexer.next_token()?;
                    },
                    tk!(TokenKind::RightParen) => self.ast.push(elsezero),
                    t @ _ => self.ast.push(t),
                }
            },
            tk!(TokenKind::RightParen) => self.ast.push(elsezero),
            t @ _ => self.ast.push(t),
        }
        self.ast.push(Token::keyword(Keyword::End, Loc::zero()));

        Ok(())
    }

    fn rewrite_folded_block_loop_instrs(&mut self, first: &mut VecDeque<Token>) -> Result<(), RewriteError> {
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

            match token {
                tk!(TokenKind::RightParen) => break,
                tk!(TokenKind::LeftParen) => self.rewrite_folded_instrs(first)?,
                instr @ instr!(_) => self.ast.push(instr),
                tk!(TokenKind::Empty) => { self.ast.push(token); break; },
                token @ _ => self.ast.push(token),
            }
        }

        Ok(())
    }

    fn rewrite_folded_instrs_internal(&mut self, first: &mut VecDeque<Token>) -> Result<(), RewriteError> {
        let mut first_instr: Vec<Token> = vec![];
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

            match token {
                tk!(TokenKind::RightParen) => {   
                    for instr in &first_instr { self.ast.push(instr.clone()); }
                    first_instr.clear();
                    break;
                },
                tk!(TokenKind::LeftParen) => self.rewrite_folded_instrs(first)?,
                instr @ instr!(_) => first_instr.push(instr),
                tk!(TokenKind::Empty) => { self.ast.push(token); break; },
                token @ _ => first_instr.push(token),
            }
        }

        Ok(())
    }
}

#[test]
fn test_rewrite_instrs_folded1() {
    assert_eq_rewrite(
        "(func (block nop i32.const 0 unreachable))", 
        "(module (func (type <#:gensym>) block nop i32.const 0 unreachable end))"
    );
    assert_eq_rewrite(
        "(func (loop nop i32.const 0 unreachable))", 
        "(module (func (type <#:gensym>) loop nop i32.const 0 unreachable end))"
    );
}

#[test]
fn test_rewrite_instrs_folded2() {
    assert_eq_rewrite(
        "(func (i32.add (local.get 0) (i32.const 2)))", 
        "(module (func (type <#:gensym>) local.get 0 i32.const 2 i32.add))"
    );
}

#[test]
fn test_rewrite_instrs_folded_if() {
    assert_eq_rewrite(
        "(module (func (if (nop) (then unreachable) (else drop))))",
        "(module (func (type <#:gensym>) nop if unreachable else drop end))"
    );
    assert_eq_rewrite(
        "(module (func (if $iiff (result i32) (nop) (then unreachable) (else drop))))",
        "(module (func (type <#:gensym>) nop if $iiff (result i32) unreachable else drop end))"
    );
    assert_eq_rewrite(
        "(module (func (if (result i32) (nop) (then unreachable))))",
        "(module (func (type <#:gensym>) nop if (result i32) unreachable else end))"
    );
    assert_eq_rewrite(
        "(module (func (if (type 0) (nop) (then unreachable) (else drop))))",
        "(module (func (type <#:gensym>) nop if (type 0) unreachable else drop end))"
    );
    assert_eq_rewrite(
        "(module (func (if (param i64) (nop) (then drop))))",
        "(module (func (type <#:gensym>) nop if (type <#:gensym>) (param i64) drop else end))"
    );
    assert_eq_rewrite(
        "(module (func (if (param i32 i64) (nop) (then unreachable) (else drop))))",
        "(module (func (type <#:gensym>) nop if (type <#:gensym>) (param i32) (param i64) unreachable else drop end))"
    );
    assert_eq_rewrite(
        "(module (func (if (type 1) (param f32) (nop) (then drop))))",
        "(module (func (type <#:gensym>) nop if (type 1) (param f32) drop else end))"
    );
    assert_eq_rewrite(
        "(module (func (if (type 1) (param f32 f64) (nop) (then unreachable) (else drop))))",
        "(module (func (type <#:gensym>) nop if (type 1) (param f32) (param f64) unreachable else drop end))"
    );
    assert_eq_rewrite(
        "(module (func (if (type 1) (param f32) (result f64) (nop) (then unreachable) (else drop))))",
        "(module (func (type <#:gensym>) nop if (type 1) (param f32) (result f64) unreachable else drop end))"
    );
    assert_eq_rewrite(
        "(module (func (if (type 1) (param $p i64) (param f32 i32) (result f64) (nop) (then nop) (else nop))))",
        "(module (func (type <#:gensym>) nop if (type 1) (param $p i64) (param f32) (param i32) (result f64) nop else nop end))"
    );
    assert_eq_rewrite(
        "(module $mod (func (if (result i32) (i32.lt_s (local.get $input) (i32.const 0)) (then (i32.sub (i32.const 0) (local.get $input))) (else (local.get $input)))))",
        "(module $mod (func (type <#:gensym>) local.get $input i32.const 0 i32.lt_s if (result i32) i32.const 0 local.get $input i32.sub else local.get $input end))"
    );
}

#[test]
fn test_rewrite_instrs_folded_if_nested() {
    assert_eq_rewrite(
        "(module (func (if (nop) (then (if (drop) (then select))) (else drop))))",
        "(module (func (type <#:gensym>) nop if drop if select else end else drop end))"
    );
}

#[test]
fn test_rewrite_instrs_folded_nested() {
    assert_eq_rewrite(
        "(func (block nop i32.const 0 drop))", 
        "(module (func (type <#:gensym>) block nop i32.const 0 drop end))"
    );
    assert_eq_rewrite(
        "(func (loop nop i32.const 0 drop))", 
        "(module (func (type <#:gensym>) loop nop i32.const 0 drop end))"
    );
    assert_eq_rewrite(
        "(func (block nop i32.const 0 (if (drop) (then select))))", 
        "(module (func (type <#:gensym>) block nop i32.const 0 drop if select else end end))"
    );
    assert_eq_rewrite(
        "(func (loop i32.const 0 (if (drop) (then select)) drop))", 
        "(module (func (type <#:gensym>) loop i32.const 0 drop if select else end drop end))"
    );
}