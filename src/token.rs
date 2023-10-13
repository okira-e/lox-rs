use crate::token_kinds::TokenKind;
use crate::literal_types::Literal;

/// Token is a struct that represents a token in a source file.
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub literal: Option<Literal>,
}

impl Token {
    pub fn to_string(&self) -> String {
        return format!("{}, type: {:?}, literal: {}", &self.lexeme, &self.kind, self.literal.clone().unwrap_or(Literal::Nil).to_string());
    }
}

impl AsRef<Token> for Token {
    fn as_ref(&self) -> &Token {
        return self;
    }
}