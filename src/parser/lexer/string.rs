use std::io::{Read, Seek};
use super::*;

impl<R> Lexer<R> where R: Read + Seek {

pub fn lex_string(&mut self) -> LexResult {

    let mut string = vec![];
    let mut string_c = self.read()?;
    let mut byte_length_of_codepoint = 0;  // 1 ~ 3
    let mut rest_of_byte_of_char = 0;  // 0 ~ 3
    loop {
        match string_c {
            // end of string
            b'"' => { self.loc.add_pos(); break; },
            // escape sequence
            b'\\' => {
                self.loc.add_pos();
                match self.read()? {
                    b't' =>  { self.loc.add_pos(); string.push(b'\t'); },
                    b'n' =>  { self.loc.add_pos(); string.push(b'\n'); },
                    b'r' =>  { self.loc.add_pos(); string.push(b'\r'); },
                    b'"' =>  { self.loc.add_pos(); string.push(b'"'); },
                    b'\'' => { self.loc.add_pos(); string.push(b'\''); },
                    b'\\' => { self.loc.add_pos(); string.push(b'\\'); },
                    b'u' =>  {
                        self.loc.add_pos();

                        // hexnum
                        let mut hexnum;

                        string_c = self.read()?;
                        self.loc.add_pos();
                        if string_c != b'{' {
                            return Err(self.err(string_c));
                        }

                        string_c = self.read()?;
                        match string_c {
                            n_char @ b'0' ..= b'9' => { self.loc.add_pos(); hexnum = (n_char - 48) as u32; },
                            n_char @ b'A' ..= b'F' => { self.loc.add_pos(); hexnum = (n_char - 55) as u32; },
                            n_char @ b'a' ..= b'f' => { self.loc.add_pos(); hexnum = (n_char - 87) as u32; },
                            0xFF => return Err(LexError::eof(self.loc)),
                            _ => return Err(self.err(string_c)),
                        }

                        string_c = self.read()?;
                        loop {
                            match string_c {
                                b'_' => self.loc.add_pos(),
                                n_char @ b'0' ..= b'9' => { self.loc.add_pos(); hexnum = hexnum * 16 + (n_char - 48) as u32; },
                                n_char @ b'A' ..= b'F' => { self.loc.add_pos(); hexnum = hexnum * 16 + (n_char - 55) as u32; },
                                n_char @ b'a' ..= b'f' => { self.loc.add_pos(); hexnum = hexnum * 16 + (n_char - 87) as u32; },
                                b'}' => { self.loc.add_pos(); break; },
                                0xFF => return Err(LexError::eof(self.loc)),
                                _ => return Err(self.err(string_c)),
                            }
                            string_c = self.read()?;
                        }

                        if let Some(c) = std::char::from_u32(hexnum) {
                            let mut res = String::from_utf8(string.to_vec())?;
                            res.push(c);
                            string = res.bytes().collect();
                        } else {
                            return Err(self.err(string_c));
                        }
                    },
                    n_char @ b'0' ..= b'9' |
                    n_char @ b'A' ..= b'F' |
                    n_char @ b'a' ..= b'f' => {
                        self.loc.add_pos();

                        let n = match n_char {
                            b'0' ..= b'9' => n_char - 48,
                            b'A' ..= b'F' => n_char - 55,
                            b'a' ..= b'f' => n_char - 87,
                            _ => unreachable!(),
                        };
                        
                        match self.read()? {
                            m_char @ b'0' ..= b'9' |
                            m_char @ b'A' ..= b'F' |
                            m_char @ b'a' ..= b'f' => {
                                self.loc.add_pos();
                                let m = match m_char {
                                    b'0' ..= b'9' => m_char - 48,
                                    b'A' ..= b'F' => m_char - 55,
                                    b'a' ..= b'f' => m_char - 87,
                                    _ => unreachable!(),
                                };
                                string.push(16 * n + m);
                            },
                            0xFF => return Err(LexError::eof(self.loc)),
                            _ => return Err(self.err(string_c)),
                        }
                    },
                    0xFF => return Err(LexError::eof(self.loc)),
                    _ => return Err(self.err(string_c)),
                }
            },
            _ if string_c > 0x20 && string_c != 0x7F => {
                if rest_of_byte_of_char == 0 {
                    // count byte as codepoint (not utf-8 bit pattern)
                    match string_c {
                        0x00 ..= 0x7F => self.loc.add_pos(),
                        0xC2 ..= 0xDF => {                                        
                            rest_of_byte_of_char = 1;
                            byte_length_of_codepoint = 2;
                        },
                        0xE0 ..= 0xEF => {
                            rest_of_byte_of_char = 2;
                            byte_length_of_codepoint = 2;                                        
                        },
                        0xF0 ..= 0xF7 => {
                            rest_of_byte_of_char = 3;
                            byte_length_of_codepoint = 3;                                        
                        },
                        _ => return Err(self.err(string_c)),
                    }
                } else {
                    match string_c {
                        0x80 ..= 0xBF => {
                            rest_of_byte_of_char -= 1;
                            if rest_of_byte_of_char == 0 {
                                for _ in 0..byte_length_of_codepoint {
                                    self.loc.add_pos();
                                }
                            }
                        }
                        _ => return Err(self.err(string_c)),
                    }
                }
                string.push(string_c);
            },
            _ => return Err(self.err(string_c)),
        }
        string_c = self.read()?;
    }
    let res = String::from_utf8(string.to_vec())?;

    self.current = self.read()?;
    return Ok(Token::string(res, self.loc))
}

}

#[test]
fn test_escaped_char() {
    let s = r#""\48\65\6c\6c\6f""#;
    read_char(s);
}

#[test]
fn test_escaped_unicode_char() {
    let s = r#""the\u{2464}""#;
    read_char(s);
}

#[allow(dead_code)]
fn read_char(s: &str) {
    use std::io::{Cursor, BufReader};
    let cursor = Cursor::new(s);
    let reader = BufReader::new(cursor);
    let mut lexer = Lexer::new(reader);
    while let Ok(token) = lexer.next_token() {
        if let TokenKind::Empty = token.value {
            break;
        }
        dbg!(token);
    }
}