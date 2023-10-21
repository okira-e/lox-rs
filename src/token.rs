use std::fmt::Display;
use crate::token_kinds::TokenKind;
use crate::literal::Literal;

/// Token is a struct that represents a token in a source file.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    /// The actual text of the token. Can be the variable name for identifiers.
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
    pub literal: Option<Literal>,
}

impl Token {
    pub fn to_string(&self) -> String {
        return format!("{}, type: {:?}, literal: {}", &self.lexeme, &self.kind, self.literal.clone()
            .unwrap_or(Literal::Nil).to_string());
    }
}

impl AsRef<Token> for Token {
    fn as_ref(&self) -> &Token {
        return self;
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.to_string());
    }
}