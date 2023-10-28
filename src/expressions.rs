use crate::literal::Literal;
use crate::token::Token;

/// A trait that represents an expression in the AST.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Binary expressions are expressions that have a left and right side, and an operator in between.
    /// ## Example
    /// ```
    /// 1 + 2
    /// ```
    BinaryExpression {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    /// Call expressions are expressions that call a function.
    /// ## Example
    /// ```
    /// a_function();
    CallExpression {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Box<Expr>>,
    },
    /// Get expressions are expressions that get a property from an object
    /// ## Example
    /// ```
    /// object.property
    /// ```
    GetExpression { object: Box<Expr>, name: Token },
    /// Grouping expressions are expressions that group other expressions together.
    /// ## Example
    /// ```
    /// (1 + 2)
    /// ```
    GroupingExpression { expression: Box<Expr> },
    /// Literal expressions are expressions that are literals.
    /// ## Example
    /// ```
    /// 1
    /// ```
    LiteralExpression { value: Option<Literal> },
    /// Logical expressions are expressions that are logical.
    /// ## Example
    /// ```
    /// true and false
    /// ```
    LogicalExpression {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    /// Set expressions are expressions that set a property on an object.
    /// ## Example
    /// ```
    /// object.property = 1;
    /// ```
    SetExpression {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
    /// Super expressions are expressions that call a method on the superclass.
    /// ## Example
    /// ```
    /// super.method();
    /// ```
    SuperExpression { keyword: Token, method: Token },
    /// Self expressions are expressions that call a method on the current class.
    /// ## Example
    /// ```
    /// self.method();
    /// ```
    SelfExpression { keyword: Token },
    /// Unary expressions are expressions that have a single side and an operator.
    /// ## Example
    /// ```
    /// !true
    /// ```
    UnaryExpression { operator: Token, right: Box<Expr> },
    /// Variable expressions are expressions that are variables.
    /// ## Example
    /// ```
    /// var x = 1;
    /// ```
    VarDeclExpression { name: Token },
    /// Variable resolution expressions are expressions that resolve a variable.
    /// ## Example
    /// ```
    /// x
    /// ```
    VariableResolutionExpression { name: Token },
    /// Assign expressions are expressions that assign a value to a variable.
    /// ## Example
    /// ```
    /// x = 1;
    /// ```
    AssignmentExpression { name: Token, value: Box<Expr> },
}
