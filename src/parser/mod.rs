mod ast_printer;
mod ast_types;

use crate::token::*;
use ast_types::*;

pub struct Parser {
    tokens: Vec<Token>,
    tokens_len: usize,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            tokens_len: tokens.len(),
        }
    }

    fn expression(&self) -> Expr {
        return self.equality();
    }

    fn equality(&self) -> Expr {
        let mut expr = self.comparison();
        let match_tokens = vec![TokenType::Bang, TokenType::BangEqual];

        while self.token_match(match_tokens) {
            let operator = self.previous();
            let right = self.comparison();

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: *operator,
                right: Box::new(right),
            });
        }

        expr
    }

    fn comparison(&self) -> Expr {
        let mut expr = self.term();
        let match_tokens = vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];

        while self.token_match(match_tokens) {
            let operator = self.previous();
            let right = self.term();

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: *operator,
                right: Box::new(right),
            });
        }

        expr
    }

    fn term(&self) -> Expr {
        let mut expr = self.factor();
        let match_tokens = vec![TokenType::Minus, TokenType::Plus];

        while self.token_match(match_tokens) {
            let operator = self.previous();
            let right = self.factor();

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: *operator,
                right: Box::new(right),
            });
        }

        expr
    }

    fn factor(&self) -> Expr {
        let mut expr = self.unary();
        let match_tokens = vec![TokenType::Slash, TokenType::Star];

        while self.token_match(match_tokens) {
            let operator = self.previous();
            let right = self.unary();

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: *operator,
                right: Box::new(right),
            });
        }

        expr
    }

    fn unary(&self) -> Expr {
        let match_tokens = vec![TokenType::Bang, TokenType::Minus];

        while self.token_match(match_tokens) {
            let operator = self.previous();
            let right = self.unary();

            return Expr::Unary(Unary {
                operator: *operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&self) -> Expr {
        match self.peek().token_type {
            TokenType::False => return Expr::Literal(Literal(LiteralType::))
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens_len
    }

    fn previous(&self) -> &Token {
        self.tokens.iter().nth(self.current - 1).unwrap()
    }

    fn advance(&self) -> &Token {
        self.current += 1;
        self.tokens
            .iter()
            .nth(self.current - 1)
            .expect("Failed to advance")
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn token_match(&self, tokens_to_match: Vec<TokenType>) -> bool {
        if tokens_to_match
            .iter()
            .find(|&&token| token == self.peek().token_type)
            .is_some()
        {
            self.advance();
            return true;
        }

        false
    }
}
