use crate::expressions::Expr;
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    ExpressionStmt {
        expression: Box<Expr>,
    },
    PrintStmt {
        expression: Box<Expr>,
    },
    VarDeclStmt {
        name: Token,
        initializer: Expr,
    },
    AssignmentStmt {
        name: Token,
        value: Box<Expr>,
    },
    BlockStmt {
        statements: Vec<Stmt>,
    },
    IfStmt {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    WhileStmt {
        condition: Box<Expr>,
        body: Box<Stmt>,
    },
    FunctionStmt {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    ReturnStmt {
        keyword: Token,
        value: Option<Box<Expr>>,
    },
    ClassStmt {
        name: Token,
        methods: Vec<Stmt>,
        superclass: Option<Box<Expr>>,
    },
}
