use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_type(&mut self, lparen_type: Token, type_: Token) -> Result<(), RewriteError> {
        let mut header = vec![lparen_type, type_];
        let maybe_id = self.lexer.next_token()?;
        let lparen = self.scan_id(maybe_id, &mut header)?;
        for t in header { self.types.push(t); }
        self.types.push(lparen);
        let func = self.lexer.next_token()?;
        self.types.push(func);

        let token = self.lexer.next_token()?;
        self.rewrite_type_first(token)?;

        let rparen = self.lexer.next_token()?;
        self.types.push(rparen);

        Ok(())
    }

    fn rewrite_type_first(&mut self, token: Token) -> Result<(), RewriteError> {
        match token {
            rparen @ tk!(TokenKind::RightParen) => self.types.push(rparen),
            lparen @ tk!(TokenKind::LeftParen) => {                
                self.types.push(lparen);
                match self.lexer.next_token()? {
                    param @ kw!(Keyword::Param) => {
                        self.types.push(param);
                        let holding = self.rewrite_valtypes(Keyword::Param)?;
                        self.types.extend(holding);

                        let token = self.lexer.next_token()?;
                        self.rewrite_type_first(token)?;
                    },
                    result @ kw!(Keyword::Result) => {
                        self.types.push(result);
                        let holding = self.rewrite_valtypes(Keyword::Result)?;
                        self.types.extend(holding);

                        let token = self.lexer.next_token()?;
                        self.rewrite_type_rest(token)?;
                    },
                    t @ _ => self.types.push(t),
                }
            },
            t @ _ => self.types.push(t),
        }

        Ok(())
    }

    fn rewrite_type_rest(&mut self, token: Token) -> Result<(), RewriteError> {
        match token {
            rparen @ tk!(TokenKind::RightParen) => self.types.push(rparen),
            lparen @ tk!(TokenKind::LeftParen) => {                
                self.types.push(lparen);
                match self.lexer.next_token()? {
                    result @ kw!(Keyword::Result) => {
                        self.types.push(result);
                        let holding = self.rewrite_valtypes(Keyword::Result)?;
                        self.types.extend(holding);

                        let token = self.lexer.next_token()?;
                        self.rewrite_type_rest(token)?;
                    },
                    t @ _ => self.types.push(t),
                }
            },
            t @ _ => self.types.push(t),
        }

        Ok(())
    }
}

#[test]
fn test_rewrite_type() {
    assert_eq_rewrite("(type (func))", "(module (type (func)))");
    assert_eq_rewrite("(type $id (func))", "(module (type $id (func)))");

    assert_eq_rewrite(
        "(type (func (param i32)))", 
        "(module (type (func (param i32))))"
    );
    assert_eq_rewrite(
        "(type (func (param i32 f64)))", 
        "(module (type (func (param i32) (param f64))))"
    );
    assert_eq_rewrite(
        "(type (func (param i32 f64) (param i64)))", 
        "(module (type (func (param i32) (param f64) (param i64))))"
    );
    assert_eq_rewrite(
        "(type (func (param $pp f64) (param i64 f32)))", 
        "(module (type (func (param $pp f64) (param i64) (param f32))))"
    );

    assert_eq_rewrite(
        "(type (func (result i32)))", 
        "(module (type (func (result i32))))"
    );
    assert_eq_rewrite(
        "(type (func (result i32 f64)))", 
        "(module (type (func (result i32) (result f64))))"
    );
    assert_eq_rewrite(
        "(type (func (result i32 f64) (result i64)))", 
        "(module (type (func (result i32) (result f64) (result i64))))"
    );

    assert_eq_rewrite(
        "(type (func (param i32) (result f32)))", 
        "(module (type (func (param i32) (result f32))))"
    );
    assert_eq_rewrite(
        "(type (func (param i32 i64) (result f32)))", 
        "(module (type (func (param i32) (param i64) (result f32))))"
    );
    assert_eq_rewrite(
        "(type (func (param i32 i64) (param $pp f32) (result f32 f64)))", 
        "(module (type (func (param i32) (param i64) (param $pp f32) (result f32) (result f64))))"
    );
}