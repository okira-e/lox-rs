use std::fmt::{Debug, Display};
use crate::literal_types::Literal;
use crate::token::Token;

pub trait ExprVisitor<'a, R: Display> {
    fn visit_assign_expr(&self, expr: &AssignExpression::<R>) -> Result<R, String>;
    fn visit_binary_expr(&self, expr: &BinaryExpression::<R>) -> Result<R, String>;
    fn visit_call_expr(&self, expr: &CallExpression::<R>) -> Result<R, String>;
    fn visit_get_expr(&self, expr: &GetExpression::<R>) -> Result<R, String>;
    fn visit_grouping_expr(&self, expr: &GroupingExpression::<R>) -> Result<R, String>;
    fn visit_literal_expr(&self, expr: &LiteralExpression) -> Result<R, String>;
    fn visit_logical_expr(&self, expr: &LogicalExpression::<R>) -> Result<R, String>;
    fn visit_set_expr(&self, expr: &SetExpression::<R>) -> Result<R, String>;
    fn visit_super_expr(&self, expr: &SuperExpression) -> Result<R, String>;
    fn visit_self_expr(&self, expr: &SelfExpression) -> Result<R, String>;
    fn visit_unary_expr(&self, expr: &UnaryExpression::<R>) -> Result<R, String>;
    fn visit_variable_expr(&self, expr: &VariableExpression) -> Result<R, String>;
}

/// A trait that represents an expression in the AST.
/// 'a is the lifetime of the expression.
pub trait Expr<R: Display> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, String>;
}

impl Display for dyn Expr<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", self);
    }
}

/// Assign expressions are expressions that assign a value to a variable.
/// ## Example
/// ```
/// var x = 1;
/// ```
pub struct AssignExpression<R> {
    pub name: Token,
    pub value: Box<dyn Expr::<R>>,
}

impl<'a, R: Display> Expr<R> for AssignExpression<R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, String> {
        return visitor.visit_assign_expr(self);
    }
}

/// Binary expressions are expressions that have a left and right side, and an operator in between.
/// ## Example
/// ```
/// 1 + 2
/// ```
pub struct BinaryExpression<R> {
    pub left: Box<dyn Expr::<R>>,
    pub operator: Token,
    pub right: Box<dyn Expr::<R>>,
}

impl<'a, R: Display> Expr<R> for BinaryExpression<R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, String> {
        return visitor.visit_binary_expr(self);
    }
}

/// Call expressions are expressions that call a function.
/// ## Example
/// ```
/// a_function();
/// ```
pub struct CallExpression<R> {
    pub callee: Box<dyn Expr::<R>>,
    pub paren: Token,
    pub arguments: Vec<Box<dyn Expr::<R>>>,
}

impl<'a, R: Display> Expr<R> for CallExpression<R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, String> {
        return visitor.visit_call_expr(self);
    }
}

/// Get expressions are expressions that get a property from an object.
/// ## Example
/// ```
/// object.property
/// ```
pub struct GetExpression<R> {
    pub object: Box<dyn Expr::<R>>,
    pub name: Token,
}

impl<'a, R: Display> Expr<R> for GetExpression<R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, String> {
        return visitor.visit_get_expr(self);
    }
}

/// Grouping expressions are expressions that group other expressions together.
/// ## Example
/// ```
/// (1 + 2)
/// ```
pub struct GroupingExpression<R> {
    pub expression: Box<dyn Expr::<R>>,
}

impl<'a, R: Display> Expr<R> for GroupingExpression<R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, String> {
        return visitor.visit_grouping_expr(self);
    }
}

/// Literal expressions are expressions that are literals.
/// ## Example
/// ```
/// 1
/// ```
pub struct LiteralExpression {
    pub value: Option<Literal>,
}

impl<'a, R: Display> Expr<R> for LiteralExpression {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, String> {
        return visitor.visit_literal_expr(self);
    }
}

/// Logical expressions are expressions that are logical.
/// ## Example
/// ```
/// true and false
/// ```
pub struct LogicalExpression<R> {
    pub left: Box<dyn Expr::<R>>,
    pub operator: Token,
    pub right: Box<dyn Expr::<R>>,
}

impl<'a, R: Display> Expr<R> for LogicalExpression<R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, String> {
        return visitor.visit_logical_expr(self);
    }
}

/// Set expressions are expressions that set a property on an object.
/// ## Example
/// ```
/// object.property = 1;
/// ```
pub struct SetExpression<R> {
    pub object: Box<dyn Expr::<R>>,
    pub name: Token,
    pub value: Box<dyn Expr::<R>>,
}

impl<'a, R: Display> Expr<R> for SetExpression<R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, String> {
        return visitor.visit_set_expr(self);
    }
}

/// Super expressions are expressions that call a method on the superclass.
/// ## Example
/// ```
/// super.method();
/// ```
pub struct SuperExpression {
    pub keyword: Token,
    pub method: Token,
}

impl<'a, R: Display> Expr<R> for SuperExpression {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, String> {
        return visitor.visit_super_expr(self);
    }
}

/// Self expressions are expressions that call a method on the current class.
pub struct SelfExpression {
    pub keyword: Token,
}

impl<'a, R: Display> Expr<R> for SelfExpression {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, String> {
        return visitor.visit_self_expr(self);
    }
}

/// Unary expressions are expressions that have a single side and an operator.
/// ## Example
/// ```
/// !true
/// ```
pub struct UnaryExpression<R> {
    pub operator: Token,
    pub right: Box<dyn Expr::<R>>,
}

impl<'a, R: Display> Expr<R> for UnaryExpression<R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, String> {
        return visitor.visit_unary_expr(self);
    }
}


/// Variable expressions are expressions that are variables.
/// ## Example
/// ```
/// var x = 1;
/// ```
pub struct VariableExpression {
    pub name: Token,
}

impl<'a, R: Display> Expr<R> for VariableExpression {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, String> {
        return visitor.visit_variable_expr(self);
    }
}
