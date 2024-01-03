use crate::expressions::Expr;
use crate::language_error::Error;
use crate::literal::Literal;
use crate::report_error;
use crate::stmt::Stmt;
use crate::token_kinds::TokenKind;
use std::collections::HashMap;
use std::io::Write;

type Env = Vec<HashMap<String, Literal>>;

pub fn interpret(statements: &Vec<Stmt>) {
    let mut env = Env::new();
    env.push(HashMap::new());

    for statement in statements {
        let mut do_break = false;
        execute(Box::new(statement), &mut env).unwrap_or_else(|err| {
            report_error(&err);
            do_break = true;
        });

        if do_break {
            break;
        }
    }
}

/// Executes the given statement.
fn execute(stmt: Box<&Stmt>, env: &mut Env) -> Result<(), Error> {
    return match stmt.as_ref() {
        Stmt::VarDeclStmt { name, initializer } => {
            let value = evaluate(initializer, env);
            if value.is_err() {
                return Err(value.err().unwrap());
            }

            if get_symbol_in_scope(env, &name.lexeme).is_some() {
                return Err(Error {
                    msg: format!("Variable \"{}\" already declared.", name.lexeme),
                    line: Some(name.line),
                    column: 0,
                    hint: None,
                });
            }

            add_symbol_to_current_scope(env, name.clone().lexeme, value.unwrap());
            return Ok(());
        }
        Stmt::AssignmentStmt {
            // FIX: `a = b = 5;` is not currently allowed.
            expression,
        } => {
            if let Expr::AssignmentExpression { name, value } = expression.as_ref() {
                if !get_symbol_in_scope(env, &name.lexeme).is_some() {
                    return Err(Error {
                        msg: format!("Assignment of undeclared variable \"{}\".", name.lexeme),
                        line: Some(name.line),
                        column: 0,
                        hint: None,
                    });
                }

                let value = evaluate(value, env);
                if value.is_err() {
                    return Err(value.err().unwrap());
                }

                add_symbol_to_current_scope(env, name.clone().lexeme, value.unwrap());

                return Ok(());
            } else {
                return Err(Error {
                    msg: format!("Invalid assignment."),
                    line: Some(0),
                    column: 0,
                    hint: None,
                });
            }
        }
        Stmt::BlockStmt { statements } => {
            env.push(HashMap::new());

            for statement in statements {
                execute(Box::new(statement), env)?;
            }

            env.pop();

            return Ok(());
        }
        Stmt::ClassStmt {
            methods,
            name,
            superclass,
        } => {
            return todo!();
        }
        Stmt::ExpressionStmt { expression } => {
            return match evaluate(expression, env) {
                Ok(_) => {
                    return Ok(());
                }
                Err(err) => {
                    return Err(err);
                }
            };
        }
        Stmt::FunctionStmt { name, params, body } => {
            return todo!();
        }
        Stmt::IfStmt {
            condition,
            then_branch,
            else_branch,
        } => {
            let bool_condition = evaluate(condition, env)?;

            let mut success = false;

            match bool_condition {
                Literal::Number(val) => {
                    if val == 0f64 {
                        success = false;
                    } else {
                        success = true;
                    }
                }
                Literal::String(val) => {
                    if val == "".to_string() {
                        success = false;
                    } else {
                        success = true;
                    }
                }
                Literal::Boolean(val) => {
                    success = val;
                }
                Literal::Nil => {
                    success = false;
                }
            }

            if success {
                execute(Box::new(&**then_branch), env)?;
            } else {
                if let Some(else_body) = else_branch {
                    execute(Box::new(*&else_body), env)?;
                }
            }

            return Ok(());
        }
        Stmt::PrintStmt { expression } => {
            let mut stdout = std::io::stdout();

            let value = evaluate(expression, env);
            if value.is_err() {
                return Err(value.err().unwrap());
            }

            /* return */
            match stdout.write(format!("{}\n", value.unwrap().to_string()).as_ref()) {
                Ok(_) => Ok(()),
                Err(_) => {
                    return Err(Error {
                        msg: format!("Error writing to stdout"),
                        line: None,
                        column: 0,
                        hint: None,
                    });
                }
            }
        }
        Stmt::ReturnStmt { keyword, value } => {
            return todo!();
        }
        Stmt::WhileStmt { condition, body } => {
            return todo!();
        }
        Stmt::None { err } => {
            return Err(Error {
                msg: err.to_owned(),
                line: None,
                column: 0,
                hint: None,
            });
        }
    };
}

