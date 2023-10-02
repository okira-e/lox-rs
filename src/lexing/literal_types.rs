#[derive(Debug, Clone)]
/// LiteralTypes is an enum that represents the type of a literal. It's used to determine how to
/// parse the literal.
pub enum LiteralKinds<'a> {
    Number(f64),
    String(&'a str),
    Boolean(bool),
    Nil,
}

impl LiteralKinds<'_> {
    pub fn to_string(&self) -> String {
        match self {
            LiteralKinds::Number(n) => n.to_string(),
            LiteralKinds::String(s) => s.to_string(),
            LiteralKinds::Boolean(b) => b.to_string(),
            LiteralKinds::Nil => "None".into(),
        }
    }
}
