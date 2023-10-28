use crate::language_error::Error;
use crate::literal::Literal;
use crate::token::Token;
use crate::token_kinds::TokenKind;

/// Tokenizer is responsible for scanning the source code and returning a vector of tokens and errors.
/// The Tokenizer stores errors and returns them in a vector alongside the tokens.
pub struct Tokenizer<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start_of_lexeme: usize,
    current_char: usize,
    line: usize,
    column: usize,
    // NOTE: Set but not currently used.
    errors: Vec<Error>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Tokenizer {
        return Tokenizer {
            source,
            tokens: Vec::new(),
            start_of_lexeme: 0,
            current_char: 0,
            line: 1,
            column: 0,
            errors: Vec::new(),
        };
    }

    /// scans the source code for tokens.
    pub fn scan_tokens(&mut self) -> (&Vec<Token>, &Vec<Error>) {
        while !self.is_at_end() {
            self.start_of_lexeme = self.current_char;
            self.column = self.start_of_lexeme + 1;

            self.scan_token();
        }

        self.tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: "".into(),
            line: self.line,
            column: self.column,
            literal: None,
        });

        return (&self.tokens, &self.errors);
    }

    /// scan_token scans the current character and adds a new token to the tokens vector.
    /// If the character is not recognized, it adds an error to the errors vector.
    fn scan_token(&mut self) {
        let current_char = self.advance();

        match current_char {
            '\n' => {
                self.line += 1;
            }
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
                while self.peek() != '"' && !self.is_at_end() && self.peek() != '\n' {
                    self.advance();
                }

                // If we're at the end of the source code before a closing '"', add an error.
                if self.peek() == '\n' || self.is_at_end() {
                    self.errors.push(Error::new(
                        "Unterminated string.".into(),
                        Some(self.line),
                        self.column,
                        None,
                    ));

                    return;
                }

                // Otherwise, we've found the closing '"', so we can add the string literal.
                self.advance();

                // The value of the string literal is the substring of the source code from the
                // start index to the current index.
                let value =
                    self.source[self.start_of_lexeme + 1..self.current_char - 1].to_string();
                self.add_token(TokenKind::String, Some(Literal::String(value)));
            }
            _ => {
                // Handle numbers and identifiers.
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

                    let value = self.source[self.start_of_lexeme..self.current_char]
                        .parse::<f64>()
                        .unwrap_or_else(|err| {
                            self.errors.push(Error::new(
                                format!("Error parsing number: {}.", err),
                                Some(self.line),
                                self.column,
                                None,
                            ));

                            return 0f64;
                        });

                    self.add_token(TokenKind::Number, Some(Literal::Number(value)));
                } else if current_char.is_alphabetic() {
                    // Identify if the typed keyword is reserved or an identifier.

                    while !(self.peek() == '\n'
                            || self.peek() == ' '
                            || self.is_at_end()
                            || self.peek() == '\t'
                            || self.peek() == '(' // This is for function calls.
                            || self.peek() == ')'
                            || self.peek() == '{' // For names before scopes and blocks.
                            || self.peek() == '}'
                            || self.peek() == ','
                            || self.peek() == '.'
                            || self.peek() == ';')
                    {
                        self.advance();
                    }

                    let value: &str = self.source[self.start_of_lexeme..self.current_char].into();

                    let kind = self.match_keyword(value);

                    self.add_token(kind, None);
                } else {
                    self.errors.push(Error::new(
                        format!("Unrecognized character \"{}\".", current_char),
                        Some(self.line),
                        self.column,
                        None,
                    ));
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
    /// `literal` param is an optional string that represents the literal value of the token. It can be
    /// None if the token doesn't have a literal value. Or it can be a string for string literals
    /// and number literals.
    fn add_token(&mut self, kind: TokenKind, literal: Option<Literal>) {
        // The text of the token is the substring of the source code from the start index to the
        // current index.
        let text = &self.source[self.start_of_lexeme..self.current_char];
        self.tokens.push(Token {
            kind,
            lexeme: text.to_string(),
            line: self.line,
            column: self.column,
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

        let next_char = self
            .source
            .chars()
            .nth(self.current_char)
            .unwrap_or_else(|| {
                panic!(
                    "No character at index {}. Last read character was {}.",
                    self.current_char,
                    self.source.chars().nth(self.current_char - 1).unwrap()
                );
            });
        if next_char != expected_next {
            return false;
        }

        self.current_char += 1;
        return true;
    }

    /// advance consumes the current character the Tokenizer's at and returns it.
    /// Then it increments the current index.
    fn advance(&mut self) -> char {
        let char = self
            .source
            .chars()
            .nth(self.current_char)
            .unwrap_or_else(|| {
                println!(
                    "No character at index {}. Last read character was {}.",
                    self.current_char,
                    self.source.chars().nth(self.current_char - 1).unwrap()
                );
                std::process::exit(1);
            });
        self.current_char += 1;

        return char;
    }

    /// peek returns the current character the Tokenizer's at without consuming it.
    /// If the Tokenizer is at the end of the source code, it returns the null character.
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.source.chars().nth(self.current_char).unwrap();
    }

    /// peek_next returns the next character the Tokenizer's at without consuming it.
    fn peek_next(&self) -> char {
        if self.current_char + 1 >= self.source.len() {
            return '\0';
        }

        return self.source.chars().nth(self.current_char + 1).unwrap();
    }

    /// Checks if the Tokenizer is at the end of the source code.
    fn is_at_end(&self) -> bool {
        return self.current_char >= self.source.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod scanning {
        use super::*;

        // Keep in mind that the tokenizer adds an EOF token to the end of the tokens vector.

        #[test]
        fn scan_tokens() {
            let input = "\
            var name = \"Lox\";

            if name == \"Lox\" {
                print \"Hello, \" + name + \"!\";
            } else {
                print \"Hello, world!\";
            }
            ";
            let mut tokenizer = Tokenizer::new(input);

            let (tokens, errors) = tokenizer.scan_tokens();

            assert_eq!(tokens.len(), 25);
            assert_eq!(errors.len(), 0);
        }

        #[test]
        fn comments() {
            let input = "() // This is a comment )";
            let mut tokenizer = Tokenizer::new(input);

            let (tokens, errors) = tokenizer.scan_tokens();
            assert_eq!(tokens.len(), 3);
            assert_eq!(errors.len(), 0);
        }

        #[test]
        fn whitespace() {
            let input = " ( ) ".into();
            let mut tokenizer = Tokenizer::new(input);

            let (tokens, errors) = tokenizer.scan_tokens();
            assert_eq!(tokens.len(), 3);
            assert_eq!(errors.len(), 0);
        }

        #[test]
        fn strings() {
            let input = "\"Hello, world!\"".into();
            let mut tokenizer = Tokenizer::new(input);

            let (tokens, errors) = tokenizer.scan_tokens();
            assert_eq!(tokens.len(), 2);
            assert_eq!(errors.len(), 0);
        }

        #[test]
        fn unterminated_string() {
            let input = "\"Hello, world!))";
            let mut tokenizer = Tokenizer::new(input);

            let (tokens, errors) = tokenizer.scan_tokens();

            assert_eq!(tokens.len(), 1);
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].line, Some(1));
        }

        #[test]
        fn unterminated_string_multiple_lines_src() {
            let input = "\"Hello, world!))\n var x = 1;";
            let mut tokenizer = Tokenizer::new(input);

            let (tokens, errors) = tokenizer.scan_tokens();

            assert_eq!(tokens.len(), 6);
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].line, Some(1));
        }
    }

    mod handling_errors {
        use crate::tokenizer::Tokenizer;

        #[test]
        fn unexpected_token() {
            let input = "var x = 5;^"; // The ^ is the unexpected token.
            let mut tokenizer = Tokenizer::new(input);

            let (tokens, errors) = tokenizer.scan_tokens();

            assert_eq!(tokens.len(), 6);
            assert_eq!(errors.len(), 1);
        }

        #[test]
        fn multiple_errors() {
            let input = "(*^) (+^) (^)";
            let mut tokenizer = Tokenizer::new(input);

            let (tokens, errors) = tokenizer.scan_tokens();

            assert_eq!(tokens.len(), 9);
            assert_eq!(errors.len(), 3);
        }

        #[test]
        fn error_message() {
            let input = "(*^)";
            let mut tokenizer = Tokenizer::new(input);

            let (tokens, errors) = tokenizer.scan_tokens();
            assert_eq!(tokens.len(), 4);
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].msg, String::from("Unrecognized character \"^\"."));
        }
    }
}