/// Evaluates the given expression.
fn evaluate(expr: &Expr, env: &mut Env) -> Result<Literal, Error> {
    match expr {
        Expr::AssignmentExpression { name: _name, value } => {
            return evaluate(value, env);
        }
        Expr::BinaryExpression {
            left,
            operator,
            right,
        } => {
            let left = evaluate(left, env);
            let right = evaluate(right, env);

            return match operator.kind {
                TokenKind::Plus => match left {
                    Ok(Literal::Number(left)) => match right {
                        Ok(Literal::Number(right)) => Ok(Literal::Number(left + right)),
                        Ok(Literal::String(right)) => {
                            Ok(Literal::String(left.to_string() + &right))
                        }
                        Err(err) => Err(err),
                        _ => Err(Error {
                            msg: format!(
                                "Operands of \"{}\" must be two numbers or two strings.",
                                &operator.lexeme
                            ),
                            line: Some(operator.line),
                            column: 0,
                            hint: None,
                        }),
                    },
                    Ok(Literal::String(left)) => match right {
                        Ok(Literal::Number(right)) => {
                            Ok(Literal::String(left + &right.to_string()))
                        }
                        Ok(Literal::String(right)) => Ok(Literal::String(left + &right)),
                        Err(err) => Err(err),
                        _ => Err(Error {
                            msg: format!(
                                "Operands of \"{}\" must be two numbers or two strings.",
                                &operator.lexeme
                            ),
                            line: Some(operator.line),
                            column: 0,
                            hint: None,
                        }),
                    },
                    Err(err) => Err(err),
                    _ => Err(Error {
                        msg: format!(
                            "Operands of \"{}\" must be two numbers or two strings.",
                            &operator.lexeme
                        ),
                        line: Some(operator.line),
                        column: 0,
                        hint: None,
                    }),
                },
                TokenKind::Minus | TokenKind::Star | TokenKind::Slash => {
                    match left {
                        Ok(Literal::Number(left)) => {
                            match right {
                                Ok(Literal::Number(right)) => {
                                    match operator.kind {
                                        TokenKind::Minus => {
                                            return Ok(Literal::Number(left - right));
                                        }
                                        TokenKind::Star => {
                                            return Ok(Literal::Number(left * right));
                                        }
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
                                        _ => Ok(Literal::Number(0f64)), // This should never happen.
                                    }
                                }
                                Err(err) => Err(err),
                                _ => Err(Error {
                                    msg: format!(
                                        "Operands of \"{}\" must be two numbers.",
                                        &operator.lexeme
                                    ),
                                    line: Some(operator.line),
                                    column: 0,
                                    hint: None,
                                }),
                            }
                        }
                        Err(err) => Err(err),
                        _ => Err(Error {
                            msg: format!(
                                "Operands of \"{}\" must be two numbers.",
                                &operator.lexeme
                            ),
                            line: Some(operator.line),
                            column: 0,
                            hint: None,
                        }),
                    }
                }
                TokenKind::BangEqual | TokenKind::EqualEqual => {
                    return match left {
                        Ok(Literal::Number(left)) => {
                            match right {
                                Ok(Literal::Number(right)) => {
                                    Ok(Literal::Boolean(match operator.kind {
                                        TokenKind::BangEqual => left != right,
                                        TokenKind::EqualEqual => left == right,
                                        _ => false, // This should never happen.
                                    }))
                                }
                                Ok(Literal::String(_right)) => Ok(Literal::Boolean(false)),
                                Ok(Literal::Boolean(_right)) => Ok(Literal::Boolean(false)),
                                Ok(Literal::Nil) => {
                                    Ok(Literal::Boolean(match operator.kind {
                                        TokenKind::BangEqual => true,
                                        TokenKind::EqualEqual => false,
                                        _ => false, // This should never happen.
                                    }))
                                }
                                Err(err) => Err(err),
                            }
                        }
                        Err(err) => Err(err),
                        Ok(Literal::String(left)) => {
                            match right {
                                Ok(Literal::String(right)) => {
                                    Ok(Literal::Boolean(match operator.kind {
                                        TokenKind::BangEqual => left != right,
                                        TokenKind::EqualEqual => left == right,
                                        _ => false, // This should never happen.
                                    }))
                                }
                                Ok(Literal::Number(_right)) => Ok(Literal::Boolean(false)),
                                Ok(Literal::Boolean(_right)) => Ok(Literal::Boolean(false)),
                                Ok(Literal::Nil) => {
                                    Ok(Literal::Boolean(match operator.kind {
                                        TokenKind::BangEqual => true,
                                        TokenKind::EqualEqual => false,
                                        _ => false, // This should never happen.
                                    }))
                                }
                                Err(err) => Err(err),
                            }
                        }
                        Ok(Literal::Boolean(left)) => {
                            match right {
                                Ok(Literal::Boolean(right)) => {
                                    Ok(Literal::Boolean(match operator.kind {
                                        TokenKind::BangEqual => left != right,
                                        TokenKind::EqualEqual => left == right,
                                        _ => false, // This should never happen.
                                    }))
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
                                    Ok(Literal::Boolean(match operator.kind {
                                        TokenKind::BangEqual => false,
                                        TokenKind::EqualEqual => true,
                                        _ => false, // This should never happen.
                                    }))
                                }
                                Ok(Literal::Number(_right)) => {
                                    Ok(Literal::Boolean(match operator.kind {
                                        TokenKind::BangEqual => true,
                                        TokenKind::EqualEqual => false,
                                        _ => false, // This should never happen.
                                    }))
                                }
                                Ok(Literal::String(_right)) => {
                                    Ok(Literal::Boolean(match operator.kind {
                                        TokenKind::BangEqual => true,
                                        TokenKind::EqualEqual => false,
                                        _ => false, // This should never happen.
                                    }))
                                }
                                Ok(Literal::Boolean(_right)) => {
                                    Ok(Literal::Boolean(match operator.kind {
                                        TokenKind::BangEqual => false,
                                        TokenKind::EqualEqual => false,
                                        _ => false, // This should never happen.
                                    }))
                                }
                                Err(err) => Err(err),
                            }
                        }
                    };
                }
                TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::Less
                | TokenKind::LessEqual => {
                    return match left {
                        Ok(Literal::Number(left)) => {
                            match right {
                                Ok(Literal::Number(right)) => {
                                    Ok(Literal::Boolean(match operator.kind {
                                        TokenKind::Greater => left > right,
                                        TokenKind::GreaterEqual => left >= right,
                                        TokenKind::Less => left < right,
                                        TokenKind::LessEqual => left <= right,
                                        _ => false, // This should never happen.
                                    }))
                                }
                                Err(err) => Err(err),
                                _ => Err(Error {
                                    msg: format!(
                                        "Operands of \"{}\" must be two numbers.",
                                        &operator.lexeme
                                    ),
                                    line: Some(operator.line),
                                    column: 0,
                                    hint: None,
                                }),
                            }
                        }
                        Err(err) => Err(err),
                        _ => Err(Error {
                            msg: format!(
                                "Operands of \"{}\" must be two numbers.",
                                &operator.lexeme
                            ),
                            line: Some(operator.line),
                            column: 0,
                            hint: None,
                        }),
                    };
                }
                _ => todo!("Handle error"),
            };
        }
        Expr::VariableResolutionExpression { name } => {
            return match get_symbol_in_scope(env, &name.lexeme) {
                Some(value) => Ok(value.clone()),
                None => {
                    return Err(Error {
                        msg: format!("Usage of undeclared variable \"{}\".", name.lexeme),
                        line: Some(name.line),
                        column: 0,
                        hint: None,
                    });
                }
            };
        }
        Expr::CallExpression {
            arguments, callee, ..
        } => {
            todo!();
        }
        Expr::GetExpression { object, name } => {
            todo!();
        }
        Expr::GroupingExpression { expression } => {
            return evaluate(expression, env);
        }
        Expr::LiteralExpression { value } => {
            return match value {
                Some(value) => Ok(value.clone()),
                None => Ok(Literal::Nil),
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
        Expr::SuperExpression { method, .. } => {
            todo!();
        }
        Expr::SelfExpression { .. } => {
            todo!();
        }
        Expr::UnaryExpression { operator, right } => {
            let interpreted_right = evaluate(right, env);

            return match operator.kind {
                TokenKind::Minus => match interpreted_right {
                    Ok(Literal::Number(right)) => Ok(Literal::Number(-right)),
                    Err(err) => Err(err),
                    _ => Err(Error {
                        msg: format!("Operand of \"{}\" must be a number.", &operator.lexeme),
                        line: Some(operator.line),
                        column: 0,
                        hint: None,
                    }),
                },
                TokenKind::Bang => match interpreted_right {
                    Ok(Literal::Boolean(right)) => Ok(Literal::Boolean(!right)),
                    Err(err) => Err(err),
                    _ => Err(Error {
                        msg: format!("Operand of \"{}\" must be a boolean.", &operator.lexeme),
                        line: Some(operator.line),
                        column: 0,
                        hint: None,
                    }),
                },
                _ => todo!("Handle error"),
            };
        }
        Expr::VarDeclExpression { name } => {
            todo!();
        }
    }
}

/// Evaluates the given variable name.
fn get_symbol_in_scope<'a>(env: &'a Env, name: &String) -> Option<&'a Literal> {
    let mut i = if env.len() == 0 { 0 } else { env.len() - 1 };
    while i >= 0 {
        if env[i].contains_key(name) {
            return env[i].get(name);
        }

        if i == 0 {
            break;
        }

        i -= 1;
    }

    return None;
}

