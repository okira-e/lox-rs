use crate::expressions::Expr;
use crate::literal::Literal;
use crate::stmt::Stmt;

pub fn print_ast(statements: &Vec<Stmt>) {
    println!("AST:");
    for statement in statements {
        let str = print_stmt(statement);
        println!("{}", str);
    }
    println!();
}

fn print_stmt(statement: &Stmt) -> String {
    return match statement {
        Stmt::AssignmentStmt { expression } => {
            return print_expr(expression).as_str().into();
        }
        Stmt::ExpressionStmt { expression } => {
            return print_expr(expression).as_str().into();
        }
        Stmt::PrintStmt { expression } => {
            return format!("print \"{}\"", print_expr(expression));
        }
        Stmt::VarDeclStmt { name, initializer } => {
            return format!("var {} = {}", name.lexeme, print_expr(initializer));
        }
        Stmt::BlockStmt { statements } => {
            let mut ret = String::new();

            for statement in statements {
                ret += print_stmt(statement).as_str();
                ret += "\n";
            }
            // Remove the last newline
            if ret.len() > 0 {
                ret.pop();
            }

            return ret;
        }
        Stmt::IfStmt { .. } => {
            return todo!();
        }
        Stmt::WhileStmt { .. } => {
            return todo!();
        }
        Stmt::FunctionStmt { .. } => {
            return todo!();
        }
        Stmt::ReturnStmt { .. } => {
            return todo!();
        }
        Stmt::ClassStmt { .. } => {
            return todo!();
        }
    };
}

fn print_expr(expr: &Expr) -> String {
    return match expr {
        Expr::AssignmentExpression { name, value } => {
            format!("= {} {}", name.lexeme, print_expr(value))
        }
        Expr::BinaryExpression {
            left,
            operator,
            right,
        } => {
            format!(
                "({} {} {})",
                operator.lexeme,
                print_expr(left),
                print_expr(right)
            )
        }
        Expr::CallExpression {
            arguments, callee, ..
        } => {
            format!(
                "call {} with {}",
                print_expr(callee),
                print_expr_vec(arguments)
            )
        }
        Expr::GetExpression { object, name } => {
            format!(".{} {}", print_expr(object), name.lexeme)
        }
        Expr::GroupingExpression { expression } => {
            format!("(group {})", print_expr(expression))
        }
        Expr::LiteralExpression { value } => {
            format!("{}", value.as_ref().unwrap_or(&Literal::Nil))
        }
        Expr::LogicalExpression {
            right,
            operator,
            left,
        } => {
            format!(
                "{} {} {}",
                operator.lexeme,
                print_expr(right),
                print_expr(left)
            )
        }
        Expr::SetExpression {
            value,
            object,
            name,
        } => {
            format!(
                "set {}.{} = {}",
                print_expr(object),
                name.lexeme,
                print_expr(value)
            )
        }
        Expr::SuperExpression { method, .. } => {
            format!("super.{}", method.lexeme)
        }
        Expr::SelfExpression { .. } => {
            format!("self")
        }
        Expr::UnaryExpression { operator, right } => {
            format!("({} {})", operator.lexeme, print_expr(right))
        }
        Expr::VarDeclExpression { name } => {
            format!("{}", name.lexeme)
        }
        Expr::VariableResolutionExpression { name } => {
            format!("{}", name.lexeme)
        }
    };
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
    fn test_expression() {
        let stmt = Stmt::ExpressionStmt {
            expression: Box::new(
                Expr::BinaryExpression {
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
                }
            ),
        };

        assert_eq!(print_stmt(&stmt), "(* (- 123) (group 45.67))");
    }

    #[test]
    fn test_print() {
        let stmt = Stmt::PrintStmt {
            expression: Box::new(
                Expr::LiteralExpression {
                    value: Some(Literal::String("Hello, world!".into())),
                }
            ),
        };

        assert_eq!(print_stmt(&stmt), "print \"Hello, world!\"");
    }

    #[test]
    fn test_var_decl() {
        let stmt = Stmt::VarDeclStmt {
            name: Token {
                kind: TokenKind::Identifier,
                lexeme: "a".into(),
                line: 1,
                column: 1,
                literal: None,
            },
            initializer: Expr::LiteralExpression {
                value: Some(Literal::Number(1.into())),
            },
        };

        assert_eq!(print_stmt(&stmt), "var a = 1");
    }

    #[test]
    fn test_assignment() {
        let stmt = Stmt::AssignmentStmt {
            expression: Box::new(
                Expr::AssignmentExpression {
                    name: Token {
                        kind: TokenKind::Identifier,
                        lexeme: "a".into(),
                        line: 1,
                        column: 1,
                        literal: None,
                    },
                    value: Box::new(
                        Expr::LiteralExpression {
                            value: Some(Literal::Number(1.into())),
                        }
                    ),
                }
            ),
        };

        assert_eq!(print_stmt(&stmt), "= a 1");
    }

    #[test]
    fn test_block() {
        let stmt = Stmt::BlockStmt {
            statements: vec![
                Stmt::VarDeclStmt {
                    name: Token {
                        kind: TokenKind::Identifier,
                        lexeme: "a".into(),
                        line: 1,
                        column: 1,
                        literal: None,
                    },
                    initializer: Expr::LiteralExpression {
                        value: Some(Literal::Number(1.into())),
                    },
                },
                Stmt::VarDeclStmt {
                    name: Token {
                        kind: TokenKind::Identifier,
                        lexeme: "b".into(),
                        line: 1,
                        column: 1,
                        literal: None,
                    },
                    initializer: Expr::LiteralExpression {
                        value: Some(Literal::Number(2.into())),
                    },
                },
            ],
        };

        assert_eq!(print_stmt(&stmt), "var a = 1\nvar b = 2");
    }
}
