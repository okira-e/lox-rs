use crate::expressions::Expr;
use crate::language_error::Error;
use crate::literal::Literal;
use crate::report_error;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::token_kinds::TokenKind;

/// The Parser is responsible for taking a list of tokens and turning them into an AST.
/// It reports (doesn't return) any errors that occur during parsing.
///
/// ## Grammar:
/// * program               → complete_statement* EOF ;
/// * complete_statement    → declaration ";" ;
/// * declaration           → varDecl | statement ;
/// * varDecl               → "var" IDENTIFIER ("=" expression)? ;
/// * statement             → exprStmt | printStmt ;
/// * exprStmt              → expression ;
/// * printStmt             → "print" expression ;
/// * expression            → assignment ;
/// * assignment            → IDENTIFIER "=" assignment | equality ;
/// * equality              → comparison ( ( "!=" | "==" ) comparison )* ;
/// * comparison            → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
/// * term                  → factor ( ( "-" | "+" ) factor )* ;
/// * factor                → unary ( ( "/" | "*" ) unary )* ;
/// * unary                 → ( "!" | "-" ) unary | primary ;
/// * primary               → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER ;
///
/// Note: 
/// * `(a)*` means 0 or more of a.
/// * `?` means that it is optional.
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
    pub errors: Vec<Error>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        return Parser {
            tokens,
            current: 0,
            errors: Vec::new(),
        };
    }

    /// Parses the tokens into an AST. Reports any errors that occur during parsing and populates
    /// the error list.
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::<Stmt>::new();

        while !self.is_at_end() {
            statements.push(self.complete_statement_rule());
        }

        return statements;
    }

    fn complete_statement_rule(&mut self) -> Stmt {
        let stmt = self.declaration_rule();

        if self.tokens[self.current].kind != TokenKind::Semicolon {
            let err = Error::new(
                "Expected \";\" after statement.".into(),
                Some(self.previous().line),
                self.tokens[self.current].column,
                None,
            );

            report_error(&err);

            self.errors.push(err);
        } else {
            self.advance();
        }

        return stmt;
    }

    fn declaration_rule(&mut self) -> Stmt {
        return if self.current_token().kind == TokenKind::Var {
            self.var_declaration_rule()
        } else {
            self.statement_rule()
        };
    }

    fn var_declaration_rule(&mut self) -> Stmt {
        self.advance(); // current is variable name.

        if self.current_token().kind != TokenKind::Identifier {
            let err = Error::new(
                "Expected identifier after \"var\".".into(),
                Some(self.previous().line),
                self.previous().column,
                None,
            );

            report_error(&err);

            self.errors.push(err);

            self.synchronise();

            // Not sure what to return here. We need to return a Stmt, but we don't have one.
            return Stmt::VarDeclStmt {
                name: self.previous().clone(),
                initializer: Expr::LiteralExpression {
                    value: Some(Literal::Nil),
                },
            };
        }

        let var_name = self.current_token().lexeme.clone();

        self.advance(); // current is "=" or ";"

        let value = if self.current_token().kind == TokenKind::Equal {
            self.advance();

            self.expression_rule()
        } else {
            Box::new(Expr::LiteralExpression { value: None })
        };

        return Stmt::VarDeclStmt {
            name: Token {
                kind: TokenKind::Identifier,
                lexeme: var_name,
                line: self.previous().line,
                column: self.previous().column,
                literal: None,
            },
            initializer: *value,
        };
    }

    fn statement_rule(&mut self) -> Stmt {
        return if self.current_token().kind == TokenKind::Print {
            // Print statement.
            self.print_statement_rule()
        } else if self.peek().kind == TokenKind::Equal {
            // Assignment statement.
            let ret = Stmt::AssignmentStmt {
                name: self.current_token().clone(),
                value: self.assignment_rule(),
            };

            return ret;
        } else {
            // Expression statement. An expression wrapped in a statement.
            self.expression_statement_rule()
        };
    }

    fn print_statement_rule(&mut self) -> Stmt {
        self.advance();

        let value = self.expression_rule();

        return Stmt::PrintStmt { expression: value };
    }

    fn expression_statement_rule(&mut self) -> Stmt {
        let expr = self.expression_rule();

        return Stmt::ExpressionStmt { expression: expr };
    }

    fn expression_rule(&mut self) -> Box<Expr> {
        return self.assignment_rule();
    }

    fn assignment_rule(&mut self) -> Box<Expr> {
        let expr = self.equality_rule();

        if self.current_token().kind == TokenKind::Equal {
            let var_name = self.previous().clone();

            self.advance();

            let value = self.assignment_rule();

            return Box::new(Expr::AssignmentExpression {
                name: var_name,
                value,
            });
        }

        return expr;
    }

    fn equality_rule(&mut self) -> Box<Expr> {
        let mut expr = self.comparison_rule();

        while self.tokens[self.current].kind == TokenKind::BangEqual
            || self.tokens[self.current].kind == TokenKind::EqualEqual
        {
            self.advance();

            expr = Box::new(Expr::BinaryExpression {
                left: expr,
                operator: self.previous().clone(),
                right: self.comparison_rule(),
            });
        }

        return expr;
    }

    fn comparison_rule(&mut self) -> Box<Expr> {
        let mut expr = self.term_rule();

        while self.tokens[self.current].kind == TokenKind::Greater
            || self.tokens[self.current].kind == TokenKind::GreaterEqual
            || self.tokens[self.current].kind == TokenKind::Less
            || self.tokens[self.current].kind == TokenKind::LessEqual
        {
            self.advance();

            expr = Box::new(Expr::BinaryExpression {
                left: expr,
                operator: self.previous().clone(),
                right: self.term_rule(),
            });
        }

        return expr;
    }

    fn term_rule(&mut self) -> Box<Expr> {
        let mut expr = self.factor_rule();

        while self.tokens[self.current].kind == TokenKind::Minus
            || self.tokens[self.current].kind == TokenKind::Plus
        {
            self.advance();

            expr = Box::new(Expr::BinaryExpression {
                left: expr,
                operator: self.previous().clone(),
                right: self.factor_rule(),
            });
        }

        return expr;
    }

    fn factor_rule(&mut self) -> Box<Expr> {
        let mut expr = self.unary_rule();

        while self.tokens[self.current].kind == TokenKind::Slash
            || self.tokens[self.current].kind == TokenKind::Star
        {
            self.advance();

            expr = Box::new(Expr::BinaryExpression {
                left: expr,
                operator: self.previous().clone(),
                right: self.unary_rule(),
            });
        }

        return expr;
    }

    fn unary_rule(&mut self) -> Box<Expr> {
        // FIX: This currently doesn't support multiple unary operators in a row like `!!true`.
        if self.tokens[self.current].kind == TokenKind::Bang
            || self.tokens[self.current].kind == TokenKind::Minus
        {
            self.advance();

            let expr = Box::new(Expr::UnaryExpression {
                operator: self.previous().clone(),
                right: self.unary_rule(),
            });

            return expr;
        }

        return self.primary_rule();
    }

    fn primary_rule(&mut self) -> Box<Expr> {
        return if self.current_token().kind == TokenKind::True {
            self.advance();

            Box::new(Expr::LiteralExpression {
                value: Some(Literal::Boolean(true)),
            })
        } else if self.current_token().kind == TokenKind::False {
            self.advance();

            Box::new(Expr::LiteralExpression {
                value: Some(Literal::Boolean(false)),
            })
        } else if self.current_token().kind == TokenKind::Nil {
            self.advance();

            Box::new(Expr::LiteralExpression {
                value: Some(Literal::Nil),
            })
        } else if self.current_token().kind == TokenKind::String
            || self.current_token().kind == TokenKind::Number
        {
            self.advance();

            Box::new(Expr::LiteralExpression {
                value: self.previous().literal.clone(),
            })
        } else if self.current_token().kind == TokenKind::LeftParen {
            // We don't capture any of the parentheses tokens. We only group the expression.

            self.advance();

            let expr: Box<Expr> = self.expression_rule();

            // Check if the next token is a closing parenthesis.
            if self.current_token().kind != TokenKind::RightParen {
                let err = Error::new(
                    "Expected \")\" after expression.".into(),
                    Some(self.current_token().line),
                    self.current_token().column,
                    None,
                );

                report_error(&err);

                self.errors.push(err);
            }

            self.advance();

            Box::new(Expr::GroupingExpression { expression: expr })
        } else {
            self.advance();

            Box::new(Expr::VariableResolutionExpression {
                name: self.previous().clone(),
            })
        };
    }

    /// Runs whenever we encounter a parsing error. It will discard the current statement and jump
    /// to the next one.
    fn synchronise(&mut self) {
        while self.current_token().kind != TokenKind::Semicolon {
            self.advance();
        }
    }

    /// Get the next token and advance the current token.
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    /// Checks if we are at the end of the token list.
    fn is_at_end(&self) -> bool {
        return self.tokens[self.current].kind == TokenKind::Eof;
    }

    /// Get the next token without advancing the current token.
    fn peek(&self) -> &Token {
        return self.tokens.get(self.current + 1).unwrap_or_else(|| {
            println!("Error getting token at index {}.", self.current + 1);
            std::process::exit(1);
        });
    }

    fn current_token(&self) -> &Token {
        return self.tokens.get(self.current).unwrap_or_else(|| {
            println!("Error getting token at index {}.", self.current);
            std::process::exit(1);
        });
    }

    /// Get the previous token.
    fn previous(&self) -> &Token {
        return self.tokens.get(self.current - 1).unwrap_or_else(|| {
            println!("Error getting token at index {}.", self.current - 1);
            std::process::exit(1);
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::Expr::GroupingExpression;
    use crate::literal::Literal;

    use super::*;

    #[test]
    fn test_parser() {
        let tokens = vec![
            Token {
                kind: TokenKind::LeftParen,
                lexeme: "(".into(),
                line: 1,
                column: 0,
                literal: None,
            },
            Token {
                kind: TokenKind::Number,
                lexeme: "123".into(),
                line: 1,
                column: 1,
                literal: Some(Literal::Number(123.into())),
            },
            Token {
                kind: TokenKind::Star,
                lexeme: "*".into(),
                line: 1,
                column: 2,
                literal: None,
            },
            Token {
                kind: TokenKind::Number,
                lexeme: "45.67".into(),
                line: 1,
                column: 3,
                literal: Some(Literal::Number(45.67.into())),
            },
            Token {
                kind: TokenKind::RightParen,
                lexeme: ")".into(),
                line: 1,
                column: 0,
                literal: None,
            },
            Token {
                kind: TokenKind::Semicolon,
                lexeme: ";".into(),
                line: 1,
                column: 0,
                literal: None,
            },
            Token {
                kind: TokenKind::Eof,
                lexeme: "".into(),
                line: 1,
                column: 4,
                literal: None,
            },
        ];

        let mut parser = Parser::new(&tokens);

        let statements = parser.parse();

        assert_eq!(
            statements[0],
            Stmt::ExpressionStmt {
                expression: Box::new(GroupingExpression {
                    expression: Box::new(Expr::BinaryExpression {
                        left: Box::new(Expr::LiteralExpression {
                            value: Some(Literal::Number(123.into())),
                        }),
                        operator: Token {
                            kind: TokenKind::Star,
                            lexeme: "*".into(),
                            line: 1,
                            column: 2,
                            literal: None,
                        },
                        right: Box::new(Expr::LiteralExpression {
                            value: Some(Literal::Number(45.67.into())),
                        }),
                    }),
                })
            }
        );
    }
}
