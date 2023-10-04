use crate::literal_types::LiteralKinds;
use crate::token::Token;
use crate::token_kinds::TokenKind;

/// Tokenizer is responsible for scanning the source code and returning a vector of tokens and errors.
pub struct Tokenizer<'a> {
    source: &'a str,
    tokens: Vec<Token::<'a>>,
    start_of_lexeme: usize,
    current_char: usize,
    line: usize,
    errors: Vec<TokenizerError>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Tokenizer {
        return Tokenizer {
            source,
            tokens: Vec::new(),
            start_of_lexeme: 0,
            current_char: 0,
            line: 1,
            errors: Vec::new(),
        };
    }

    /// scan_tokens scans the source code and returns a vector of tokens.
    pub fn scan_tokens(&mut self) -> (&Vec<Token>, &Vec<TokenizerError>) {
        while !self.is_at_end() {
            self.start_of_lexeme = self.current_char;
            self.scan_token();
        }

        self.tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: "".into(),
            line: self.line,
            literal: None,
        });

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
            }
            '=' => {
                let kind = if self.match_char('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };

                self.add_token(kind, None);
            }
            '<' => {
                let kind = if self.match_char('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };

                self.add_token(kind, None);
            }
            '>' => {
                let kind = if self.match_char('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };

                self.add_token(kind, None);
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenKind::Slash, None);
                }
            }
            '"' => {
                // As long as the next character isn't a double quote and we're not at the end
                // of the source code, keep advancing.
                while self.peek() != '"' && !self.is_at_end() {
                    // If we encounter a newline, increment the line number.
                    if self.peek() == '\n' {
                        self.line += 1;
                    }

                    self.advance();
                }

                // If we're at the end of the source code before a closing '"', add an error.
                if self.is_at_end() {
                    self.errors.push(TokenizerError {
                        line: self.line,
                        message: "Unterminated string.".into(),
                        hint: None,
                    });

                    return;
                }

                // Otherwise, we've found the closing '"', so we can add the string literal.
                self.advance();

                // The value of the string literal is the substring of the source code from the
                // start index to the current index.
                let value = self.source[self.start_of_lexeme + 1..self.current_char - 1].into();
                self.add_token(TokenKind::String, Some(LiteralKinds::String(value)));
            }
            _ => { // Handle numbers and identifiers.
                if current_char.is_numeric() {
                    // If it's a digit, scan and add a number token.
                    while self.peek().is_numeric() {
                        self.advance();
                    }

                    if self.peek() == '.' && self.peek_next().is_numeric() {
                        self.advance();

                        while self.peek().is_numeric() {
                            self.advance();
                        }
                    }

                    let value = self.source[self.start_of_lexeme..self.current_char].parse::<f64>().unwrap_or_else(|err| {
                        self.errors.push(TokenizerError {
                            line: self.line,
                            message: format!("Error parsing number: {}", err),
                            hint: None,
                        });

                        return 0f64;
                    });

                    self.add_token(TokenKind::Number, Some(LiteralKinds::Number(value)));
                } else if current_char.is_alphabetic() {
                    // Identify if the typed keyword is reserved or an identifier.

                    while self.peek() != '\n' && self.peek() != ' ' && !self.is_at_end() {
                        self.advance();
                    }

                    let value: &str = self.source[self.start_of_lexeme..self.current_char].into();

                    let kind = self.match_keyword(value);

                    self.add_token(kind, None);
                }
            }
        }
    }

    /// match_keyword checks if the given word is a keyword.
    /// If it is, it returns the corresponding token kind, otherwise it returns the identifier
    fn match_keyword(&self, word: &str) -> TokenKind {
        match word {
            "and" => TokenKind::And,
            "class" => TokenKind::Class,
            "else" => TokenKind::Else,
            "false" => TokenKind::False,
            "for" => TokenKind::For,
            "fun" => TokenKind::Fun,
            "if" => TokenKind::If,
            "nil" => TokenKind::Nil,
            "or" => TokenKind::Or,
            "print" => TokenKind::Print,
            "return" => TokenKind::Return,
            "super" => TokenKind::Super,
            "self" => TokenKind::Self_,
            "true" => TokenKind::True,
            "var" => TokenKind::Var,
            "while" => TokenKind::While,
            _ => TokenKind::Identifier,
        }
    }

    /// add_token adds a new token to the tokens vector.
    /// literal is an optional string that represents the literal value of the token. It can be
    /// None if the token doesn't have a literal value. Or it can be a string for string literals
    /// and number literals.
    fn add_token(&mut self, kind: TokenKind, literal: Option<LiteralKinds<'a>>) {
        // The text of the token is the substring of the source code from the start index to the
        // current index.
        let text = &self.source[self.start_of_lexeme..self.current_char];
        self.tokens.push(Token {
            kind,
            lexeme: text,
            line: self.line,
            literal,
        });
    }

    /// match_char checks if the next character is the expected character.
    /// If it is, it increments the current index and returns true, otherwise it returns false.
    /// This is useful for checking for multi-character tokens like `!=` or `==`.
    fn match_char(&mut self, expected_next: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let next_char = self.source.chars().nth(self.current_char).unwrap_or_else(|| {
            // This should never happen because we check if we're at the end of the source code
            // before calling this function.
            panic!("No character at index {}. Last read character was {}", self.current_char, self.source.chars().nth(self.current_char - 1).unwrap());
        });
        if next_char != expected_next {
            return false;
        }

        self.current_char += 1;
        return true;
    }

    /// advance consumes the current character the parser's at and returns it.
    /// Then it increments the current index.
    fn advance(&mut self) -> char {
        let char = self.source.chars().nth(self.current_char).unwrap_or_else(|| {
            panic!("No character at index {}. Last read character was {}", self.current_char, self.source.chars().nth(self.current_char - 1).unwrap());
        });
        self.current_char += 1;

        return char;
    }

    /// peek returns the current character the parser's at without consuming it.
    /// If the parser is at the end of the source code, it returns the null character.
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.source.chars().nth(self.current_char).unwrap();
    }

    /// peek_next returns the next character the parser's at without consuming it.
    fn peek_next(&self) -> char {
        if self.current_char + 1 >= self.source.len() {
            return '\0';
        }

        return self.source.chars().nth(self.current_char + 1).unwrap();
    }

    fn is_at_end(&self) -> bool {
        return self.current_char >= self.source.len();
    }

    // fn add_error(&mut self, value: &str, hint: Option<String>) {
    //     self.errors.push(
    //         TokenizerError {
    //             line: self.line,
    //             message: format!("Unrecognized {} \"{}\" at line {}.", if value.len() == 1 {
    //                 "character"
    //             } else {
    //                 "identifier"
    //             }, value, self.line),
    //             hint,
    //         }
    //     );
    // }
}

#[derive(Debug)]
pub struct TokenizerError {
    pub line: usize,
    pub message: String,
    pub hint: Option<String>,
}