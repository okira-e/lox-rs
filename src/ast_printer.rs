use std::fmt::Display;
use crate::expressions::{
    Expr,
    ExprVisitor,
    BinaryExpression,
    GroupingExpression,
    LiteralExpression,
    UnaryExpression,
    VariableExpression,
    AssignExpression,
    CallExpression,
    GetExpression,
    LogicalExpression,
    SetExpression,
    SuperExpression,
    SelfExpression,
};

/// A struct that implements the ExprVisitor trait. To print the AST.
struct AstPrinter<'a> {
    _phantom: std::marker::PhantomData<&'a str>,
}

impl<'a> AstPrinter<'a> {
    /// Create a new AstPrinter.
    fn new() -> Self {
        return AstPrinter {
            _phantom: std::marker::PhantomData
        };
    }

    /// Helper function to print an expression.
    fn print(&self, expr: &'a dyn Expr<&str>) -> Result<String, &'static str> {
        // `self` here is the visitor of type AstPrinter
        // and `expr` is the expression to print.
        return expr.accept(self);
    }

    /// Helper function to wrap an expression in parentheses.
    /// It takes a name and a slice of expressions and wraps them all up in parentheses.
    /// ## Abstract Example
    /// ```
    /// paranthesize("+", &[LiteralExpression::new(1), LiteralExpression::new(2)])
    /// ```
    /// would return
    /// ```
    /// "(+ 1 2)"
    /// ```
    fn paranthesize<R: Display>(&self, name: &str, exprs: &'a [&dyn Expr::<R>]) -> Result<String, &'static str> {
        let mut expr_str: String = "(".into();
        expr_str += name;

        for expr in exprs {
            expr_str += " ";
            expr_str += expr.accept(self)?.as_str();
        }
        expr_str += ")";

        return Ok(expr_str);
    }
}

impl<'a, R: Display> ExprVisitor<'a, R> for AstPrinter<'a> {
    fn visit_assign_expr(&self, expr: &AssignExpression<R>) -> Result<String, &'static str> {
        return Ok("AssignExpression".into());
    }

    fn visit_binary_expr(&self, expr: &BinaryExpression<R>) -> Result<String, &'static str> {
        return self.paranthesize(expr.operator.lexeme, &[expr.left.as_ref(), expr.right.as_ref()]);
    }

    fn visit_call_expr(&self, expr: &CallExpression<R>) -> Result<String, &'static str> {
        return Ok("CallExpression".into());
    }

    fn visit_get_expr(&self, expr: &GetExpression<R>) -> Result<String, &'static str> {
        return Ok("GetExpression".into());
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpression<R>) -> Result<String, &'static str> {
        return self.paranthesize("group", &[expr.expression.as_ref()]);
    }

    fn visit_literal_expr(&self, expr: &LiteralExpression) -> Result<String, &'static str> {
        return match &expr.value {
            Some(value) => Ok(value.to_string().into()),
            None => Ok("nil".into()),
        };
    }

    fn visit_logical_expr(&self, expr: &LogicalExpression<R>) -> Result<String, &'static str> {
        return Ok("LogicalExpression".into());
    }

    fn visit_set_expr(&self, expr: &SetExpression<R>) -> Result<String, &'static str> {
        return Ok("SetExpression".into());
    }

    fn visit_super_expr(&self, expr: &SuperExpression) -> Result<String, &'static str> {
        return Ok("SuperExpression".into());
    }

    fn visit_self_expr(&self, expr: &SelfExpression) -> Result<String, &'static str> {
        return Ok("SelfExpression".into());
    }

    fn visit_unary_expr(&self, expr: &UnaryExpression<R>) -> Result<String, &'static str> {
        return self.paranthesize(expr.operator.lexeme, &[expr.right.as_ref()]);
    }

    fn visit_variable_expr(&self, expr: &VariableExpression) -> Result<String, &'static str> {
        return Ok("VariableExpression".into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;
    use crate::token_kinds::TokenKind;
    use crate::literal_types::Literal;

    #[test]
    fn test_ast_printer() {
        let expr = BinaryExpression {
            left: Box::new(
                UnaryExpression::<&'static str> {
                    operator: Token {
                        kind: TokenKind::Minus,
                        lexeme: "-",
                        line: 1,
                        literal: None,
                    },
                    right: Box::new(
                        LiteralExpression {
                            value: Some(Literal::Number(123.into())),
                        }
                    ),
                }
            ),
            operator: Token {
                kind: TokenKind::Star,
                lexeme: "*",
                line: 1,
                literal: None,
            },
            right: Box::new(
                GroupingExpression {
                    expression: Box::new(
                        LiteralExpression {
                            value: Some(Literal::Number(45.67.into())),
                        }
                    ),
                }
            ),
        };

        let ast_printer = AstPrinter::new();

        assert_eq!(ast_printer.print(&expr).unwrap(), "(* (- 123) (group 45.67))");
    }

    // #[test]
    // fn test_ast_printer() {
    //     let ast_printer = AstPrinter { _phantom: std::marker::PhantomData };
    //
    //     let expression = BinaryExpression {
    //         left: Box::new(UnaryExpression {
    //             operator: Token::new(TokenKind::Minus, "-", None, 1),
    //             right: Box::new(LiteralExpression::new(Some(Literal::Number(123.0)))),
    //         }),
    //         operator: Token::new(TokenKind::Star, "*", None, 1),
    //         right: Box::new(GroupingExpression {
    //             expression: Box::new(LiteralExpression::new(Some(Literal::Number(45.67)))),
    //         }),
    //     };
    //
    //     assert_eq!(ast_printer.print(&expression).unwrap(), "(* (- 123) (group 45.67))");
    // }
}