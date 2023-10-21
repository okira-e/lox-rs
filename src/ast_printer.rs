use crate::expressions::Expr;
use crate::literal::Literal;


pub fn print_ast(ast: &Expr) -> String {
    return print_expr(ast);
}

fn print_expr(expr: &Expr) -> String {
    return match expr {
        Expr::AssignExpression {
            name,
            value,
        } => {
            format!("={} {}", name.lexeme, print_expr(value))
        },
        Expr::BinaryExpression {
            left,
            operator,
            right,
        } => {
            format!("({} {} {})", operator.lexeme, print_expr(left), print_expr(right))
        },
        Expr::CallExpression {
            arguments,
            callee,
            ..
        } => {
            format!(
                "call {} with {}",
                print_expr(callee),
                print_expr_vec(arguments)
            )
        },
        Expr::GetExpression {
            object,
            name,
        } => {
            format!(".{} {}", print_expr(object), name.lexeme)
        },
        Expr::GroupingExpression {
            expression,
        } => {
            format!("(group {})", print_expr(expression))
        },
        Expr::LiteralExpression {
            value,
        } => {
            format!("{}", value.as_ref().unwrap_or(&Literal::Nil))
        },
        Expr::LogicalExpression {
            right,
            operator,
            left,
        } => {
            format!("{} {} {}", operator.lexeme, print_expr(right), print_expr(left))
        },
        Expr::SetExpression {
            value,
            object,
            name,
        } => {
            format!("set {}.{} = {}", print_expr(object), name.lexeme, print_expr(value))
        },
        Expr::SuperExpression {
            method,
            ..
        } => {
            format!("super.{}", method.lexeme)
        },
        Expr::SelfExpression {
            ..
        } => {
            format!("self")
        },
        Expr::UnaryExpression {
            operator,
            right,
        } => {
            format!("({} {})", operator.lexeme, print_expr(right))
        },
        Expr::VariableExpression {
            name,
        } => {
            format!("{}", name.lexeme)
        },
    }
}

fn print_expr_vec(expressions: &Vec<Box<Expr>>) -> String {
    let mut expr_str = String::new();

    for expression in expressions {
        expr_str += print_expr(expression).as_str();
    }

    return expr_str;
}

#[cfg(test)]
mod tests {
    use crate::token::Token;
    use crate::token_kinds::TokenKind;
    use super::*;

    #[test]
    fn test_print_ast() {
        let expr = Expr::BinaryExpression {
            left: Box::new(
                Expr::UnaryExpression {
                    operator: Token {
                        kind: TokenKind::Minus,
                        lexeme: "-".into(),
                        line: 1,
                        column: 1,
                        literal: None,
                    },
                    right: Box::new(
                        Expr::LiteralExpression {
                            value: Some(Literal::Number(123.into())),
                        }
                    ),
                }
            ),
            operator: Token {
                kind: TokenKind::Star,
                lexeme: "*".into(),
                line: 1,
                column: 2,
                literal: None,
            },
            right: Box::new(
                Expr::GroupingExpression {
                    expression: Box::new(
                        Expr::LiteralExpression {
                            value: Some(Literal::Number(45.67.into())),
                        }
                    ),
                }
            ),
        };

        assert_eq!(print_ast(&expr), "(* (- 123) (group 45.67))");
    }
}