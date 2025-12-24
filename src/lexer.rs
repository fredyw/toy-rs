use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Int(i64),
    Float(f64),
    Str(String),
    Identifier(String),
    Let,
    Fn,
    If,
    Else,
    True,
    False,
    Plus,
    Minus,
    Star,
    Slash,
    Eq,
    EqEq,
    Lt,
    Gt,
    Bang,
    LParen,
    RParen,
    LBrace,
    RBrace,
    SemiColon,
    Comma,
    Eof,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.input.next() {
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('*') => Token::Star,
            Some('/') => Token::Slash,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some(';') => Token::SemiColon,
            Some(',') => Token::Comma,
            Some('!') => Token::Bang,
            Some('<') => Token::Lt,
            Some('>') => Token::Gt,
            Some('=') => {
                if let Some(&'=') = self.input.peek() {
                    self.input.next();
                    Token::EqEq
                } else {
                    Token::Eq
                }
            }
            Some(ch) if ch.is_ascii_digit() => self.read_number(ch),
            Some(ch) if ch.is_alphabetic() || ch == '_' => self.read_identifier(ch),
            None => Token::Eof,
            Some(ch) => panic!("Unexpected character: {}", ch),
        }
    }

    fn read_number(&mut self, first_digit: char) -> Token {
        let mut number_str = String::from(first_digit);
        let mut has_dot = false;
        while let Some(&ch) = self.input.peek() {
            if ch.is_ascii_digit() {
                self.input.next();
                number_str.push(ch);
            } else if ch == '.' && !has_dot {
                self.input.next();
                number_str.push(ch);
                has_dot = true;
            } else {
                break;
            }
        }

        if has_dot {
            let value = number_str.parse::<f64>().unwrap();
            Token::Float(value)
        } else {
            let value = number_str.parse::<i64>().unwrap();
            Token::Int(value)
        }
    }

    fn read_identifier(&mut self, first_char: char) -> Token {
        let mut ident = String::from(first_char);
        while let Some(&ch) = self.input.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                self.input.next();
                ident.push(ch);
            } else {
                break;
            }
        }
        match ident.as_str() {
            "let" => Token::Let,
            "fn" => Token::Fn,
            "if" => Token::If,
            "else" => Token::Else,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Identifier(ident),
        }
    }
}
