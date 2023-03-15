use std::iter::Peekable;
use std::str::Bytes;
use std::iter::Enumerate;

use crate::token::{Token, TokenType};
use crate::interpreter::ErrorHandler;

use std::collections::HashMap;

pub struct Lexer<'a> {
    src: &'a str,
    bytes: Peekable<Enumerate<Bytes<'a>>>,
    peekbytes: Peekable<Bytes<'a>>,
    keywords: HashMap<&'a str, TokenType>,

    pub tokens: Vec<Token<'a>>,

    start: usize,
    line: usize,
}

impl <'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let tokens = Vec::new();
        let bytes = src.bytes().enumerate().peekable();
        let mut peekbytes = src.bytes().peekable();
        peekbytes.next();


        let mut keywords = HashMap::new();
        {
            use TokenType::*;
            keywords.insert("and", And);
            keywords.insert("struct", Struct);
            keywords.insert("dimension", Dimension);
            keywords.insert("unit", Unit);
            keywords.insert("for", For);
            keywords.insert("if", If);
            keywords.insert("or", Or);
            keywords.insert("while", While);
            keywords.insert("print", Print);
            keywords.insert("return", Return);
            keywords.insert("self", Slf);
            keywords.insert("true", True);
            keywords.insert("false", False);
        }
        Self {
            src,
            tokens,
            bytes,
            keywords,
            peekbytes,

            start: 0,
            line: 1
        }
    }

    pub fn scan_tokens(&mut self, error_handler: &mut ErrorHandler) {
        while self.scan_token(error_handler) {}

        self.tokens.push(Token::new(TokenType::Eof, "", None, self.line, self.src.len(), self.src.len()));
    }

    fn scan_token(&mut self, error_handler: &mut ErrorHandler) -> bool {
        let (cur, c) = if let Some(c) = self.advance() {c} else {return false};
        self.start = cur;
        {
            use TokenType::*;
            match c as char {
                '(' => self.add_token(LeftParen, None, cur),
                ')' => self.add_token(RightParen, None, cur),
                '[' => self.add_token(LeftSq, None, cur),
                ']' => self.add_token(RightSq, None, cur),
                '{' => self.add_token(LeftCurly, None, cur),
                '}' => self.add_token(RightCurly, None, cur),
                ',' => self.add_token(Comma, None, cur),
                '.' => self.add_token(Dot, None, cur),
                '-' => self.add_token(Minus, None, cur),
                '+' => self.add_token(Plus, None, cur),
                ';' => self.add_token(Semicolon, None, cur),
                '*' => self.add_token(Star, None, cur),
                '^' => self.add_token(Caret, None, cur),

                '!' => {
                    let matches = self.peek_match('=');
                    self.add_token(if matches {ExEq} else {Ex}, None, cur);
                }
                '=' => {
                    let matches = self.peek_match('=');
                    self.add_token(if matches {EqEq} else {Eq}, None, cur);
                }
                '<' => {
                    let matches = self.peek_match('=');
                    self.add_token(if matches {LsEq} else {Ls}, None, cur);
                }
                '>' => {
                    let matches = self.peek_match('=');
                    self.add_token(if matches {GrEq} else {Gr}, None, cur);
                }
                '/' => {
                    let slcomment = self.peek_match('/');
                    if slcomment {
                        loop {
                            let peek = self.bytes.peek();
                            if let Some(p) = peek {
                                if p.1 == '\n' as u8 {
                                    break;
                                }
                            } else {
                                break;
                            }
                            self.advance();
                        }
                    } else {
                        let mlcomment = self.peek_match('*');
                        if mlcomment {
                            self.advance();
                            loop {
                                let peek = self.bytes.peek();
                                if let Some(p) = peek {
                                    if p.1 == '\n' as u8 {
                                        self.line += 1;
                                    } else if p.1 == '*' as u8 {
                                        let nth = self.peekbytes.peek();
                                        if let Some(n) = nth {
                                            if *n == '/' as u8 {
                                                self.advance();
                                                self.advance();
                                                break;
                                            }
                                        }
                                    }
                                } else {
                                    break;
                                }
                                self.advance();
                            }
                        } else {
                            self.add_token(Slash, None, cur);
                        }
                    }
                }

                '\n' => self.line +=1,

                '"' => self.string(error_handler),

                ' ' => (),

                _ => {
                    if c.is_ascii_digit() {
                        self.number(cur);
                    } else if c.is_ascii_alphabetic() {
                        self.identifier(cur);
                    } else {
                        error_handler.error(self.line, &format!("Unexpected character: {c}"));
                    }
                }
            }
        }
        true
    }

    fn string(&mut self, error_handler: &mut ErrorHandler) {
        loop {
            let peek = self.bytes.peek();
            if let Some(c) = peek {
                if c.1 == '"' as u8 {break;}
                else if c.1 == '\n' as u8 {self.line += 1}
                self.advance();
            } else {
                error_handler.error(self.line, "Unterminated string");
                return;
            }
        }
        let cur = self.advance().unwrap().0;

        let val = &self.src[(self.start + 1)..cur];
        self.add_token(TokenType::Str, Some(val), cur);
    }

    fn number(&mut self, mut cur: usize) {
        loop {
            let peek = self.bytes.peek();
            if let Some(p) = peek {
                if !p.1.is_ascii_digit() {
                    break;
                }
                cur = self.advance().unwrap().0;
            } else { break; }
        }

        let peek = self.bytes.peek();
        if let Some(p) = peek {
            if p.1 == '.' as u8 {
                let nth = self.peekbytes.peek();
                if let Some(n) = nth {
                    if n.is_ascii_digit() {
                        cur = self.advance().unwrap().0;

                        loop {
                            let peek = self.bytes.peek();
                            if let Some(p) = peek {
                                if !p.1.is_ascii_digit() {
                                    break;
                                }
                                cur = self.advance().unwrap().0;
                            } else { break; }
                        }
                    }
                }
            }
        }
        self.add_token(TokenType::Num, Some(&self.src[self.start..(cur+1)]), cur);
    }

    fn identifier(&mut self, mut cur: usize) {
        loop {
            let peek = self.bytes.peek();
            if let Some(p) = peek {
                if !p.1.is_ascii_alphanumeric() {
                    break;
                }
                cur = self.advance().unwrap().0;
            } else {
                break;
            }
        }

        let text = &self.src[self.start..(cur + 1)];
        let ty = self.keywords.get(text);
        if let Some(t) = ty {
            self.add_token(*t, None, cur);
        } else {
            self.add_token(TokenType::Id, Some(text), cur);
        }
    }

    fn peek_match(&mut self, expected: char) -> bool {
        if let Some(c) = self.bytes.peek() {
            if c.1 != expected as u8 {
                return false
            }
        } else {
            return false;
        }

        self.advance();
        return true;
    }

    fn add_token(&mut self, t_type: TokenType, literal: Option<&'a str>, cur: usize) {
        let text = &self.src[self.start..(cur + 1)];
        self.tokens.push(Token::new(t_type, text, literal, self.line, self.start, cur + 1));
    }

    fn advance(&mut self) -> Option<(usize, u8)> {
        self.peekbytes.next();
        self.bytes.next()
    }
}