mod error;
#[macro_use] mod comment;
#[macro_use] mod keyword;
mod string;
mod token;

use std::io::{Read, Seek};
use super::annot::{Loc};

pub use self::error::*;
pub use self::comment::*;
pub use self::keyword::*;
pub use self::string::*;
pub use self::token::*;

#[derive(Debug)]
pub struct Lexer<R: Read + Seek> {
    reader: R,
    current: u8,
    loc: Loc,
    peeked_byte: u8,
    peeked_token: Option<Token>,
}

pub type LexResult = Result<Token, LexError>;

impl<R> Lexer<R> where R: Read + Seek {

pub fn new(mut reader: R) -> Lexer<R> {
    let loc = Loc::default();
    let mut buf: &mut [u8] = &mut [0;1];
    let n = reader.read(&mut buf).unwrap();
    let current = if n == 0 { 0xFF } else { buf[0] };
    let peeked_token: Option<Token> = None;

    Lexer {
        reader: reader,
        current: current,
        loc: loc,
        peeked_byte: 0,
        peeked_token: peeked_token,
    }
}

pub fn next_token(&mut self) -> LexResult {
    if let Some(peeked) = &self.peeked_token {
        let result = peeked.clone();
        self.peeked_token = None;
        Ok(result)
    } else {
        self.next_token_internal()
    }
}

// pub fn peek_token(&mut self) -> LexResult {
//     if self.peeked_token.is_none() {
//         self.peeked_token = Some(self.next_token_internal()?);
//     }
//     let result = self.peeked_token.clone();
//     Ok(result.unwrap())
// }

fn next_token_internal(&mut self) -> LexResult {

    loop {
        match self.current {
            // space (normal delimiter)
            b'\t' | b' ' => {
                self.loc.add_pos();
            },

            // space (LF)
            b'\n' => {
                self.loc.newline();
            },

            // space (CR)
            b'\r' => {},

            // line comment
            b';' => {
                self.loc.add_pos();
                lex_line_comment!(self, self.reader);
            },

            // keyword
            b'a' ..= b'z' => {                
                self.loc.add_pos();
                let begin = self.loc;

                let mut keyword = vec![self.current];
                let mut keyword_c = self.read()?;
                loop {
                    if is_idchar(keyword_c) {
                        self.loc.add_pos();
                        keyword.push(keyword_c);
                    } else {
                        self.current = keyword_c;
                        break;
                    }
                    keyword_c = self.read()?;
                }

                match keyword.as_slice() {
                    b"inf" => return Ok(Token::number_f(std::f64::INFINITY, begin)),
                    b"nan" => return Ok(Token::number_f(std::f64::NAN, begin)),
                    _ => return vec_to_keyword(keyword.as_slice())
                                .map_or(Ok(Token::reserved(keyword, begin)),
                                |kw| Ok(Token::keyword(kw, begin)))                    
                }
            },

            // num or hexnum (uN)
            b'0' ..= b'9' => return self.lex_number(b'+', self.loc.added(1)),

            // number
            b @ b'+' | b @ b'-' => {
                self.loc.add_pos();
                self.current = self.read()?;
                return self.lex_number(b, self.loc);
            }

            // string
            b'"' => {
                self.loc.add_pos();
                return self.lex_string();
            },

            // id
            b'$' => {
                self.loc.add_pos();

                let new_loc = self.loc;

                let mut id = vec![];
                let mut id_c = self.read()?;
                loop {
                    if is_idchar(id_c) {
                        self.loc.add_pos();
                        id.push(id_c);
                    } else {
                        self.current = id_c;
                        break;
                    }
                    id_c = self.read()?;
                }

                let res = String::from_utf8(id.to_vec())?;
                return Ok(Token::id(res, new_loc))
            },

            // left paren or start of block comment
            b'(' => {
                self.loc.add_pos();
                let c = self.read()?;

                if c != b';' {
                    // left paren
                    self.current = c;
                    return Ok(Token::left_paren(self.loc));
                }
                self.loc.add_pos();

                // block comment
                self.lex_block_comment()?;
            },

            // right paren
            b')' => {
                self.loc.add_pos();
                self.current = self.read()?;
                // println!("self.current: {:?}", self.current);
                return Ok(Token::right_paren(self.loc));
            },

            // reserved
            _ if is_idchar(self.current) => {
                let current = self.current;
                let mut reserved = vec![current];
                let mut id_c = self.read()?;
                loop {
                    if is_idchar(id_c) {
                        self.loc.add_pos();
                        reserved.push(id_c);
                    } else {
                        self.current = id_c;
                        break;
                    }
                    id_c = self.read()?;
                }
                return Ok(Token::reserved(reserved, self.loc));
            },

            // EOF
            0xFF => return Ok(Token::empty(self.loc)),

            // invalid
            _ => return Err(self.err(self.current)),
        };

        self.current = self.read()?;
    }
}

fn lex_bytes(&mut self, frag: &[u8]) -> Result<(), LexError> {
    for c in frag {
        self.current = self.read()?;
        self.loc.add_pos();
        if self.current != *c {
            return Err(self.err(self.current));
        }
    }
    Ok(())
}

pub fn lex_number(&mut self, sign: u8, begin: Loc) -> LexResult {
    match self.current {
        b'i' => {
            self.loc.add_pos();

            self.lex_bytes(b"nf")?;

            self.current = self.read()?;

            match sign {
                b'+' => return Ok(Token::number_f(std::f64::INFINITY, begin)),
                b'-' => return Ok(Token::number_f(std::f64::NEG_INFINITY, begin)),
                _ => return Err(self.err(self.current)),
            }
            
        },
        b'n' => {
            self.loc.add_pos();

            self.lex_bytes(b"an")?;

            self.current = self.read()?;

            return Ok(Token::number_f(std::f64::NAN, begin));
        },
        _ => {},
    }
    if self.current == b'0' {
        if self.peek()? == b'x' {
            self.read()?;
            self.loc.add_pos();  // for 0
            self.loc.add_pos();  // for x
            // hexnum
            let mut hexnum = 0;

            loop {
                match self.current {
                    b'_' => self.loc.add_pos(),
                    b'0' => { self.loc.add_pos(); hexnum = hexnum * 16 + 0; },
                    b'1' => { self.loc.add_pos(); hexnum = hexnum * 16 + 1; },
                    b'2' => { self.loc.add_pos(); hexnum = hexnum * 16 + 2; },
                    b'3' => { self.loc.add_pos(); hexnum = hexnum * 16 + 3; },
                    b'4' => { self.loc.add_pos(); hexnum = hexnum * 16 + 4; },
                    b'5' => { self.loc.add_pos(); hexnum = hexnum * 16 + 5; },
                    b'6' => { self.loc.add_pos(); hexnum = hexnum * 16 + 6; },
                    b'7' => { self.loc.add_pos(); hexnum = hexnum * 16 + 7; },
                    b'8' => { self.loc.add_pos(); hexnum = hexnum * 16 + 8; },
                    b'9' => { self.loc.add_pos(); hexnum = hexnum * 16 + 9; },
                    b'A' | b'a' => { self.loc.add_pos(); hexnum = hexnum * 16 + 10; },
                    b'B' | b'b' => { self.loc.add_pos(); hexnum = hexnum * 16 + 11; },
                    b'C' | b'c' => { self.loc.add_pos(); hexnum = hexnum * 16 + 12; },
                    b'D' | b'd' => { self.loc.add_pos(); hexnum = hexnum * 16 + 13; },
                    b'E' | b'e' => { self.loc.add_pos(); hexnum = hexnum * 16 + 14; },
                    b'F' | b'f' => { self.loc.add_pos(); hexnum = hexnum * 16 + 15; },
                    0xFF => return Err(LexError::eof(self.loc)),
                    _ => break,                
                }
                self.current = self.read()?;
            }
            
            match sign {
                b'+' => return Ok(Token::number_u(hexnum, begin)),
                b'-' => return Ok(Token::number_i(-(hexnum as isize), begin)),
                _ => return Err(self.err(self.current)),
            }    
        }
    }

    // num
    let mut num = 0;
    let mut frac = 0.0_f64;
    let mut current = self.current;
    loop {
        match current {
            b'_' => self.loc.add_pos(),
            b'.' => {
                // begin frac
                self.loc.add_pos();                
                current = self.read()?;
                let mut digit = 1;
                loop {
                    let powed = 10.0f64.powi(digit);
                    match current {
                        b'_' => { self.loc.add_pos(); current = self.read()?; continue; },
                        b'0' => { self.loc.add_pos(); frac = frac + 0.0 / powed; },
                        b'1' => { self.loc.add_pos(); frac = frac + 1.0 / powed; },
                        b'2' => { self.loc.add_pos(); frac = frac + 2.0 / powed; },
                        b'3' => { self.loc.add_pos(); frac = frac + 3.0 / powed; },
                        b'4' => { self.loc.add_pos(); frac = frac + 4.0 / powed; },
                        b'5' => { self.loc.add_pos(); frac = frac + 5.0 / powed; },
                        b'6' => { self.loc.add_pos(); frac = frac + 6.0 / powed; },
                        b'7' => { self.loc.add_pos(); frac = frac + 7.0 / powed; },
                        b'8' => { self.loc.add_pos(); frac = frac + 8.0 / powed; },
                        b'9' => { self.loc.add_pos(); frac = frac + 9.0 / powed; },
                        0xFF => return Err(LexError::eof(self.loc)),
                        _ => break,
                    }
                    digit += 1;                
                    current = self.read()?;
                }

                let mut float = frac + num as f64;

                match sign {
                    b'+' => {},
                    b'-' => { float = -float },
                    _ => return Err(self.err(self.current)),
                }    
                self.current = current;
                
                return Ok(Token::number_f(float, begin));
            }
            b'0' => { self.loc.add_pos(); num = num * 10 + 0; },
            b'1' => { self.loc.add_pos(); num = num * 10 + 1; },
            b'2' => { self.loc.add_pos(); num = num * 10 + 2; },
            b'3' => { self.loc.add_pos(); num = num * 10 + 3; },
            b'4' => { self.loc.add_pos(); num = num * 10 + 4; },
            b'5' => { self.loc.add_pos(); num = num * 10 + 5; },
            b'6' => { self.loc.add_pos(); num = num * 10 + 6; },
            b'7' => { self.loc.add_pos(); num = num * 10 + 7; },
            b'8' => { self.loc.add_pos(); num = num * 10 + 8; },
            b'9' => { self.loc.add_pos(); num = num * 10 + 9; },
            0xFF => return Err(LexError::eof(self.loc)),
            _ => break,
        }
        current = self.read()?;
    }

    self.current = current;
    match sign {
        b'+' => Ok(Token::number_u(num, begin)),
        b'-' => Ok(Token::number_i(-(num as isize), begin)),
        _ => Err(self.err(self.current)),
    }    
}

fn read(&mut self) -> Result<u8, LexError> {
    if self.peeked_byte == 0 {
        self.read_internal()
    } else {
        let peeked = self.peeked_byte;
        self.peeked_byte = 0;
        Ok(peeked)
    }
}

fn peek(&mut self) -> Result<u8, LexError> {
    if self.peeked_byte == 0 {
        self.peeked_byte = self.read_internal()?;
    }
    Ok(self.peeked_byte)
}

fn read_internal(&mut self) -> Result<u8, LexError> {
    let mut buf: &mut [u8] = &mut [0;1];
    let n = self.reader.read(&mut buf)?;

    if n == 0 { return Ok(0xFF) }
    Ok(buf[0])
}

fn err(&self, c: u8) -> LexError {
    LexError::invalid_char(c, self.loc)
}

}

fn is_idchar(c: u8) -> bool {
    match c {
        b'0' ..= b'9' |
        b'A' ..= b'Z' |
        b'a' ..= b'z' |
        b'!' | b'#' | b'$' | b'%' | b'&' | b'\'' | b'*' | b'+' | b'-' | b'.' | b'/' |
        b':' | b'<' | b'=' | b'>' | b'?' | b'@' | b'\\' | b'^' | b'_' | b'`' | b'|' | b'~' => true,
        _ => false,
    }
}

#[test]
fn test() {
    // unsafe {
    // let u = std::mem::transmute::<i8, u8>(-1);
    // println!("{:?}", u);
    // }
    p!(Token::number_i(-1, Loc::default()));
}
