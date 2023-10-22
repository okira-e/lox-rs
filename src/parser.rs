use crate::expressions::{Expr};
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
/// * complete_statement    → statement ";" ;
/// * statement             → exprStmt | printStmt ;
/// * exprStmt              → expression ;
/// * printStmt             → "print" expression ;
/// * expression            → equality ;
/// * equality              → comparison ( ( "!=" | "==" ) comparison )* ;
/// * comparison            → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
/// * term                  → factor ( ( "-" | "+" ) factor )* ;
/// * factor                → unary ( ( "/" | "*" ) unary )* ;
/// * unary                 → ( "!" | "-" ) unary | primary ;
/// * primary               → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
///
/// Note: `(a)*` means 0 or more of a.
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

    /// complete_statement → statement ";" ;
    fn complete_statement_rule(&mut self) -> Stmt {
        let stmt = self.statement_rule();

        if self.tokens[self.current].kind == TokenKind::Semicolon {
            self.advance();
        } else {
            let err = Error::new(
                "Expected \";\" after statement".into(),
                Some(self.previous().line),
                self.tokens[self.current].column,
                None,
            );

            report_error(&err);

            self.errors.push(err);
        }

        return stmt;
    }


    /// statement → exprStmt | printStmt ;
    fn statement_rule(&mut self) -> Stmt {
        return if self.current_token().kind == TokenKind::Print {
            self.print_statement_rule()
        } else {
            self.expression_statement_rule()
        };
    }

    /// printStmt → "print" expression ;
    fn print_statement_rule(&mut self) -> Stmt {
        self.advance();

        let value = self.expression_rule();

        return Stmt::PrintStmt {
            expression: value,
        };
    }

    // exprStmt → expression ;
    fn expression_statement_rule(&mut self) -> Stmt {
        let expr = self.expression_rule();

        return Stmt::ExpressionStmt {
            expression: expr,
        };
    }

    /// expression → equality ;
    fn expression_rule(&mut self) -> Box<Expr> {
        return self.equality_rule();
    }

    /// equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality_rule(&mut self) -> Box<Expr> {
        let mut expr = self.comparison_rule();

        while self.tokens[self.current].kind == TokenKind::BangEqual || self.tokens[self.current].kind == TokenKind::EqualEqual {
            self.advance();

            expr = Box::new(Expr::BinaryExpression {
                left: expr,
                operator: self.previous().clone(),
                right: self.comparison_rule(),
            });
        }

        return expr;
    }

    /// comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison_rule(&mut self) -> Box<Expr> {
        let mut expr = self.term_rule();

        while self.tokens[self.current].kind == TokenKind::Greater || self.tokens[self.current].kind == TokenKind::GreaterEqual
            || self.tokens[self.current].kind == TokenKind::Less || self.tokens[self.current].kind == TokenKind::LessEqual
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

    /// term → factor ( ( "-" | "+" ) factor )* ;
    fn term_rule(&mut self) -> Box<Expr> {
        let mut expr = self.factor_rule();

        while self.tokens[self.current].kind == TokenKind::Minus || self.tokens[self.current].kind == TokenKind::Plus {
            self.advance();

            expr = Box::new(Expr::BinaryExpression {
                left: expr,
                operator: self.previous().clone(),
                right: self.factor_rule(),
            });
        }

        return expr;
    }

    /// factor → unary ( ( "/" | "*" ) unary )* ;
    fn factor_rule(&mut self) -> Box<Expr> {
        let mut expr = self.unary_rule();

        while self.tokens[self.current].kind == TokenKind::Slash || self.tokens[self.current].kind == TokenKind::Star {
            self.advance();

            expr = Box::new(Expr::BinaryExpression {
                left: expr,
                operator: self.previous().clone(),
                right: self.unary_rule(),
            });
        }

        return expr;
    }

    /// unary → ( "!" | "-" ) unary | primary ;
    fn unary_rule(&mut self) -> Box<Expr> {
        // TODO: This currently doesn't support multiple unary operators in a row like `!!true`.
        if self.tokens[self.current].kind == TokenKind::Bang || self.tokens[self.current].kind == TokenKind::Minus {
            self.advance();

            let expr = Box::new(Expr::UnaryExpression {
                operator: self.previous().clone(),
                right: self.unary_rule(),
            });

            return expr;
        }

        return self.primary_rule();
    }

    /// primary → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary_rule(&mut self) -> Box<Expr> {
        return if self.current_token().kind == TokenKind::True {
            self.advance();

            Box::new(
                Expr::LiteralExpression {
                    value: Some(Literal::Boolean(true)),
                }
            )
        } else if self.current_token().kind == TokenKind::False {
            self.advance();

            Box::new(
                Expr::LiteralExpression {
                    value: Some(Literal::Boolean(false)),
                }
            )
        } else if self.current_token().kind == TokenKind::Nil {
            self.advance();

            Box::new(
                Expr::LiteralExpression {
                    value: Some(Literal::Nil),
                }
            )
        } else if self.current_token().kind == TokenKind::String || self.current_token().kind == TokenKind::Number {
            self.advance();

            Box::new(
                Expr::LiteralExpression {
                    value: self.previous().literal.clone(),
                }
            )
        } else if self.current_token().kind == TokenKind::LeftParen {
            // We don't capture any of the parentheses tokens. We only group the expression.

            self.advance();

            let expr: Box<Expr> = self.expression_rule();

            // Check if the next token is a closing parenthesis.
            if self.current_token().kind != TokenKind::RightParen {
                let err = Error::new(
                    "Expected \")\" after expression".into(),
                    Some(self.current_token().line),
                    self.current_token().column,
                    None,
                );


                report_error(&err);

                self.errors.push(err);
            }

            self.advance();

            Box::new(
                Expr::GroupingExpression {
                    expression: expr,
                }
            )
        } else {
            let err = Error::new(
                "Unrecognised token".into(),
                Some(self.current_token().line),
                self.current_token().column,
                None,
            );

            report_error(&err);

            self.errors.push(err);

            self.advance();

            Box::new(
                Expr::LiteralExpression {
                    value: Some(Literal::Nil),
                }
            )
        };
    }

    /// Get the next token and advance the current token.
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    fn synchronise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().kind == TokenKind::Semicolon {
                return;
            }

            match self.tokens[self.current].kind {
                TokenKind::Class => {}
                TokenKind::Fun => {}
                TokenKind::Var => {}
                TokenKind::For => {}
                TokenKind::If => {}
                TokenKind::While => {}
                TokenKind::Print => {}
                TokenKind::Return => {}
                _ => {
                    self.advance();
                }
            }
        }
    }

    /// Checks if we are at the end of the token list.
    fn is_at_end(&self) -> bool {
        return self.tokens[self.current].kind == TokenKind::Eof;
    }

    /// Get the next token without advancing the current token.
    fn peek(&self) -> &Token {
        return self.tokens.get(self.current + 1).unwrap_or_else(|| {
            println!("Error getting token at index {}", self.current + 1);
            std::process::exit(1);
        });
    }

    fn current_token(&self) -> &Token {
        return self.tokens.get(self.current).unwrap_or_else(|| {
            println!("Error getting token at index {}", self.current);
            std::process::exit(1);
        });
    }

    /// Get the previous token.
    fn previous(&self) -> &Token {
        return self.tokens.get(self.current - 1).unwrap_or_else(|| {
            println!("Error getting token at index {}", self.current - 1);
            std::process::exit(1);
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::Expr::GroupingExpression;
    use super::*;
    use crate::literal::Literal;

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
                kind: TokenKind::Eof,
                lexeme: "".into(),
                line: 1,
                column: 4,
                literal: None,
            },
        ];

        let mut parser = Parser::new(&tokens);

        let expr = parser.expression_rule();

        assert_eq!(
            expr,
            Box::new(
                GroupingExpression {
                    expression: Box::new(
                        Expr::BinaryExpression {
                            left: Box::new(
                                Expr::LiteralExpression {
                                    value: Some(Literal::Number(123.into())),
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
                                Expr::LiteralExpression {
                                    value: Some(Literal::Number(45.67.into())),
                                }
                            ),
                        }
                    )
                }
            )
        );
    }
}