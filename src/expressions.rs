use std::fmt::{Debug, Display};
use crate::literal_types::Literal;
use crate::token::Token;

pub trait ExprVisitor<'a, R: Display + Debug> {
    fn visit_assign_expr(&self, expr: &AssignExpression::<R>) -> Result<R, &'static str>;
    fn visit_binary_expr(&self, expr: &BinaryExpression::<R>) -> Result<R, &'static str>;
    fn visit_call_expr(&self, expr: &CallExpression::<R>) -> Result<R, &'static str>;
    fn visit_get_expr(&self, expr: &GetExpression::<R>) -> Result<R, &'static str>;
    fn visit_grouping_expr(&self, expr: &GroupingExpression::<R>) -> Result<R, &'static str>;
    fn visit_literal_expr(&self, expr: &LiteralExpression) -> Result<R, &'static str>;
    fn visit_logical_expr(&self, expr: &LogicalExpression::<R>) -> Result<R, &'static str>;
    fn visit_set_expr(&self, expr: &SetExpression::<R>) -> Result<R, &'static str>;
    fn visit_super_expr(&self, expr: &SuperExpression) -> Result<R, &'static str>;
    fn visit_self_expr(&self, expr: &SelfExpression) -> Result<R, &'static str>;
    fn visit_unary_expr(&self, expr: &UnaryExpression::<R>) -> Result<R, &'static str>;
    fn visit_variable_expr(&self, expr: &VariableExpression) -> Result<R, &'static str>;
}

/// A trait that represents an expression in the AST.
pub trait Expr<R: Display + Debug> : Display + Debug {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, &'static str>;
}

/// Assign expressions are expressions that assign a value to a variable.
/// ## Example
/// ```
/// var x = 1;
/// ```
#[derive(Debug)]
pub struct AssignExpression<R> {
    pub name: Token,
    pub value: Box<dyn Expr::<R>>,
}

impl<'a, R: Display + Debug> Expr<R> for AssignExpression<R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, &'static str> {
        return visitor.visit_assign_expr(self);
    }
}

impl<'a, R: Display + Debug> Display for AssignExpression<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", self);
    }
}

/// Binary expressions are expressions that have a left and right side, and an operator in between.
/// ## Example
/// ```
/// 1 + 2
/// ```
#[derive(Debug)]
pub struct BinaryExpression<R> {
    pub left: Box<dyn Expr::<R>>,
    pub operator: Token,
    pub right: Box<dyn Expr::<R>>,
}

impl<'a, R: Display + Debug> Expr<R> for BinaryExpression<R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, &'static str> {
        return visitor.visit_binary_expr(self);
    }
}

impl<'a, R: Display + Debug> Display for BinaryExpression<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "({} {} {})", self.operator.lexeme, self.left, self.right);
    }
}


/// Call expressions are expressions that call a function.
/// ## Example
/// ```
/// a_function();
/// ```
#[derive(Debug)]
pub struct CallExpression<R> {
    pub callee: Box<dyn Expr::<R>>,
    pub paren: Token,
    pub arguments: Vec<Box<dyn Expr::<R>>>,
}

impl<'a, R: Display + Debug> Expr<R> for CallExpression<R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, &'static str> {
        return visitor.visit_call_expr(self);
    }
}

impl<'a, R: Display + Debug> Display for CallExpression<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", self);
    }
}

/// Get expressions are expressions that get a property from an object
/// ## Example
/// ```
/// object.property
/// ```
#[derive(Debug)]
pub struct GetExpression<R> {
    pub object: Box<dyn Expr::<R>>,
    pub name: Token,
}

impl<'a, R: Display + Debug> Expr<R> for GetExpression<R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, &'static str> {
        return visitor.visit_get_expr(self);
    }
}


impl<'a, R: Display + Debug> Display for GetExpression<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", self);
    }
}

/// Grouping expressions are expressions that group other expressions together.
/// ## Example
/// ```
/// (1 + 2)
/// ```
#[derive(Debug)]
pub struct GroupingExpression<R> {
    pub expression: Box<dyn Expr::<R>>,
}

impl<'a, R: Display + Debug> Expr<R> for GroupingExpression<R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, &'static str> {
        return visitor.visit_grouping_expr(self);
    }
}

impl<'a, R: Display + Debug> Display for GroupingExpression<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", self.expression);
    }
}

/// Literal expressions are expressions that are literals.
/// ## Example
/// ```
/// 1
/// ```
#[derive(Debug)]
pub struct LiteralExpression {
    pub value: Option<Literal>,
}

impl<'a, R: Display + Debug> Expr<R> for LiteralExpression {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, &'static str> {
        return visitor.visit_literal_expr(self);
    }
}

impl<'a> Display for LiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", self.value.clone().unwrap_or(Literal::Nil));
    }
}

/// Logical expressions are expressions that are logical.
/// ## Example
/// ```
/// true and false
/// ```
#[derive(Debug)]
pub struct LogicalExpression<R> {
    pub left: Box<dyn Expr::<R>>,
    pub operator: Token,
    pub right: Box<dyn Expr::<R>>,
}

impl<'a, R: Display + Debug> Expr<R> for LogicalExpression<R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, &'static str> {
        return visitor.visit_logical_expr(self);
    }
}

impl<'a, R: Display + Debug> Display for LogicalExpression<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{} {} {}", self.operator.lexeme, self.right, self.left);
    }
}

/// Set expressions are expressions that set a property on an object.
/// ## Example
/// ```
/// object.property = 1;
/// ```
#[derive(Debug)]
pub struct SetExpression<R> {
    pub object: Box<dyn Expr::<R>>,
    pub name: Token,
    pub value: Box<dyn Expr::<R>>,
}

impl<'a, R: Display + Debug> Expr<R> for SetExpression<R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, &'static str> {
        return visitor.visit_set_expr(self);
    }
}

impl<'a, R: Display + Debug> Display for SetExpression<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", self);
    }
}

/// Super expressions are expressions that call a method on the superclass.
/// ## Example
/// ```
/// super.method();
/// ```
#[derive(Debug)]
pub struct SuperExpression {
    pub keyword: Token,
    pub method: Token,
}

impl<'a, R: Display + Debug> Expr<R> for SuperExpression {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, &'static str> {
        return visitor.visit_super_expr(self);
    }
}

impl<'a> Display for SuperExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", self);
    }
}

/// Self expressions are expressions that call a method on the current class.
#[derive(Debug)]
pub struct SelfExpression {
    pub keyword: Token,
}

impl<'a, R: Display + Debug> Expr<R> for SelfExpression {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, &'static str> {
        return visitor.visit_self_expr(self);
    }
}

impl<'a> Display for SelfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", self);
    }
}

/// Unary expressions are expressions that have a single side and an operator.
/// ## Example
/// ```
/// !true
/// ```
#[derive(Debug)]
pub struct UnaryExpression<R> {
    pub operator: Token,
    pub right: Box<dyn Expr::<R>>,
}

impl<'a, R: Display + Debug> Expr<R> for UnaryExpression<R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, &'static str> {
        return visitor.visit_unary_expr(self);
    }
}

impl<'a, R: Display + Debug> Display for UnaryExpression<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", self);
    }
}

/// Variable expressions are expressions that are variables.
/// ## Example
/// ```
/// var x = 1;
/// ```
#[derive(Debug)]
pub struct VariableExpression {
    pub name: Token,
}

impl<'a, R: Display + Debug> Expr<R> for VariableExpression {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, &'static str> {
        return visitor.visit_variable_expr(self);
    }
}

impl<'a> Display for VariableExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", self);
    }
}
