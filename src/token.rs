use crate::token_kinds::TokenKind;
use crate::literal_types::Literal;

/// Token is a struct that represents a token in a source file.
#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: &'a str,
    pub line: usize,
    pub literal: Option<Literal::<'a>>,
}

impl<'a> Token<'a> {
    pub fn to_string(&self) -> String {
        return format!("{}, type: {:?}, literal: {}", &self.lexeme, &self.kind, self.literal.as_ref().unwrap_or(&Literal::Nil).to_string());
    }
}
