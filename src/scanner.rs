use std::collections::HashMap;

use crate::{error::LoxError, token::Token, token_type::TokenType};

#[derive(Debug)]
pub struct Scanner {
    pub source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            crate::token_type::TokenType::Eof,
            String::new(),
            None,
            self.line,
        ));

        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),

            '!' => {
                let token_type = if self.advance_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type, None);
            }
            '=' => {
                let token_type = if self.advance_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type, None);
            }
            '<' => {
                let token_type = if self.advance_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type, None);
            }
            '>' => {
                let token_type = if self.advance_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type, None);
            }
            '/' => {
                if self.advance_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.add_string(),
            '0'..='9' => self.add_number(),
            'o' => {
                if self.advance_match('r') {
                    self.add_token(TokenType::Or, None);
                } else {
                    LoxError::new("Unexpected character".to_string(), self.line).report();
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => self.add_identifier(),

            _ => LoxError::new("Unexpected character".to_string(), self.line).report(),
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line));
    }

    fn advance(&mut self) -> char {
        let char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;

        char
    }

    fn advance_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn add_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            LoxError::new("Unterminated string".to_string(), self.line).report();
            return;
        }
        self.advance();
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, Some(value.to_string()));
    }

    fn add_number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        let value = &self.source[self.start..self.current];
        self.add_token(TokenType::Number, Some(value.parse::<String>().unwrap()));
    }

    fn add_identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = keywordToTokenType(text);

        self.add_token(token_type, None);
    }

    fn is_alphanumeric(&self, peek: char) -> bool {
        self.is_digit(peek) || self.is_alpha(peek)
    }

    fn is_digit(&self, peek: char) -> bool {
        char::is_ascii_digit(&peek)
    }

    fn is_alpha(&self, peek: char) -> bool {
        char::is_ascii_alphabetic(&peek)
    }
}

fn keywordToTokenType(keyword: &str) -> TokenType {
    match keyword {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "for" => TokenType::For,
        "fun" => TokenType::Fun,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,
        _ => TokenType::Identifier,
    }
}
