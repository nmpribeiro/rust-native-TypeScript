use super::scanner::Scanner;
use super::token::{Token, TokenType};
use std::mem;

pub struct Parser {
    pub scanner: Scanner,
    pub previous: Token,
    pub current: Token,

    pub had_error: bool,
    pub panic_mode: bool,
}

impl Parser {
    pub fn new(code: &str) -> Parser {
        Parser {
            scanner: Scanner::new(code.to_string()),

            previous: Token::generic_token(TokenType::Error),
            current: Token::generic_token(TokenType::Error),

            had_error: false,
            panic_mode: false,
        }
    }

    pub fn get_current(&mut self) -> Token {
        self.current.clone()
    }

    pub fn get_previous(&mut self) -> Token {
        self.previous.clone()
    }

    pub fn advance(&mut self) {
        loop {
            let tok = self.scanner.scan_token();

            self.previous = mem::replace(&mut self.current, tok);

            if let TokenType::Error = self.current.t_type {
                self.error(self.current.lexeme.clone());
            } else {
                break;
            }
        }
    }

    pub fn match_next(&mut self, t_type: TokenType) -> bool {
        if !self.check(t_type) {
            return false;
        }
        self.advance();
        true
    }

    pub fn consume(&mut self, t_type: TokenType, message: &str) {
        if t_type == self.current.t_type {
            self.advance();
        } else {
            self.error(message.to_string());
        }
    }

    pub fn check(&mut self, t_type: TokenType) -> bool {
        t_type == self.current.t_type
    }

    pub fn error(&mut self, message: String) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;

        eprint!("[Line {}] Error", self.current.line);
        match self.current.t_type {
            TokenType::EOF => eprint!(" at end"),
            TokenType::Error => (),
            _ => eprint!(" at line {}", self.current.line),
        }
        eprintln!(": {}", message);

        self.had_error = true;
    }

    pub fn synchronize(&mut self) {
        if !self.panic_mode {
            return;
        }
        self.panic_mode = false;

        while self.current.t_type != TokenType::EOF {
            if self.previous.t_type == TokenType::Semicolon {
                return;
            }

            match self.current.t_type {
                TokenType::Class
                | TokenType::Function
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => (),
            }

            self.advance();
        }
    }
}
