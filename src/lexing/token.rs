use crate::lexing::token_kinds::TokenKind;

/// Token is a struct that represents a token in a source file.
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize, literal: String) -> Token {
        Token {
            kind,
            lexeme,
            line,
            literal,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{:?} {} {}", self.kind, self.lexeme, self.literal);
    }
}
