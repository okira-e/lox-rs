use crate::token_kinds::TokenKind;
use crate::literal_types::LiteralKinds;

/// Token is a struct that represents a token in a source file.
#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: &'a str,
    pub line: usize,
    pub literal: Option<LiteralKinds::<'a>>,
}

impl<'a> Token<'a> {
    pub fn to_string(&self) -> String {
        return format!("{}, type: {:?}, literal: {}", &self.lexeme, &self.kind, self.literal.as_ref().unwrap_or(&LiteralKinds::Nil).to_string());
    }
}
