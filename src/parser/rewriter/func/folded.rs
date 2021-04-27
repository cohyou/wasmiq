use std::collections::VecDeque;
use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_instrs(&mut self, first: Vec<Token>) -> Result<Vec<Token>, RewriteError> {
        let mut result = vec![]; 
        let mut tokens = VecDeque::from(first);
        let mut token = if let Some(token) = tokens.pop_front() {
            token
        } else {
            self.lexer.next_token()?
        };

        loop {
            match token {
                empty @ tk!(TokenKind::Empty) => {
                    result.push(empty);
                    break;
                },
                loop_ @ instr!(Instr::Loop(_, _)) => {
                    result.push(loop_);
                    let token = if let Some(token) = tokens.pop_front() {
                        token
                    } else {
                        self.lexer.next_token()?
                    };
                    let loop_instrs = self.rewrite_loop(token)?;
                    p!(tokens_to_string(loop_instrs.clone()));
                    result.extend(loop_instrs);
                },
                if_ @ instr!(Instr::If(_, _, _)) => {
                    result.push(if_);
                    let if_instrs = self.rewrite_if()?;
                    result.extend(if_instrs);
                },
                tk!(TokenKind::LeftParen) => {
                    let folded_instrs = self.rewrite_folded_instrs(&mut tokens)?;
                    result.extend(folded_instrs);
                },
                rparen @ tk!(TokenKind::RightParen) => {
                    result.push(rparen);
                    break;
                },
                t @ _ => result.push(t),
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

        Ok(result)
    }

    pub fn rewrite_folded_instrs(&mut self, first: &mut VecDeque<Token>) -> Result<Vec<Token>, RewriteError> {
        let mut result = vec![]; 
        let token = if let Some(token) = first.pop_front() {
            token
        } else {
            self.lexer.next_token()?
        };

        match token {
            instr_block @ instr!(Instr::Block(_, _)) => {
                let folded_block = self.rewrite_folded_block(instr_block, first)?;
                result.extend(folded_block);
            },
            instr_loop @ instr!(Instr::Loop(_, _)) => {
                let folded_loop = self.rewrite_folded_loop(instr_loop, first)?;
                result.extend(folded_loop);
            },
            instr_if @ instr!(Instr::If(_, _, _)) => {
                let folded_if = self.rewrite_folded_if(instr_if)?;
                result.extend(folded_if);
            },
            instr!(_) => {
                let mut new_first = vec![token];
                for f in first { new_first.push(f.clone()); }

                let folded_instrs = self.rewrite_folded_instrs_internal(&mut VecDeque::from(new_first))?;
                result.extend(folded_instrs);
            },
            tk!(TokenKind::LeftParen) => {
                let folded_instrs = self.rewrite_folded_instrs(first)?;
                result.extend(folded_instrs);
            },
            _ => {
                panic!("{:?} (rewrite_folded_instrs)", token);
            },
        }
        
        Ok(result)
    }

    fn rewrite_folded_block(&mut self, instr_block: Token, first: &mut VecDeque<Token>) -> Result<Vec<Token>, RewriteError> {
        let mut result = vec![];

        result.push(instr_block);
        let (label, token) = self.scan_label()?;
        result.extend(label);
        
        let (holding_if, tokens) = self.rewrite_blocktype_first(token)?;
        result.extend(holding_if);
        first.extend(tokens);

        let folded_block_loop_instrs = self.rewrite_folded_block_loop_instrs(first)?;
        result.extend(folded_block_loop_instrs);
        result.push(Token::keyword(Keyword::End, Loc::zero()));

        Ok(result)
    }

    fn rewrite_folded_loop(&mut self, instr_loop: Token, first: &mut VecDeque<Token>) -> Result<Vec<Token>, RewriteError> {
        let mut result = vec![];

        result.push(instr_loop);
        let (label, token) = self.scan_label()?;
        result.extend(label);
        
        let (holding_if, tokens) = self.rewrite_blocktype_first(token)?;
        result.extend(holding_if);

        first.extend(tokens);
        let folded_block_loop_instrs = self.rewrite_folded_block_loop_instrs(first)?;
        result.extend(folded_block_loop_instrs);

        result.push(Token::keyword(Keyword::End, Loc::zero()));

        Ok(result)
    }

    fn rewrite_folded_if(&mut self, instr_if: Token) -> Result<Vec<Token>, RewriteError> {
        let mut result = vec![];

        let mut holding = vec![instr_if];
        let (label, token) = self.scan_label()?;
        holding.extend(label);
        
        let (holding_if, tokens) = self.rewrite_blocktype_first(token)?;
        holding.extend(holding_if);

        let mut tokens = VecDeque::from(tokens);
        let folded_instrs = self.rewrite_folded_instrs(&mut tokens)?;
        result.extend(folded_instrs);

        result.extend(holding);

        let _lparen = self.lexer.next_token()?;
        let _then = self.lexer.next_token()?;

        let folded_instrs_internal = self.rewrite_folded_instrs_internal(&mut VecDeque::new())?;
        result.extend(folded_instrs_internal);

        let elsezero = Token::keyword(Keyword::Else, Loc::zero());
        match self.lexer.next_token()? {
            tk!(TokenKind::LeftParen) => {
                match self.lexer.next_token()? {
                    else_ @ kw!(Keyword::Else) => {
                        result.push(else_);
                        let folded_instrs_internal = self.rewrite_folded_instrs_internal(&mut VecDeque::new())?;
                        result.extend(folded_instrs_internal);
                        let _rparen = self.lexer.next_token()?;
                    },
                    tk!(TokenKind::RightParen) => result.push(elsezero),
                    t @ _ => result.push(t),
                }
            },
            tk!(TokenKind::RightParen) => result.push(elsezero),
            t @ _ => result.push(t),
        }
        result.push(Token::keyword(Keyword::End, Loc::zero()));

        Ok(result)
    }

    fn rewrite_folded_block_loop_instrs(&mut self, first: &mut VecDeque<Token>) -> Result<Vec<Token>, RewriteError> {
        let mut result = vec![];

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
                tk!(TokenKind::LeftParen) => {
                    let folded_instrs = self.rewrite_folded_instrs(first)?;
                    result.extend(folded_instrs);
                },
                instr @ instr!(_) => result.push(instr),
                tk!(TokenKind::Empty) => { result.push(token); break; },
                token @ _ => result.push(token),
            }
        }

        Ok(result)
    }

    fn rewrite_folded_instrs_internal(&mut self, first: &mut VecDeque<Token>) -> Result<Vec<Token>, RewriteError> {
        let mut result = vec![];

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
                    for instr in &first_instr { result.push(instr.clone()); }
                    first_instr.clear();
                    break;
                },
                tk!(TokenKind::LeftParen) => {
                    let folded_instrs = self.rewrite_folded_instrs(first)?;
                    result.extend(folded_instrs);
                },
                instr @ instr!(_) => first_instr.push(instr),
                tk!(TokenKind::Empty) => { result.push(token); break; },
                token @ _ => first_instr.push(token),
            }
        }

        Ok(result)
    }
}

