use crate::token::{keyword, Token, TokenType};

/// Cut `source` into tokens. Returns everything it managed to scan alongside every
/// error it found; the caller decides whether to go on.
pub fn scan(source: &str) -> (Vec<Token>, Vec<String>) {
    let mut s = Scanner {
        src: source.chars().collect(),
        start: 0,
        current: 0,
        line: 1,
        tokens: Vec::new(),
        errors: Vec::new(),
    };
    s.run();
    (s.tokens, s.errors)
}

struct Scanner {
    src: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
    errors: Vec<String>,
}

impl Scanner {
    fn run(&mut self) {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            kind: TokenType::Eof,
            lexeme: String::new(),
            line: self.last_token_line,
        });
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add(TokenType::LParen),
            ')' => self.add(TokenType::RParen),
            '{' => self.add(TokenType::LBrace),
            '}' => self.add(TokenType::RBrace),
            ',' => self.add(TokenType::Comma),
            ';' => self.add(TokenType::Semicolon),
            '+' => self.add(TokenType::Plus),
            '-' => self.add(TokenType::Minus),
            '*' => self.add(TokenType::Star),
            '/' => {
                if self.matches('/') {
                    while !self.at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add(TokenType::Slash)
                }
            }
            '!' => {
                if self.matches('=') {
                    self.add(TokenType::BangEqual)
                } else {
                    self.add(TokenType::Bang)
                }
            }
            '=' => {
                if self.matches('=') {
                    self.add(TokenType::EqualEqual)
                } else {
                    self.add(TokenType::Equal)
                }
            }
            '>' => {
                if self.matches('=') {
                    self.add(TokenType::GreaterEqual)
                } else {
                    self.add(TokenType::Greater)
                }
            }
            '<' => {
                if self.matches('=') {
                    self.add(TokenType::LessEqual)
                } else {
                    self.add(TokenType::Less)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            _ if self.is_digit(c) => self.number(),
            _ if self.is_identifier_start(c) => self.identifier(),
            _ => self.error(self.line, "Character is not part of any token."),
        }
    }

    fn string(&mut self) {
        let start_line = self.line;

        while !self.at_end() {
            let c = self.advance();
            if c == '"' {
                self.add(TokenType::Str);
                return;
            }
            if c == '\n' {
                self.line += 1;
            }
        }

        self.error(start_line, "String is never closed.");
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add(TokenType::Number);

    }

    fn identifier(&mut self) {
        // TODO(you): scan an identifier, then decide whether it is a keyword; keyword() in
        //            token.rs does the lookup (1.2, 1.3).
        todo!("identifier")
    }

    // --- primitives ---------------------------------------------------------------

    fn at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn advance(&mut self) -> char {
        let c = self.src[self.current];
        self.current += 1;
        c
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.at_end() || self.src[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.src[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.src.len() {
            '\0'
        } else {
            self.src[self.current + 1]
        }
    }

    fn add(&mut self, kind: TokenType) {
        self.tokens.push(Token {
            kind,
            lexeme: self.src[self.start..self.current].iter().collect(),
            line: self.line,
        });
    }

    fn error(&mut self, line: usize, message: &str) {
        self.errors.push(format!("[line {}] Error: {}", line, message));
    }
}
