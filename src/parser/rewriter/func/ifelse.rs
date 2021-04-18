use std::collections::VecDeque;
use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_if(&mut self) -> Result<(), RewriteError> {
        let (holding, token) = self.scan_label()?;
        self.ast.extend(holding);

        let (holding, tokens) = self.rewrite_blocktype_if_first(token)?;
        self.ast.extend(holding);
        let mut tokens = VecDeque::from(tokens);
        let mut token = tokens.pop_front().unwrap();

        let mut else_exists = false; 
        loop {
            match token {
                instr!(Instr::If(_, _, _)) => {
                    self.ast.push(token);
                    self.rewrite_if()?;
                },
                kw!(Keyword::Else) => {
                    self.ast.push(token);
                    else_exists = true;
                },
                kw!(Keyword::End) => {
                    if !else_exists {
                        self.ast.push(Token::keyword(Keyword::Else, Loc::zero()));
                    }
                    self.ast.push(token);
                    break;
                },
                tk!(TokenKind::Empty) => {
                    self.ast.push(token);
                    break;
                },
                _ => {
                    self.ast.push(token);
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
}

#[test]
fn test_rewrite_if1() {
    assert_eq_rewrite(
        "(func i32.const 0 if nop else end)", 
        "(module (func (type <#:gensym>) i32.const 0 if nop else end))"
    );
    assert_eq_rewrite(
        "(func (type 0) i32.const 0 if $id else end)", 
        "(module (func (type 0) i32.const 0 if $id else end))"
    );
    assert_eq_rewrite(
        "(func (type 4) i32.const 0 if (result i32) nop else end)", 
        "(module (func (type 4) i32.const 0 if (result i32) nop else end))"
    );
    assert_eq_rewrite(
        "(func (type 4) i32.const 0 if (type 0) (result i32) else end)", 
        "(module (func (type 4) i32.const 0 if (type 0) (result i32) else end))"
    );
    assert_eq_rewrite(
        "(func $fid (type 4) i32.const 0 if (type 8) nop else end)", 
        "(module (func $fid (type 4) i32.const 0 if (type 8) nop else end))"
    );
    assert_eq_rewrite(
        "(func (type 4) i32.const 0 if (param i32) nop else end)", 
        "(module (func (type 4) i32.const 0 if (type <#:gensym>) (param i32) nop else end))"
    );
    assert_eq_rewrite(
        "(func (type 4) i32.const 0 if (param i32 i64) nop else end)", 
        "(module (func (type 4) i32.const 0 if (type <#:gensym>) (param i32) (param i64) nop else end))"
    );
}

#[test]
fn test_rewrite_if2() {
    assert_eq_rewrite(
        "(func i32.const 0 if (type 0) (param i64 f32) nop else end)", 
        "(module (func (type <#:gensym>) i32.const 0 if (type 0) (param i64) (param f32) nop else end))"
    );
    assert_eq_rewrite(
        "(func i32.const 0 if (type 0) (result i64 f32) else end)", 
        "(module (func (type <#:gensym>) i32.const 0 if (type 0) (result i64) (result f32) else end))"
    );
    assert_eq_rewrite(
        "(func i32.const 0 if $id (type 0) (param i64 f32) (result i64) nop else end)", 
        "(module (func (type <#:gensym>) i32.const 0 if $id (type 0) (param i64) (param f32) (result i64) nop else end))"
    );
    assert_eq_rewrite(
        "(func i32.const 0 if (type 0) (param $pr f32) (result i64) else end)", 
        "(module (func (type <#:gensym>) i32.const 0 if (type 0) (param $pr f32) (result i64) else end))"
    );
    assert_eq_rewrite(
        "(func i32.const 0 if $id (param $pr1 f32) (result i64) (result f64) nop else end)", 
        "(module (func (type <#:gensym>) i32.const 0 if $id (type <#:gensym>) (param $pr1 f32) (result i64) (result f64) nop else end))"
    );
    assert_eq_rewrite(
        "(func i32.const 0 if (param $pr1 f32) (param $pr2 i64) (result i64) nop else end)", 
        "(module (func (type <#:gensym>) i32.const 0 if (type <#:gensym>) (param $pr1 f32) (param $pr2 i64) (result i64) nop else end))"
    );
}

#[test]
fn test_rewrite_if_no_else() {
    assert_eq_rewrite(
        "(func i32.const 0 if nop end)", 
        "(module (func (type <#:gensym>) i32.const 0 if nop else end))"
    );
    assert_eq_rewrite(
        "(func (type 0) i32.const 0 if $id end)", 
        "(module (func (type 0) i32.const 0 if $id else end))"
    );
    assert_eq_rewrite(
        "(func (type 4) i32.const 0 if (result i32) end)", 
        "(module (func (type 4) i32.const 0 if (result i32) else end))"
    );
    assert_eq_rewrite(
        "(func (type 4) i32.const 0 if (type 0) (result i32) end)",
        "(module (func (type 4) i32.const 0 if (type 0) (result i32) else end))"
    );
    assert_eq_rewrite(
        "(func $fid (type 4) i32.const 0 if (type 8) nop end)", 
        "(module (func $fid (type 4) i32.const 0 if (type 8) nop else end))"
    );
    assert_eq_rewrite(
        "(func (type 4) i32.const 0 if (param i32) nop end)", 
        "(module (func (type 4) i32.const 0 if (type <#:gensym>) (param i32) nop else end))"
    );
    assert_eq_rewrite(
        "(func (type 4) i32.const 0 if (param i32 i64) nop end)", 
        "(module (func (type 4) i32.const 0 if (type <#:gensym>) (param i32) (param i64) nop else end))"
    );
}

#[test]
fn test_rewrite_if_nested() {
    assert_eq_rewrite(
        "(func i32.const 0 if (result f64) if (result f32) end end)", 
        "(module (func (type <#:gensym>) i32.const 0 if (result f64) if (result f32) else end else end))"
    );
    assert_eq_rewrite(
        "(func i32.const 0 if if if end end end)", 
        "(module (func (type <#:gensym>) i32.const 0 if if if else end else end else end))"
    );
    assert_eq_rewrite(
        "(func i32.const 0 if nop if nop if nop end end end)", 
        "(module (func (type <#:gensym>) i32.const 0 if nop if nop if nop else end else end else end))"
    )
}
