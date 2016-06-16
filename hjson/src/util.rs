
use std::i32;
use std::io;

use super::error::{Error, ErrorCode, Result};

use super::value::Value;

pub struct StringReader<Iter: Iterator<Item=u8>> {
    iter: Iter,
    line: usize,
    col: usize,
    ch: Vec<u8>,
}

impl<Iter> StringReader<Iter>
    where Iter: Iterator<Item=u8> {

    #[inline]
    pub fn new(iter: Iter) -> Self {
        StringReader {
            iter: iter,
            line: 1,
            col: 0,
            ch: Vec::new(),
        }
    }

    fn next(&mut self) -> Option<io::Result<u8>> {
        match self.iter.next() {
            None => None,
            Some(b'\n') => {
                self.line += 1;
                self.col = 0;
                Some(Ok(b'\n'))
            },
            Some(c) => {
                self.col += 1;
                Some(Ok(c))
            },
        }
    }

    pub fn pos(&mut self) -> (usize, usize) {
        (self.line, self.col)
    }

    pub fn eof(&mut self) -> Result<bool> {
        Ok(try!(self.peek()).is_none())
    }

    pub fn peek_next(&mut self, idx: usize) -> Result<Option<u8>> {
        while self.ch.len() <= idx {
            match self.next() {
                Some(Err(err)) => return Err(Error::Io(err)),
                Some(Ok(ch)) => self.ch.push(ch),
                None => return Ok(None),
            }
        }
        Ok(Some(self.ch[idx]))
    }

    // pub fn peek_next_or_null(&mut self, idx: usize) -> Result<u8> {
    //     Ok(try!(self.peek_next(idx)).unwrap_or(b'\x00'))
    // }

    pub fn peek(&mut self) -> Result<Option<u8>> {
        self.peek_next(0)
    }

    pub fn peek_or_null(&mut self) -> Result<u8> {
        Ok(try!(self.peek()).unwrap_or(b'\x00'))
    }

    pub fn eat_char(&mut self) {
        self.ch.remove(0);
    }

    pub fn uneat_char(&mut self, ch: u8) {
        self.ch.insert(0, ch);
    }

    pub fn next_char(&mut self) -> Result<Option<u8>> {
        match self.ch.first() {
            Some(&ch) => { self.eat_char(); Ok(Some(ch)) },
            None => {
                match self.next() {
                    Some(Err(err)) => Err(Error::Io(err)),
                    Some(Ok(ch)) => Ok(Some(ch)),
                    None => Ok(None),
                }
            }
        }
    }

    pub fn next_char_or_null(&mut self) -> Result<u8> {
        Ok(try!(self.next_char()).unwrap_or(b'\x00'))
    }

    fn eat_line(&mut self) -> Result<()> {
        loop {
            match try!(self.peek()) {
                Some(b'\n') | None => return Ok(()),
                _ => {},
            }
            self.eat_char();
        }
    }

    pub fn parse_whitespace(&mut self) -> Result<()> {
        loop {
            match try!(self.peek_or_null()) {
                b' ' | b'\n' | b'\t' | b'\r' => {
                    self.eat_char();
                }
                b'#' => try!(self.eat_line()),
                b'/' => {
                    match try!(self.peek_next(1)) {
                        Some(b'/') => try!(self.eat_line()),
                        Some(b'*') => {
                            self.eat_char();
                            self.eat_char();
                            while !(try!(self.peek()).unwrap_or(b'*') == b'*' && try!(self.peek_next(1)).unwrap_or(b'/') == b'/') {
                                self.eat_char();
                            }
                            self.eat_char();
                            self.eat_char();
                        },
                        Some(_) => self.eat_char(),
                        None => return Err(self.error(ErrorCode::TrailingCharacters)), //todo
                    }
                }
                _ => { return Ok(()); },
            }
        }
    }

    pub fn error(&mut self, reason: ErrorCode) -> Error {
        Error::Syntax(reason, self.line, self.col)
    }
}


pub struct ParseNumber<Iter: Iterator<Item=u8>> {
    rdr: StringReader<Iter>,
}

macro_rules! try_or_invalid {
    ($e:expr) => {
        match $e {
            Some(v) => v,
            None => { return Err(Error::Syntax(ErrorCode::InvalidNumber, 0, 0)); }
        }
    }
}

impl<Iter: Iterator<Item=u8>> ParseNumber<Iter> {

    #[inline]
    pub fn new(iter: Iter) -> Self {
        ParseNumber {
            rdr: StringReader::new(iter),
        }
    }

    pub fn parse(&mut self, stop_at_next: bool) -> Result<Value> {

        match self.parse_integer() {
            Ok(v) => {

                try!(self.rdr.parse_whitespace());

                let ch = try!(self.rdr.next_char_or_null());

                if stop_at_next {
                  // end scan if we find a punctuator character like ,}] or a comment
                  //if (ch == b',' || ch == b'}' || ch == b']' ||
                  //  ch == b'#' || ch == b'/' && (text[at] == b'/' || text[at] == b'*')) ch = 0;
                }

                match ch {
                    b'\x00' => { return Ok(v); },
                    _ => { return Err(Error::Syntax(ErrorCode::InvalidNumber, 0, 0)); },
                }
            },
            Err(e) => Err(e),
        }
    }

