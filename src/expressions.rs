use std::fmt::Display;
use crate::literal_types::Literal;
use crate::token::Token;

pub trait ExprVisitor<'a, R: Display> {
    fn visit_assign_expr(&self, expr: &AssignExpression::<R>) -> Result<String, &'static str>;
    fn visit_binary_expr(&self, expr: &BinaryExpression::<R>) -> Result<String, &'static str>;
    fn visit_call_expr(&self, expr: &CallExpression::<R>) -> Result<String, &'static str>;
    fn visit_get_expr(&self, expr: &GetExpression::<R>) -> Result<String, &'static str>;
    fn visit_grouping_expr(&self, expr: &GroupingExpression::<R>) -> Result<String, &'static str>;
    fn visit_literal_expr(&self, expr: &LiteralExpression) -> Result<String, &'static str>;
    fn visit_logical_expr(&self, expr: &LogicalExpression::<R>) -> Result<String, &'static str>;
    fn visit_set_expr(&self, expr: &SetExpression::<R>) -> Result<String, &'static str>;
    fn visit_super_expr(&self, expr: &SuperExpression) -> Result<String, &'static str>;
    fn visit_self_expr(&self, expr: &SelfExpression) -> Result<String, &'static str>;
    fn visit_unary_expr(&self, expr: &UnaryExpression::<R>) -> Result<String, &'static str>;
    fn visit_variable_expr(&self, expr: &VariableExpression) -> Result<String, &'static str>;
}

pub trait Expr<'a, R: Display> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<String, &'static str>;
}

/* AssignExpression */

pub struct AssignExpression<'a, R> {
    pub name: Token::<'a>,
    pub value: Box<dyn Expr::<'a, R>>,
}

impl<'a, R: Display> Expr<'a, R> for AssignExpression<'_, R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<String, &'static str> {
        return visitor.visit_assign_expr(self);
    }
}

/* BinaryExpression */
pub struct BinaryExpression<'a, R> {
    pub left: Box<dyn Expr::<'a, R>>,
    pub operator: Token::<'a>,
    pub right: Box<dyn Expr::<'a, R>>,
}

impl<'a, R: Display> Expr<'a, R> for BinaryExpression<'_, R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<String, &'static str> {
        return visitor.visit_binary_expr(self);
    }
}

/* CallExpression */
pub struct CallExpression<'a, R> {
    pub callee: Box<dyn Expr::<'a, R>>,
    pub paren: Token::<'a>,
    pub arguments: Vec<Box<dyn Expr::<'a, R>>>,
}

impl<'a, R: Display> Expr<'a, R> for CallExpression<'_, R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<String, &'static str> {
        return visitor.visit_call_expr(self);
    }
}

/* GetExpression */
pub struct GetExpression<'a, R> {
    pub object: Box<dyn Expr::<'a, R>>,
    pub name: Token::<'a>,
}

impl<'a, R: Display> Expr<'a, R> for GetExpression<'_, R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<String, &'static str> {
        return visitor.visit_get_expr(self);
    }
}

/* GroupingExpression */
pub struct GroupingExpression<'a, R> {
    pub expression: Box<dyn Expr::<'a, R>>,
}

impl<'a, R: Display> Expr<'a, R> for GroupingExpression<'_, R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<String, &'static str> {
        return visitor.visit_grouping_expr(self);
    }
}

/* LiteralExpression */
pub struct LiteralExpression<'a> {
    pub value: Option<Literal::<'a>>,
}

impl<'a, R: Display> Expr<'a, R> for LiteralExpression<'_> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<String, &'static str> {
        return visitor.visit_literal_expr(self);
    }
}

/* LogicalExpression */
pub struct LogicalExpression<'a, R> {
    pub left: Box<dyn Expr::<'a, R>>,
    pub operator: Token::<'a>,
    pub right: Box<dyn Expr::<'a, R>>,
}

impl<'a, R: Display> Expr<'a, R> for LogicalExpression<'_, R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<String, &'static str> {
        return visitor.visit_logical_expr(self);
    }
}

/* SetExpression */
pub struct SetExpression<'a, R> {
    pub object: Box<dyn Expr::<'a, R>>,
    pub name: Token::<'a>,
    pub value: Box<dyn Expr::<'a, R>>,
}

impl<'a, R: Display> Expr<'a, R> for SetExpression<'_, R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<String, &'static str> {
        return visitor.visit_set_expr(self);
    }
}

/* SuperExpression */
pub struct SuperExpression<'a> {
    pub keyword: Token::<'a>,
    pub method: Token::<'a>,
}

impl<'a, R: Display> Expr<'a, R> for SuperExpression<'_> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<String, &'static str> {
        return visitor.visit_super_expr(self);
    }
}

/* SelfExpression */
pub struct SelfExpression<'a> {
    pub keyword: Token::<'a>,
}

impl<'a, R: Display> Expr<'a, R> for SelfExpression<'_> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<String, &'static str> {
        return visitor.visit_self_expr(self);
    }
}

/* UnaryExpression */
pub struct UnaryExpression<'a, R> {
    pub operator: Token::<'a>,
    pub right: Box<dyn Expr::<'a, R>>,
}

impl<'a, R: Display> Expr<'a, R> for UnaryExpression<'_, R> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<String, &'static str> {
        return visitor.visit_unary_expr(self);
    }
}


/* VariableExpression */
pub struct VariableExpression<'a> {
    pub name: Token::<'a>,
}

impl<'a, R: Display> Expr<'a, R> for VariableExpression<'_> {
    fn accept(&self, visitor: &dyn ExprVisitor<R>) -> Result<String, &'static str> {
        return visitor.visit_variable_expr(self);
    }
}
