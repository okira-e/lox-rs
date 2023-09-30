use crate::lexing::token_kinds::TokenKind;
use crate::lexing::literal_types::LiteralTypes;

/// Token is a struct that represents a token in a source file.
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub literal: String,
    pub literal_type: Option<LiteralTypes>,
}

impl Token {
    pub fn to_string(&self) -> String {
        return format!("{:?} {} {}", self.kind, self.lexeme, self.literal);
    }
}