    fn parse_integer(&mut self) -> Result<Value> {
        let pos = if try!(self.rdr.peek_or_null()) == b'-' {
            self.rdr.eat_char();
            false
        } else { true };

        match try!(self.rdr.next_char_or_null()) {
            b'0' => {
                // There can be only one leading '0'.
                match try!(self.rdr.peek_or_null()) {
                    b'0' ... b'9' => {
                        Err(Error::Syntax(ErrorCode::InvalidNumber, 0, 0))
                    }
                    _ => {
                        self.parse_num_next(pos, 0)
                    }
                }
            },
            c @ b'1' ... b'9' => {
                let mut res: u64 = (c as u64) - ('0' as u64);

                loop {
                    match try!(self.rdr.peek_or_null()) {
                        c @ b'0' ... b'9' => {
                            self.rdr.eat_char();

                            let digit = (c as u64) - ('0' as u64);

                            // We need to be careful with overflow. If we can, try to keep the
                            // number as a `u64` until we grow too large. At that point, switch to
                            // parsing the value as a `f64`.
                            match res.checked_mul(10).and_then(|val| val.checked_add(digit)) {
                                Some(res_) => { res = res_; }
                                None => {
                                    return self.parse_float(pos, (res as f64) * 10.0 + (digit as f64));
                                }
                            }
                        }
                        _ => {
                            return self.parse_num_next(pos, res);
                        }
                    }
                }
            }
            _ => {
                Err(Error::Syntax(ErrorCode::InvalidNumber, 0, 0))
            }
        }
    }

    fn parse_float(&mut self,

                      pos: bool,
                      mut res: f64) -> Result<Value> {
        loop {
            match try!(self.rdr.next_char_or_null()) {
                c @ b'0' ... b'9' => {
                    let digit = (c as u64) - ('0' as u64);

                    res *= 10.0;
                    res += digit as f64;
                }
                _ => {
                    match try!(self.rdr.peek_or_null()) {
                        b'.' => {
                            return self.parse_decimal(pos, res);
                        }
                        b'e' | b'E' => {
                            return self.parse_exponent(pos, res);
                        }
                        _ => {
                            if !pos {
                                res = -res;
                            }

                            return Ok(Value::F64(res));
                        }
                    }
                }
            }
        }
    }

    fn parse_num_next(&mut self,

                       pos: bool,
                       res: u64) -> Result<Value> {
        match try!(self.rdr.peek_or_null()) {
            b'.' => {
                self.parse_decimal(pos, res as f64)
            }
            b'e' | b'E' => {
                self.parse_exponent(pos, res as f64)
            }
            _ => {
                if pos {
                    Ok(Value::U64(res))
                } else {
                    let res_i64 = (res as i64).wrapping_neg();

                    // Convert into a float if we underflow.
                    if res_i64 > 0 {
                        Ok(Value::F64(-(res as f64)))
                    } else {
                        Ok(Value::I64(res_i64))
                    }
                }
            }
        }
    }

    fn parse_decimal(&mut self,

                        pos: bool,
                        mut res: f64) -> Result<Value> {
        self.rdr.eat_char();

        let mut dec = 0.1;

        // Make sure a digit follows the decimal place.
        match try!(self.rdr.next_char_or_null()) {
            c @ b'0' ... b'9' => {
                res += (((c as u64) - (b'0' as u64)) as f64) * dec;
            }
             _ => { return Err(Error::Syntax(ErrorCode::InvalidNumber, 0, 0)); }
        }

        loop {
            match try!(self.rdr.peek_or_null()) {
                c @ b'0' ... b'9' => {
                    self.rdr.eat_char();

                    dec /= 10.0;
                    res += (((c as u64) - (b'0' as u64)) as f64) * dec;
                }
                _ => { break; }
            }
        }

        match try!(self.rdr.peek_or_null()) {
            b'e' | b'E' => {
                self.parse_exponent(pos, res)
            }
            _ => {
                if pos {
                    Ok(Value::F64(res))
                } else {
                    Ok(Value::F64(-res))
                }
            }
        }

    }

    fn parse_exponent(&mut self, pos: bool, mut res: f64) -> Result<Value> {
        self.rdr.eat_char();

        let pos_exp = match try!(self.rdr.peek_or_null()) {
            b'+' => { self.rdr.eat_char(); true }
            b'-' => { self.rdr.eat_char(); false }
            _ => { true }
        };

        // Make sure a digit follows the exponent place.
        let mut exp = match try!(self.rdr.next_char_or_null()) {
            c @ b'0' ... b'9' => { (c as u64) - (b'0' as u64) }
            _ => { return Err(Error::Syntax(ErrorCode::InvalidNumber, 0, 0)); }
        };

        loop {
            match try!(self.rdr.peek_or_null()) {
                c @ b'0' ... b'9' => {
                    self.rdr.eat_char();

                    exp = try_or_invalid!(exp.checked_mul(10));
                    exp = try_or_invalid!(exp.checked_add((c as u64) - (b'0' as u64)));
                }
                _ => { break; }
            }
        }

        let exp = if exp <= i32::MAX as u64 {
            10_f64.powi(exp as i32)
        } else {
            return Err(Error::Syntax(ErrorCode::InvalidNumber, 0, 0));
        };

        if pos_exp {
            res *= exp;
        } else {
            res /= exp;
        }

        if pos {
            Ok(Value::F64(res))
        } else {
            Ok(Value::F64(-res))
        }
    }
}
