use crate::language_error::Error;
use crate::expressions::{Expr};
use crate::literal::Literal;
use crate::report_error;
use crate::token_kinds::TokenKind;

pub fn interpret(expr: &Expr) {
    let result = evaluate(expr);

    return match result {
        Ok(result) => {
            println!("{}", result.to_string());
        }
        Err(err) => {
            report_error(&err);
        }
    };
}

/// Evaluates the given expression.
fn evaluate(expr: &Expr) -> Result<Literal, Error> {
    match expr {
        Expr::AssignExpression {
            name,
            value,
        } => {
            todo!();
        }
        Expr::BinaryExpression {
            left,
            operator,
            right,
        } => {
            let left = evaluate(left);
            let right = evaluate(right);

            return match operator.kind {
                TokenKind::Plus => {
                    match left {
                        Ok(Literal::Number(left)) => {
                            match right {
                                Ok(Literal::Number(right)) =>
                                    Ok(Literal::Number(left + right)),
                                Ok(Literal::String(right)) =>
                                    Ok(Literal::String(left.to_string() + &right)),
                                Err(err) => Err(err),
                                _ => Err(Error {
                                    msg: format!("Operands of \"{}\" must be two numbers or two strings", &operator.lexeme),
                                    line: Some(operator.line),
                                    column: 0,
                                    hint: None,
                                })
                            }
                        }
                        Ok(Literal::String(left)) => {
                            match right {
                                Ok(Literal::Number(right)) =>
                                    Ok(Literal::String(left + &right.to_string())),
                                Ok(Literal::String(right)) =>
                                    Ok(Literal::String(left + &right)),
                                Err(err) => Err(err),
                                _ => Err(Error {
                                    msg: format!("Operands of \"{}\" must be two numbers or two strings", &operator.lexeme),
                                    line: Some(operator.line),
                                    column: 0,
                                    hint: None,
                                })
                            }
                        }
                        Err(err) => Err(err),
                        _ => Err(Error {
                            msg: format!("Operands of \"{}\" must be two numbers or two strings", &operator.lexeme),
                            line: Some(operator.line),
                            column: 0,
                            hint: None,
                        })
                    }
                }
                TokenKind::Minus | TokenKind::Star | TokenKind::Slash
                => {
                    match left {
                        Ok(Literal::Number(left)) => {
                            match right {
                                Ok(Literal::Number(right)) => {
                                    match operator.kind {
                                        TokenKind::Minus => return Ok(Literal::Number(left - right)),
                                        TokenKind::Star => return Ok(Literal::Number(left * right)),
                                        TokenKind::Slash => {
                                            if right == 0f64 {
                                                return Err(Error {
                                                    msg: "Cannot divide by zero.".into(),
                                                    line: Some(operator.line),
                                                    column: 0,
                                                    hint: None,
                                                });
                                            }

                                            return Ok(Literal::Number(left / right));
                                        }
                                        _ => Ok(Literal::Number(0f64)) // This should never happen.
                                    }
                                }
                                Err(err) => Err(err),
                                _ => {
                                    Err(Error {
                                        msg: format!("Operands of \"{}\" must be two numbers", &operator.lexeme),
                                        line: Some(operator.line),
                                        column: 0,
                                        hint: None,
                                    })
                                }
                            }
                        }
                        Err(err) => Err(err),
                        _ => {
                            Err(Error {
                                msg: format!("Operands of \"{}\" must be two numbers", &operator.lexeme),
                                line: Some(operator.line),
                                column: 0,
                                hint: None,
                            })
                        }
                    }
                }
                TokenKind::BangEqual |
                TokenKind::EqualEqual
                => {
                    return match left {
                        Ok(Literal::Number(left)) => {
                            match right {
                                Ok(Literal::Number(right)) => Ok(Literal::Boolean(
                                    match operator.kind {
                                        TokenKind::BangEqual => left != right,
                                        TokenKind::EqualEqual => left == right,
                                        _ => false // This should never happen.
                                    }
                                )),
                                Ok(Literal::String(_right)) => Ok(Literal::Boolean(false)),
                                Ok(Literal::Boolean(_right)) => Ok(Literal::Boolean(false)),
                                Ok(Literal::Nil) => {
                                    Ok(Literal::Boolean(
                                        match operator.kind {
                                            TokenKind::BangEqual => true,
                                            TokenKind::EqualEqual => false,
                                            _ => false // This should never happen.
                                        }
                                    ))
                                }
                                Err(err) => Err(err),
                            }
                        }
                        Err(err) => Err(err),
                        Ok(Literal::String(left)) => {
                            match right {
                                Ok(Literal::String(right)) => {
                                    Ok(Literal::Boolean(
                                        match operator.kind {
                                            TokenKind::BangEqual => left != right,
                                            TokenKind::EqualEqual => left == right,
                                            _ => false // This should never happen.
                                        }
                                    ))
                                }
                                Ok(Literal::Number(_right)) => Ok(Literal::Boolean(false)),
                                Ok(Literal::Boolean(_right)) => Ok(Literal::Boolean(false)),
                                Ok(Literal::Nil) => {
                                    Ok(Literal::Boolean(
                                        match operator.kind {
                                            TokenKind::BangEqual => true,
                                            TokenKind::EqualEqual => false,
                                            _ => false // This should never happen.
                                        }
                                    ))
                                }
                                Err(err) => Err(err),
                            }
                        }
                        Ok(Literal::Boolean(left)) => {
                            match right {
                                Ok(Literal::Boolean(right)) => {
                                    Ok(Literal::Boolean(
                                        match operator.kind {
                                            TokenKind::BangEqual => left != right,
                                            TokenKind::EqualEqual => left == right,
                                            _ => false // This should never happen.
                                        }
                                    ))
                                }
                                Ok(Literal::Number(_right)) => Ok(Literal::Boolean(false)),
                                Ok(Literal::String(_right)) => Ok(Literal::Boolean(false)),
                                Ok(Literal::Nil) => Ok(Literal::Boolean(false)),
                                Err(err) => Err(err),
                            }
                        }
                        Ok(Literal::Nil) => {
                            match right {
                                Ok(Literal::Nil) => {
                                    Ok(Literal::Boolean(
                                        match operator.kind {
                                            TokenKind::BangEqual => false,
                                            TokenKind::EqualEqual => true,
                                            _ => false // This should never happen.
                                        }
                                    ))
                                }
                                Ok(Literal::Number(_right)) => {
                                    Ok(Literal::Boolean(
                                        match operator.kind {
                                            TokenKind::BangEqual => true,
                                            TokenKind::EqualEqual => false,
                                            _ => false // This should never happen.
                                        }
                                    ))
                                }
                                Ok(Literal::String(_right)) => {
                                    Ok(Literal::Boolean(
                                        match operator.kind {
                                            TokenKind::BangEqual => true,
                                            TokenKind::EqualEqual => false,
                                            _ => false // This should never happen.
                                        }
                                    ))
                                }
                                Ok(Literal::Boolean(_right)) => {
                                    Ok(Literal::Boolean(
                                        match operator.kind {
                                            TokenKind::BangEqual => false,
                                            TokenKind::EqualEqual => false,
                                            _ => false // This should never happen.
                                        }
                                    ))
                                }
                                Err(err) => Err(err),
                            }
                        }
                    };
                }
                TokenKind::Greater |
                TokenKind::GreaterEqual |
                TokenKind::Less |
                TokenKind::LessEqual
                => {
                    return match left {
                        Ok(Literal::Number(left)) => {
                            match right {
                                Ok(Literal::Number(right)) => Ok(Literal::Boolean(
                                    match operator.kind {
                                        TokenKind::Greater => left > right,
                                        TokenKind::GreaterEqual => left >= right,
                                        TokenKind::Less => left < right,
                                        TokenKind::LessEqual => left <= right,
                                        _ => false // This should never happen.
                                    }
                                )),
                                Err(err) => Err(err),
                                _ => {
                                    Err(Error {
                                        msg: format!("Operands of \"{}\" must be two numbers", &operator.lexeme),
                                        line: Some(operator.line),
                                        column: 0,
                                        hint: None,
                                    })
                                }
                            }
                        }
                        Err(err) => Err(err),
                        _ => {
                            Err(Error {
                                msg: format!("Operands of \"{}\" must be two numbers", &operator.lexeme),
                                line: Some(operator.line),
                                column: 0,
                                hint: None,
                            })
                        }
                    };
                }
                _ => todo!("Handle error")
            };
        }
        Expr::CallExpression {
            arguments,
            callee,
            ..
        } => {
            todo!();
        }
        Expr::GetExpression {
            object,
            name,
        } => {
            todo!();
        }
        Expr::GroupingExpression {
            expression,
        } => {
            return evaluate(expression);
        }
        Expr::LiteralExpression {
            value,
        } => {
            return match value {
                Some(value) => Ok(value.clone()),
                None => Err(Error {
                    msg: "Literal value missing.".into(),
                    line: None, // TODO: Find a value for this.
                    column: 0,
                    hint: None,
                })
            };
        }
        Expr::LogicalExpression {
            right,
            operator,
            left,
        } => {
            todo!();
        }
        Expr::SetExpression {
            value,
            object,
            name,
        } => {
            todo!();
        }
        Expr::SuperExpression {
            method,
            ..
        } => {
            todo!();
        }
        Expr::SelfExpression {
            ..
        } => {
            todo!();
        }
        Expr::UnaryExpression {
            operator,
            right,
        } => {
            let interpreted_right = evaluate(right);

            return match operator.kind {
                TokenKind::Minus => {
                    match interpreted_right {
                        Ok(Literal::Number(right)) => Ok(Literal::Number(-right)),
                        Err(err) => Err(err),
                        _ => Err(Error {
                            msg: format!("Operand of \"{}\" must be a number", &operator.lexeme),
                            line: Some(operator.line),
                            column: 0,
                            hint: None,
                        })
                    }
                }
                TokenKind::Bang => {
                    match interpreted_right {
                        Ok(Literal::Boolean(right)) => Ok(Literal::Boolean(!right)),
                        Err(err) => Err(err),
                        _ => Err(Error {
                            msg: format!("Operand of \"{}\" must be a boolean", &operator.lexeme),
                            line: Some(operator.line),
                            column: 0,
                            hint: None,
                        })
                    }
                }
                _ => todo!("Handle error")
            };
        }
        Expr::VariableExpression {
            name,
        } => {
            todo!();
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::token::Token;
    use super::*;

    #[test]
    fn binary_expressions() {
        let expr = Expr::BinaryExpression {
            left: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Number(1.into())),
            }),
            operator: Token {
                kind: TokenKind::Plus,
                lexeme: "+".into(),
                line: 0,
                column: 0,
                literal: None,
            },
            right: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Number(2.into())),
            }),
        };

        assert_eq!(evaluate(&expr).unwrap(), Literal::Number(3.into()));

        let expr = Expr::BinaryExpression {
            left: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Number(1.into())),
            }),
            operator: Token {
                kind: TokenKind::Minus,
                lexeme: "-".into(),
                line: 0,
                column: 0,
                literal: None,
            },
            right: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Number(2.into())),
            }),
        };

        assert_eq!(evaluate(&expr).unwrap(), Literal::Number((-1).into()));

        let expr = Expr::BinaryExpression {
            left: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Number(10.into())),
            }),
            operator: Token {
                kind: TokenKind::Star,
                lexeme: "*".into(),
                line: 0,
                column: 0,
                literal: None,
            },
            right: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Number(2.into())),
            }),
        };

        assert_eq!(evaluate(&expr).unwrap(), Literal::Number(20.into()));

        let expr = Expr::BinaryExpression {
            left: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Number(10.into())),
            }),
            operator: Token {
                kind: TokenKind::Slash,
                lexeme: "/".into(),
                line: 0,
                column: 0,
                literal: None,
            },
            right: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Number(2.into())),
            }),
        };

        assert_eq!(evaluate(&expr).unwrap(), Literal::Number(5.into()));

        let expr = Expr::BinaryExpression {
            left: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Number(10.into())),
            }),
            operator: Token {
                kind: TokenKind::Greater,
                lexeme: ">".into(),
                line: 0,
                column: 0,
                literal: None,
            },
            right: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Number(2.into())),
            }),
        };

        assert_eq!(evaluate(&expr).unwrap(), Literal::Boolean(true));

        let expr = Expr::BinaryExpression {
            left: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Number(10.into())),
            }),
            operator: Token {
                kind: TokenKind::GreaterEqual,
                lexeme: ">=".into(),
                line: 0,
                column: 0,
                literal: None,
            },
            right: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Number(2.into())),
            }),
        };

        assert_eq!(evaluate(&expr).unwrap(), Literal::Boolean(true));

        let expr = Expr::BinaryExpression {
            left: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Number(10.into())),
            }),
            operator: Token {
                kind: TokenKind::Less,
                lexeme: "<".into(),
                line: 0,
                column: 0,
                literal: None,
            },
            right: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Number(2.into())),
            }),
        };

        assert_eq!(evaluate(&expr).unwrap(), Literal::Boolean(false));

        let expr = Expr::BinaryExpression {
            left: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Number(10.into())),
            }),
            operator: Token {
                kind: TokenKind::LessEqual,
                lexeme: "<=".into(),
                line: 0,
                column: 0,
                literal: None,
            },
            right: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Number(2.into())),
            }),
        };

        assert_eq!(evaluate(&expr).unwrap(), Literal::Boolean(false));

        let expr = Expr::BinaryExpression {
            left: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Number(10.into())),
            }),
            operator: Token {
                kind: TokenKind::BangEqual,
                lexeme: "!=".into(),
                line: 0,
                column: 0,
                literal: None,
            },
            right: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Nil),
            }),
        };

        assert_eq!(evaluate(&expr).unwrap(), Literal::Boolean(true));

        let expr = Expr::BinaryExpression {
            left: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Number(10.into())),
            }),
            operator: Token {
                kind: TokenKind::EqualEqual,
                lexeme: "==".into(),
                line: 0,
                column: 0,
                literal: None,
            },
            right: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Nil),
            }),
        };

        assert_eq!(evaluate(&expr).unwrap(), Literal::Boolean(false));
    }

    #[test]
    fn unary_expressions() {
        let expr = Expr::UnaryExpression {
            operator: Token {
                kind: TokenKind::Minus,
                lexeme: "-".into(),
                line: 0,
                column: 0,
                literal: None,
            },
            right: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Number(1.into())),
            }),
        };

        assert_eq!(evaluate(&expr).unwrap(), Literal::Number((-1).into()));

        let expr = Expr::UnaryExpression {
            operator: Token {
                kind: TokenKind::Bang,
                lexeme: "!".into(),
                line: 0,
                column: 0,
                literal: None,
            },
            right: Box::new(Expr::LiteralExpression {
                value: Some(Literal::Boolean(true)),
            }),
        };

        assert_eq!(evaluate(&expr).unwrap(), Literal::Boolean(false));
    }
}