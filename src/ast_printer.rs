use crate::expressions::Expr;
use crate::literal_types::Literal;


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

// use std::fmt::Display;
// use crate::expressions::{
//     Expr,
//     ExprVisitor,
//     BinaryExpression,
//     GroupingExpression,
//     LiteralExpression,
//     UnaryExpression,
//     VariableExpression,
//     AssignExpression,
//     CallExpression,
//     GetExpression,
//     LogicalExpression,
//     SetExpression,
//     SuperExpression,
//     SelfExpression,
// };
//
// /// A struct that implements the ExprVisitor trait. To print the AST.
// pub struct AstPrinter<'a> {
//     _phantom: std::marker::PhantomData<&'a str>,
// }
//
// impl<'a> AstPrinter<'a> {
//     /// Create a new AstPrinter.
//     pub fn new() -> Self {
//         return AstPrinter {
//             _phantom: std::marker::PhantomData
//         };
//     }
//
//     /// Print an expression.
//     pub fn print(&self, expr: &'a dyn Expr<String>) -> Result<String, &'static str> {
//         // `self` here is the visitor of type AstPrinter
//         // and `expr` is the expression to print.
//         return expr.accept(self);
//     }
//
//     /// Helper function to wrap an expression in parentheses.
//     /// It takes a name and a slice of expressions and wraps them all up in parentheses.
//     /// ## Abstract Example
//     /// ```
//     /// paranthesize("+", &[LiteralExpression::new(1), LiteralExpression::new(2)])
//     /// ```
//     /// would return
//     /// ```
//     /// "(+ 1 2)"
//     /// ```
//     fn paranthesize(&self, name: String, exprs: &'a [&dyn Expr::<String>]) -> Result<String, &'static str> {
//         let mut expr_str: String = "(".into();
//         expr_str += name.as_str();
//
//         for expr in exprs {
//             expr_str += " ";
//             expr_str += expr.accept(self)?.as_str();
//         }
//         expr_str += ")";
//
//         return Ok(expr_str);
//     }
// }
//
// impl<'a> ExprVisitor<'a, String> for AstPrinter<'a> {
//     fn visit_assign_expr(&self, expr: &AssignExpression<String>) -> Result<String, &'static str> {
//         return Ok("AssignExpression".into());
//     }
//
//     fn visit_binary_expr(&self, expr: &BinaryExpression<String>) -> Result<String, &'static str> {
//         return self.paranthesize(expr.operator.lexeme.clone(), &[expr.left.as_ref(), expr.right.as_ref()]);
//     }
//
//     fn visit_call_expr(&self, expr: &CallExpression<String>) -> Result<String, &'static str> {
//         return Ok("CallExpression".into());
//     }
//
//     fn visit_get_expr(&self, expr: &GetExpression<String>) -> Result<String, &'static str> {
//         return Ok("GetExpression".into());
//     }
//
//     fn visit_grouping_expr(&self, expr: &GroupingExpression<String>) -> Result<String, &'static str> {
//         return self.paranthesize("group".into(), &[expr.expression.as_ref()]);
//     }
//
//     fn visit_literal_expr(&self, expr: &LiteralExpression) -> Result<String, &'static str> {
//         return match &expr.value {
//             Some(value) => Ok(value.to_string().into()),
//             None => Ok("nil".into()),
//         };
//     }
//
//     fn visit_logical_expr(&self, expr: &LogicalExpression<String>) -> Result<String, &'static str> {
//         return Ok("LogicalExpression".into());
//     }
//
//     fn visit_set_expr(&self, expr: &SetExpression<String>) -> Result<String, &'static str> {
//         return Ok("SetExpression".into());
//     }
//
//     fn visit_super_expr(&self, expr: &SuperExpression) -> Result<String, &'static str> {
//         return Ok("SuperExpression".into());
//     }
//
//     fn visit_self_expr(&self, expr: &SelfExpression) -> Result<String, &'static str> {
//         return Ok("SelfExpression".into());
//     }
//
//     fn visit_unary_expr(&self, expr: &UnaryExpression<String>) -> Result<String, &'static str> {
//         return self.paranthesize(expr.operator.lexeme.clone(), &[expr.right.as_ref()]);
//     }
//
//     fn visit_variable_expr(&self, expr: &VariableExpression) -> Result<String, &'static str> {
//         return Ok("VariableExpression".into());
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::token::Token;
//     use crate::token_kinds::TokenKind;
//     use crate::literal_types::Literal;
//
//     #[test]
//     fn test_ast_printer() {
//         let expr: BinaryExpression<String> = BinaryExpression {
//             left: Box::new(
//                 UnaryExpression {
//                     operator: Token {
//                         kind: TokenKind::Minus,
//                         lexeme: "-".into(),
//                         line: 1,
//                         column: 1,
//                         literal: None,
//                     },
//                     right: Box::new(
//                         LiteralExpression {
//                             value: Some(Literal::Number(123.into())),
//                         }
//                     ),
//                 }
//             ),
//             operator: Token {
//                 kind: TokenKind::Star,
//                 lexeme: "*".into(),
//                 line: 1,
//                 column: 2,
//                 literal: None,
//             },
//             right: Box::new(
//                 GroupingExpression {
//                     expression: Box::new(
//                         LiteralExpression {
//                             value: Some(Literal::Number(45.67.into())),
//                         }
//                     ),
//                 }
//             ),
//         };
//
//         let ast_printer = AstPrinter::new();
//
//         assert_eq!(ast_printer.print(&expr).unwrap(), "(* (- 123) (group 45.67))");
//     }
// }