/// Adds the given symbol to the current scope.
fn add_symbol_to_current_scope(env: &mut Env, name: String, value: Literal) {
    let i = env.len() - 1;
    env[i].insert(name, value);
}

#[cfg(test)]
mod tests {
    use crate::token::Token;

    use super::*;

    mod evaluate_expressions_tests {
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

            assert_eq!(
                evaluate(&expr, &mut Env::new()).unwrap(),
                Literal::Number(3.into())
            );

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

            assert_eq!(
                evaluate(&expr, &mut Env::new()).unwrap(),
                Literal::Number((-1).into())
            );

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

            assert_eq!(
                evaluate(&expr, &mut Env::new()).unwrap(),
                Literal::Number(20.into())
            );

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

            assert_eq!(
                evaluate(&expr, &mut Env::new()).unwrap(),
                Literal::Number(5.into())
            );

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

            assert_eq!(
                evaluate(&expr, &mut Env::new()).unwrap(),
                Literal::Boolean(true)
            );

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

            assert_eq!(
                evaluate(&expr, &mut Env::new()).unwrap(),
                Literal::Boolean(true)
            );

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

            assert_eq!(
                evaluate(&expr, &mut Env::new()).unwrap(),
                Literal::Boolean(false)
            );

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

            assert_eq!(
                evaluate(&expr, &mut Env::new()).unwrap(),
                Literal::Boolean(false)
            );

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

            assert_eq!(
                evaluate(&expr, &mut Env::new()).unwrap(),
                Literal::Boolean(true)
            );

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

            assert_eq!(
                evaluate(&expr, &mut Env::new()).unwrap(),
                Literal::Boolean(false)
            );
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

            assert_eq!(
                evaluate(&expr, &mut Env::new()).unwrap(),
                Literal::Number((-1).into())
            );

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

            assert_eq!(
                evaluate(&expr, &mut Env::new()).unwrap(),
                Literal::Boolean(false)
            );
        }
    }

    mod execute_statements_tests {
        use super::*;

        #[test]
        fn var_decl_statement() {
            let stmt = Stmt::VarDeclStmt {
                name: Token {
                    kind: TokenKind::Identifier,
                    lexeme: "a".into(),
                    line: 0,
                    column: 0,
                    literal: None,
                },
                initializer: Expr::LiteralExpression {
                    value: Some(Literal::Number(1.into())),
                },
            };

            let mut env = Env::new();
            env.push(HashMap::new());

            execute(Box::new(&stmt), &mut env).unwrap();

            assert_eq!(
                get_symbol_in_scope(&env, &"a".into()).unwrap(),
                &Literal::Number(1.into())
            );
        }
    }
}
