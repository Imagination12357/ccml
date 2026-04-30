use std::collections::BTreeMap;

use crate::ast::AstNode;
use crate::diag::CcmlError;

pub struct Parser<'a> {
    src: &'a str,
    idx: usize,
    line: usize,
    col: usize,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            idx: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn parse_start(mut self) -> Result<AstNode, CcmlError> {
        self.skip_ws_and_comments();
        if self.eof() {
            return Ok(AstNode::Object(BTreeMap::new()));
        }
        match self.peek() {
            Some('{') => self.parse_object(),
            Some('[') => self.parse_array(),
            _ => self.parse_implicit_root_object(),
        }
    }

    fn parse_implicit_root_object(&mut self) -> Result<AstNode, CcmlError> {
        let mut map = BTreeMap::new();
        loop {
            self.skip_ws_and_comments();
            if self.eof() {
                break;
            }
            let key = self.parse_key()?;
            self.skip_ws_and_comments();
            self.expect(':', "CCML1004", "missing colon in pair")?;
            self.skip_ws_and_comments();
            if self.eof() {
                return Err(self.err("CCML1001", "missing value"));
            }
            let value = self.parse_value()?;
            map.insert(key, value);
            self.skip_separators();
        }
        Ok(AstNode::Object(map))
    }

    fn parse_value(&mut self) -> Result<AstNode, CcmlError> {
        self.skip_ws_and_comments();
        match self.peek() {
            Some('{') => self.parse_object(),
            Some('[') => self.parse_array(),
            Some('"') => Ok(AstNode::String(self.parse_string()?)),
            Some('t') => {
                self.expect_keyword("true")?;
                Ok(AstNode::Bool(true))
            }
            Some('f') => {
                self.expect_keyword("false")?;
                Ok(AstNode::Bool(false))
            }
            Some('n') => {
                self.expect_keyword("null")?;
                Ok(AstNode::Null)
            }
            Some('-') | Some('0'..='9') => Ok(AstNode::Number(self.parse_number()?)),
            Some(_) => Err(self.err("CCML1001", "unexpected token while parsing value")),
            None => Err(self.err("CCML1001", "unexpected end of input")),
        }
    }

    fn parse_object(&mut self) -> Result<AstNode, CcmlError> {
        self.expect('{', "CCML1005", "expected '{'")?;
        let mut map = BTreeMap::new();
        loop {
            self.skip_ws_and_comments();
            if self.match_char('}') {
                break;
            }
            if self.eof() {
                return Err(self.err("CCML1005", "mismatched closing delimiter"));
            }
            let key = self.parse_key()?;
            self.skip_ws_and_comments();
            self.expect(':', "CCML1004", "missing colon in pair")?;
            self.skip_ws_and_comments();
            if self.eof() {
                return Err(self.err("CCML1001", "missing value"));
            }
            let value = self.parse_value()?;
            map.insert(key, value);
            self.skip_separators();
        }
        Ok(AstNode::Object(map))
    }

    fn parse_array(&mut self) -> Result<AstNode, CcmlError> {
        self.expect('[', "CCML1005", "expected '['")?;
        let mut out = Vec::new();
        loop {
            self.skip_ws_and_comments();
            if self.match_char(']') {
                break;
            }
            if self.eof() {
                return Err(self.err("CCML1005", "mismatched closing delimiter"));
            }
            out.push(self.parse_value()?);
            self.skip_separators();
        }
        Ok(AstNode::Array(out))
    }

    fn parse_key(&mut self) -> Result<String, CcmlError> {
        self.skip_ws_and_comments();
        match self.peek() {
            Some('"') => self.parse_string(),
            Some(_) => self.parse_bare_key(),
            None => Err(self.err("CCML1006", "invalid key token")),
        }
    }

    fn parse_bare_key(&mut self) -> Result<String, CcmlError> {
        let start = self.idx;
        while let Some(ch) = self.peek() {
            if is_bare_key_char(ch) {
                self.bump();
            } else {
                break;
            }
        }
        if self.idx == start {
            return Err(self.err("CCML1006", "invalid key token"));
        }
        let key = &self.src[start..self.idx];
        if key.chars().all(|c| c == '.' || c == '-') {
            return Err(self.err("CCML1006", "invalid key token"));
        }
        Ok(key.to_string())
    }

    fn parse_string(&mut self) -> Result<String, CcmlError> {
        self.expect('"', "CCML1002", "unterminated string")?;
        let mut out = String::new();
        while let Some(ch) = self.peek() {
            match ch {
                '"' => {
                    self.bump();
                    return Ok(out);
                }
                '\\' => {
                    self.bump();
                    let esc = self
                        .peek()
                        .ok_or_else(|| self.err("CCML1002", "unterminated string"))?;
                    self.bump();
                    match esc {
                        '"' => out.push('"'),
                        '\\' => out.push('\\'),
                        '/' => out.push('/'),
                        'b' => out.push('\u{0008}'),
                        'f' => out.push('\u{000C}'),
                        'n' => out.push('\n'),
                        'r' => out.push('\r'),
                        't' => out.push('\t'),
                        'u' => {
                            let code = self.parse_u4_hex()?;
                            if let Some(ch) = char::from_u32(code) {
                                out.push(ch);
                            } else {
                                return Err(self.err("CCML1002", "invalid unicode escape"));
                            }
                        }
                        _ => return Err(self.err("CCML1002", "invalid string escape")),
                    }
                }
                _ => {
                    out.push(ch);
                    self.bump();
                }
            }
        }
        Err(self.err("CCML1002", "unterminated string"))
    }

    fn parse_u4_hex(&mut self) -> Result<u32, CcmlError> {
        let mut value: u32 = 0;
        for _ in 0..4 {
            let ch = self
                .peek()
                .ok_or_else(|| self.err("CCML1002", "invalid unicode escape"))?;
            self.bump();
            let digit = ch
                .to_digit(16)
                .ok_or_else(|| self.err("CCML1002", "invalid unicode escape"))?;
            value = (value << 4) | digit;
        }
        Ok(value)
    }

    fn parse_number(&mut self) -> Result<String, CcmlError> {
        let start = self.idx;
        if self.match_char('-') {}

        match self.peek() {
            Some('0') => {
                self.bump();
                if matches!(self.peek(), Some('0'..='9')) {
                    return Err(self.err("CCML1003", "invalid number format"));
                }
            }
            Some('1'..='9') => {
                self.bump();
                while matches!(self.peek(), Some('0'..='9')) {
                    self.bump();
                }
            }
            _ => return Err(self.err("CCML1003", "invalid number format")),
        }

        if self.match_char('.') {
            if !matches!(self.peek(), Some('0'..='9')) {
                return Err(self.err("CCML1003", "invalid number format"));
            }
            while matches!(self.peek(), Some('0'..='9')) {
                self.bump();
            }
        }

        if matches!(self.peek(), Some('e' | 'E')) {
            self.bump();
            if matches!(self.peek(), Some('+' | '-')) {
                self.bump();
            }
            if !matches!(self.peek(), Some('0'..='9')) {
                return Err(self.err("CCML1003", "invalid number format"));
            }
            while matches!(self.peek(), Some('0'..='9')) {
                self.bump();
            }
        }

        Ok(self.src[start..self.idx].to_string())
    }

    fn expect_keyword(&mut self, kw: &str) -> Result<(), CcmlError> {
        for ch in kw.chars() {
            if self.peek() == Some(ch) {
                self.bump();
            } else {
                return Err(self.err("CCML1001", "unexpected token"));
            }
        }
        Ok(())
    }

    fn skip_separators(&mut self) {
        loop {
            let before = self.idx;
            self.skip_ws_and_comments();
            if self.match_char(',') {
                self.skip_ws_and_comments();
            }
            if self.idx == before {
                break;
            }
        }
    }

    fn skip_ws_and_comments(&mut self) {
        loop {
            let before = self.idx;
            while let Some(ch) = self.peek() {
                if ch.is_whitespace() {
                    self.bump();
                } else {
                    break;
                }
            }
            if self.peek() == Some('#') {
                while let Some(ch) = self.peek() {
                    self.bump();
                    if ch == '\n' {
                        break;
                    }
                }
            }
            if before == self.idx {
                break;
            }
        }
    }

    fn expect(&mut self, ch: char, code: &str, message: &str) -> Result<(), CcmlError> {
        if self.match_char(ch) {
            Ok(())
        } else {
            Err(self.err(code, message))
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Option<char> {
        self.src[self.idx..].chars().next()
    }

    fn bump(&mut self) {
        if let Some(ch) = self.peek() {
            self.idx += ch.len_utf8();
            if ch == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
    }

    fn eof(&self) -> bool {
        self.idx >= self.src.len()
    }

    fn err(&self, code: &str, message: &str) -> CcmlError {
        CcmlError::single(code, message, self.line, self.col)
    }
}

fn is_bare_key_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' || ch == '-'
}
