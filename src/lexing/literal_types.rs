#[derive(Debug, Clone)]
/// LiteralTypes is an enum that represents the type of a literal. It's used to determine how to
/// parse the literal.
pub enum LiteralTypes {
    Number,
    String,
    Boolean,
    Nil,
}
