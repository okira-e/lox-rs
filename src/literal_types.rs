#[derive(Debug, Clone)]
/// LiteralTypes is an enum that represents the type of a literal. It's used to determine how to
/// parse the literal.
pub enum Literal<'a> {
    Number(f64),
    String(&'a str),
    Boolean(bool),
    Nil,
}

impl Literal<'_> {
    pub fn to_string(&self) -> String {
        match self {
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => s.to_string(),
            Literal::Boolean(b) => b.to_string(),
            Literal::Nil => "None".into(),
        }
    }
}
