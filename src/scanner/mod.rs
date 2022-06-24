use std::collections::HashMap;

pub mod error;
#[cfg(test)]
mod tests;

use crate::token::*;

// 'a says the keyword hashmap must live the lifetime of the Scanner instance
pub struct Scanner<'a> {
    source: String,
    source_len: usize,
    tokens: Vec<Token>,
    line: usize,
    start: usize,
    current: usize,
    keyword_map: HashMap<&'a str, TokenType>,
    error: Option<error::ScanError>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: String) -> Self {
        let keyword_map = HashMap::from([
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::For),
            ("fun", TokenType::Fun),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
        ]);

        let source_len = source.len();
        Self {
            source,
            source_len,
            tokens: Vec::new(),
            line: 1,
            current: 0,
            start: 0,
            keyword_map,
            error: None,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, error::ScanError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF));

        if self.error.is_some() {
            return Err(self.error.unwrap());
        }

        Ok(self.tokens.to_vec())
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }
            '/' => {
                if self.match_char('/') {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                };
            }
            ' ' => {}
            '\t' => {}
            '\r' => {}
            '\n' => self.line += 1,
            '"' => self.read_string(),
            _ => {
                if is_digit(c) {
                    self.read_number();
                } else if is_alpha(c) {
                    self.read_identifier();
                } else {
                    self.error = Some(error::ScanError::UnknownToken);
                    super::error(self.line, format!("token not recognized {}", c).as_str());
                }
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(token_type));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source_len
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source
            .chars()
            .nth(self.current - 1)
            .expect("Failed to advance")
    }

    fn match_char(&mut self, char_to_match: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() != char_to_match {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }

    fn read_string(&mut self) {
        let mut char_value = '\0';
        while !self.is_at_end() {
            char_value = self.advance();

            if char_value == '"' {
                break;
            }
            if char_value == '\n' {
                self.line += 1;
            }
        }

        if char_value != '"' {
            self.error = Some(error::ScanError::UnterminatedString);
            super::error(self.line, "unterminated string!");
            return;
        }

        self.tokens
            .push(Token::new(TokenType::Literal(LiteralType::LoxString(
                String::from(&self.source[self.start..self.current]),
            ))));
    }

    fn read_number(&mut self) {
        let mut is_float = false;

        while !self.is_at_end() && is_digit(self.peek()) {
            self.advance();
        }

        if !self.is_at_end() && self.peek() == '.' {
            // consume dot
            self.advance();

            if self.is_at_end() || !is_digit(self.advance()) {
                self.error = Some(error::ScanError::NotValidNumber);
                super::error(self.line, "cannot end number with decimal");
                return;
            }

            is_float = true;
            // consume decimal portion of number
            while !self.is_at_end() && is_digit(self.peek()) {
                self.advance();
            }
        }

        let number_value = String::from(&self.source[self.start..self.current]);

        if is_float {
            let float_number_value = number_value.parse::<f64>().unwrap();

            self.tokens
                .push(Token::new(TokenType::Literal(LiteralType::Float(
                    float_number_value,
                ))));
        } else {
            let int_number_value = number_value.parse::<isize>().unwrap();

            self.tokens
                .push(Token::new(TokenType::Literal(LiteralType::Integer(
                    int_number_value,
                ))));
        }
    }

    fn read_identifier(&mut self) {
        while !self.is_at_end() && is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let identifier_value = String::from(&self.source[self.start..self.current]);
        if let Some(keyword_token) = self.keyword_map.get(identifier_value.as_str()) {
            self.tokens.push(Token::new(keyword_token.clone()));
        } else {
            self.tokens
                .push(Token::new(TokenType::Literal(LiteralType::Identifier(
                    identifier_value,
                ))));
        }
    }
}

fn is_alpha_numeric(check_param: char) -> bool {
    is_alpha(check_param) || is_digit(check_param)
}

fn is_digit(check_param: char) -> bool {
    ('0'..='9').contains(&check_param)
}

fn is_alpha(check_param: char) -> bool {
    ('A'..='Z').contains(&check_param) || ('a'..='z').contains(&check_param) || check_param == '_'
}