#[test]
fn test_rewrite_instrs_folded1() {
    assert_eq_rewrite(
        "(func (block nop i32.const 0 unreachable))", 
        "(module (func (type <#:gensym(0)>) block nop i32.const 0 unreachable end))"
    );
    assert_eq_rewrite(
        "(func (loop nop i32.const 0 unreachable))", 
        "(module (func (type <#:gensym(0)>) loop nop i32.const 0 unreachable end))"
    );
}

#[test]
fn test_rewrite_instrs_folded2() {
    assert_eq_rewrite(
        "(func (i32.add (local.get 0) (i32.const 2)))", 
        "(module (func (type <#:gensym(0)>) local.get 0 i32.const 2 i32.add))"
    );
}

#[test]
fn test_rewrite_instrs_folded_if() {
    assert_eq_rewrite(
        "(module (func (if (nop) (then unreachable) (else drop))))",
        "(module (func (type <#:gensym(0)>) nop if unreachable else drop end))"
    );
    assert_eq_rewrite(
        "(module (func (if $iiff (result i32) (nop) (then unreachable) (else drop))))",
        "(module (func (type <#:gensym(0)>) nop if $iiff (result i32) unreachable else drop end))"
    );
    assert_eq_rewrite(
        "(module (func (if (result i32) (nop) (then unreachable))))",
        "(module (func (type <#:gensym(0)>) nop if (result i32) unreachable else end))"
    );
    assert_eq_rewrite(
        "(module (func (if (type 0) (nop) (then unreachable) (else drop))))",
        "(module (func (type <#:gensym(0)>) nop if (type 0) unreachable else drop end))"
    );
    assert_eq_rewrite(
        "(module (func (if (param i64) (nop) (then drop))))",
        "(module (func (type <#:gensym(0)>) nop if (type <#:gensym(1)>) (param i64) drop else end))"
    );
    assert_eq_rewrite(
        "(module (func (if (param i32 i64) (nop) (then unreachable) (else drop))))",
        "(module (func (type <#:gensym(0)>) nop if (type <#:gensym(1)>) (param i32) (param i64) unreachable else drop end))"
    );
    assert_eq_rewrite(
        "(module (func (if (type 1) (param f32) (nop) (then drop))))",
        "(module (func (type <#:gensym(0)>) nop if (type 1) (param f32) drop else end))"
    );
    assert_eq_rewrite(
        "(module (func (if (type 1) (param f32 f64) (nop) (then unreachable) (else drop))))",
        "(module (func (type <#:gensym(0)>) nop if (type 1) (param f32) (param f64) unreachable else drop end))"
    );
    assert_eq_rewrite(
        "(module (func (if (type 1) (param f32) (result f64) (nop) (then unreachable) (else drop))))",
        "(module (func (type <#:gensym(0)>) nop if (type 1) (param f32) (result f64) unreachable else drop end))"
    );
    assert_eq_rewrite(
        "(module (func (if (type 1) (param $p i64) (param f32 i32) (result f64) (nop) (then nop) (else nop))))",
        "(module (func (type <#:gensym(0)>) nop if (type 1) (param $p i64) (param f32) (param i32) (result f64) nop else nop end))"
    );
    assert_eq_rewrite(
        "(module $mod (func (if (result i32) (i32.lt_s (local.get $input) (i32.const 0)) (then (i32.sub (i32.const 0) (local.get $input))) (else (local.get $input)))))",
        "(module $mod (func (type <#:gensym(0)>) local.get $input i32.const 0 i32.lt_s if (result i32) i32.const 0 local.get $input i32.sub else local.get $input end))"
    );
}

#[test]
fn test_rewrite_instrs_folded_if_nested() {
    assert_eq_rewrite(
        "(module (func (if (nop) (then (if (drop) (then select))) (else drop))))",
        "(module (func (type <#:gensym(0)>) nop if drop if select else end else drop end))"
    );
}

#[test]
fn test_rewrite_instrs_folded_nested() {
    assert_eq_rewrite(
        "(func (block nop i32.const 0 drop))", 
        "(module (func (type <#:gensym(0)>) block nop i32.const 0 drop end))"
    );
    assert_eq_rewrite(
        "(func (loop nop i32.const 0 drop))", 
        "(module (func (type <#:gensym(0)>) loop nop i32.const 0 drop end))"
    );
    assert_eq_rewrite(
        "(func (block nop i32.const 0 (if (drop) (then select))))", 
        "(module (func (type <#:gensym(0)>) block nop i32.const 0 drop if select else end end))"
    );
    assert_eq_rewrite(
        "(func (loop i32.const 0 (if (drop) (then select)) drop))", 
        "(module (func (type <#:gensym(0)>) loop i32.const 0 drop if select else end drop end))"
    );
}