use crate::lexing::token::Token;
use crate::lexing::token_kinds::TokenKind;

/// Lexer is responsible for scanning the source code and returning a vector of tokens and errors.
pub struct Lexer<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    errors: Vec<LexerError>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer {
        return Lexer {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors: Vec::new(),
        };
    }

    /// scan_tokens scans the source code and returns a vector of tokens.
    pub fn scan_tokens(&mut self) -> (&Vec<Token>, &Vec<LexerError>) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenKind::Eof, "".into(), self.line, "".into()));

        return (&self.tokens, &self.errors);
    }

    /// scan_token scans the current character and adds a new token to the tokens vector.
    /// If the character is not recognized, it adds an error to the errors vector.
    fn scan_token(&mut self) {
        let current_char = self.advance();

        match current_char {
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => (),
            '(' => self.add_token(TokenKind::LeftParen, None),
            ')' => self.add_token(TokenKind::RightParen, None),
            '{' => self.add_token(TokenKind::LeftBrace, None),
            '}' => self.add_token(TokenKind::RightBrace, None),
            ',' => self.add_token(TokenKind::Comma, None),
            '.' => self.add_token(TokenKind::Dot, None),
            '-' => self.add_token(TokenKind::Minus, None),
            '+' => self.add_token(TokenKind::Plus, None),
            ';' => self.add_token(TokenKind::Semicolon, None),
            '*' => self.add_token(TokenKind::Star, None),
            '!' => {
                // Check for the next character to see if it's a bang equal.
                // If it is, add a bang equal token & increment `current` to skip it, otherwise
                // add a bang token and continue.
                let kind = if self.match_char('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };

                self.add_token(kind, None);
            },
            '=' => {
                let kind = if self.match_char('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };

                self.add_token(kind, None);
            },
            '<' => {
                let kind = if self.match_char('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };

                self.add_token(kind, None);
            },
            '>' => {
                let kind = if self.match_char('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };

                self.add_token(kind, None);
            },
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenKind::Slash, None);
                }
            },
            _ => {
                self.errors.push(
                    LexerError {
                        line: self.line,
                        message: format!("Unrecognized character \"{}\" at line {}.", current_char, self.line),
                        hint: None,
                    }
                );
            }
        }
    }

    /// add_token adds a new token to the tokens vector.
    fn add_token(&mut self, kind: TokenKind, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(kind, text, self.line, literal.unwrap_or("".into())));
    }

    /// match_char checks if the next character is the expected character.
    /// If it is, it increments the current index and returns true, otherwise it returns false.
    /// This is useful for checking for multi-character tokens like `!=` or `==`.
    fn match_char(&mut self, expected_next: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected_next {
            return false;
        }

        self.current += 1;
        return true;
    }

    /// advance consumes the current character the parser's at and returns it.
    /// Then it increments the current index.
    fn advance(&mut self) -> char {
        let char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;

        return char;
    }

    /// peek returns the current character the parser's at without consuming it.
    /// If the parser is at the end of the source code, it returns the null character.
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.source.chars().nth(self.current).unwrap();
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }
}

#[derive(Debug)]
pub struct LexerError {
    pub line: usize,
    pub message: String,
    pub hint: Option<String>,
}