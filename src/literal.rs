use std::borrow::Cow;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
/// An enum that represents the type of a literal. It's used to determine how to
/// parse the literal.
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Literal {
    pub fn to_string(&self) -> String {
        match self {
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => s.to_string(),
            Literal::Boolean(b) => b.to_string(),
            Literal::Nil => "nil".into(),
        }
    }
}

impl std::ops::Add for Literal {
    type Output = Result<Self, Cow<'static, str>>;

    fn add<'a>(self, rhs: Self) -> Self::Output {
        let err_msg: Cow<'static, str> =
            format!("Operands of type {} and {} cannot be added.",
                    self.to_string(),
                    rhs.to_string()).into();

        match self {
            Literal::Number(left) => {
                return match rhs {
                    Literal::Number(right) => Ok(Literal::Number(left + right)),
                    Literal::String(right) => Ok(Literal::String(left.to_string() + &right)),
                    _ => Err(err_msg)
                };
            }
            Literal::String(left) => {
                return match rhs {
                    Literal::Number(right) => Ok(Literal::String(left + &right.to_string())),
                    Literal::String(right) => Ok(Literal::String(left + &right)),
                    _ => Err(err_msg)
                };
            }
            _ => Err(err_msg)
        }
    }
}

impl std::ops::Sub for Literal {
    type Output = Result<Self, Cow<'static, str>>;

    fn sub(self, rhs: Self) -> Self::Output {
        let err_msg =
            format!("Operands of type {} and {} cannot be subtracted.",
                    self.to_string(),
                    rhs.to_string()).into();

        match self {
            Literal::Number(left) => {
                return match rhs {
                    Literal::Number(right) => Ok(Literal::Number(left - right)),
                    _ => Err(err_msg),
                };
            }
            _ => Err(err_msg),
        }
    }
}

impl std::ops::Mul for Literal {
    type Output = Result<Self, Cow<'static, str>>;

    fn mul(self, rhs: Self) -> Self::Output {
        let err_msg =
            format!("Operands of type {} and {} cannot be multiplied.", self.to_string(), rhs.to_string()).into();

        match self {
            Literal::Number(left) => {
                return match rhs {
                    Literal::Number(right) => Ok(Literal::Number(left * right)),
                    _ => Err(err_msg),
                };
            }
            _ => Err(err_msg),
        }
    }
}

impl std::ops::Div for Literal {
    type Output = Result<Self, Cow<'static, str>>;

    fn div(self, rhs: Self) -> Self::Output {
        let invalid_types_err_msg =
            format!("Operands of type {} and {} cannot be divided.", self.to_string(), rhs.to_string()).into();

        match self {
            Literal::Number(left) => {
                return match rhs {
                    Literal::Number(right) => {
                        if right == 0f64 {
                            return Err("Cannot divide by zero.".into());
                        }

                        return Ok(Literal::Number(left / right));
                    }
                    _ => Err(invalid_types_err_msg),
                };
            }
            _ => Err(invalid_types_err_msg),
        }
    }
}

impl std::ops::Rem for Literal {
    type Output = Result<Self, Cow<'static, str>>;

    fn rem(self, rhs: Self) -> Self::Output {
        let err_msg =
            format!("Operands of type {} and {} cannot be divided for remainder.",
                    self.to_string(),
                    rhs.to_string()).into();

        match self {
            Literal::Number(left) => {
                return match rhs {
                    Literal::Number(right) => Ok(Literal::Number(left % right)),
                    _ => Err(err_msg),
                };
            }
            _ => Err(err_msg),
        }
    }
}

impl std::ops::Neg for Literal {
    type Output = Result<Self, Cow<'static, str>>;

    fn neg(self) -> Self::Output {
        let err_msg =
            format!("Operand of type {} cannot be negated with \"-\".", self.to_string()).into();

        match self {
            Literal::Number(n) => Ok(Literal::Number(-n)),
            _ => Err(err_msg),
        }
    }
}

impl std::ops::Not for Literal {
    type Output = Result<Self, Cow<'static, str>>;

    fn not(self) -> Self::Output {
        let err_msg =
            format!("Operand of type {} cannot be negated with \"!\".", self.to_string()).into();

        match self {
            Literal::Boolean(b) => Ok(Literal::Boolean(!b)),
            _ => Err(err_msg),
        }
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Literal::Number(left) => {
                return match other {
                    Literal::Number(right) => left.partial_cmp(right),
                    _ => None,
                };
            }
            Literal::String(left) => {
                return match other {
                    Literal::String(right) => left.partial_cmp(right),
                    _ => None,
                };
            }
            _ => None,
        }
    }
}

impl Eq for Literal {}

impl Ord for Literal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Literal::Number(left) => {
                return match other {
                    Literal::Number(right) => {
                        left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    _ => std::cmp::Ordering::Equal,
                };
            }
            Literal::String(left) => {
                return match other {
                    Literal::String(right) => left.cmp(right),
                    _ => std::cmp::Ordering::Equal,
                };
            }
            _ => std::cmp::Ordering::Equal,
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.to_string());
    }
}