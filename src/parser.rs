use crate::expressions::{BinaryExpression, Expr, GroupingExpression, LiteralExpression, UnaryExpression};
use crate::language_errors::CompilerError;
use crate::literal_types::Literal;
use crate::token::Token;
use crate::token_kinds::TokenKind;


/// The Parser is responsible for taking a list of tokens and turning them into an AST.
///
/// ## Grammar:
/// * expression        → equality ;
/// * equality          → comparison ( ( "!=" | "==" ) comparison )* ;
/// * comparison        → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
/// * term              → factor ( ( "-" | "+" ) factor )* ;
/// * factor            → unary ( ( "/" | "*" ) unary )* ;
/// * unary             → ( "!" | "-" ) unary | primary ;
/// * primary           → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
///
/// Note: `(a)*` means 0 or more of a.
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
    errors: Vec<CompilerError>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        return Parser {
            tokens,
            current: 0,
            errors: Vec::new(),
        };
    }

    fn expression_rule(&mut self) -> Box<dyn Expr::<String>> {
        return self.equality_rule();
    }

    fn equality_rule(&mut self) -> Box<dyn Expr::<String>> {
        let mut expr = self.comparison_rule();

        while self.peek().kind == TokenKind::BangEqual || self.peek().kind == TokenKind::EqualEqual {
            self.advance();

            expr = Box::new(BinaryExpression {
                left: expr,
                operator: self.previous().clone(),
                right: self.comparison_rule(),
            });
        }

        return expr;
    }

    fn comparison_rule(&mut self) -> Box<dyn Expr::<String>> {
        let mut expr = self.term_rule();

        while self.peek().kind == TokenKind::Greater || self.peek().kind == TokenKind::GreaterEqual
            || self.peek().kind == TokenKind::Less || self.peek().kind == TokenKind::LessEqual
        {
            self.advance();

            expr = Box::new(BinaryExpression {
                left: expr,
                operator: self.previous().clone(),
                right: self.term_rule(),
            });
        }

        return expr;
    }

    fn term_rule(&mut self) -> Box<dyn Expr::<String>> {
        let mut expr = self.factor_rule();

        while self.peek().kind == TokenKind::Minus || self.peek().kind == TokenKind::Plus {
            self.advance();

            expr = Box::new(BinaryExpression {
                left: expr,
                operator: self.previous().clone(),
                right: self.factor_rule(),
            });
        }

        return expr;
    }

    fn factor_rule(&mut self) -> Box<dyn Expr::<String>> {
        let mut expr = self.unary_rule();

        while self.peek().kind == TokenKind::Slash || self.peek().kind == TokenKind::Star {
            self.advance();

            expr = Box::new(BinaryExpression {
                left: expr,
                operator: self.previous().clone(),
                right: self.unary_rule(),
            });
        }

        return expr;
    }

    fn unary_rule(&mut self) -> Box<dyn Expr::<String>> {
        // TODO: This currently doesn't support multiple unary operators in a row like `!!true`.
        if self.peek().kind == TokenKind::Bang || self.peek().kind == TokenKind::Minus {
            self.advance();

            let expr = Box::new(UnaryExpression {
                operator: self.previous().clone(),
                right: self.unary_rule(),
            });

            return expr;
        }

        return self.primary_rule();
    }

    fn primary_rule(&mut self) -> Box<dyn Expr::<String>> {
        if self.peek().kind == TokenKind::True {
            self.advance();

            return Box::new(
                LiteralExpression {
                    value: Some(Literal::Boolean(true)),
                }
            );
        } else if self.peek().kind == TokenKind::False {
            self.advance();

            return Box::new(
                LiteralExpression {
                    value: Some(Literal::Boolean(false)),
                }
            );
        } else if self.peek().kind == TokenKind::Nil {
            self.advance();

            return Box::new(
                LiteralExpression {
                    value: Some(Literal::Nil),
                }
            );
        } else if self.peek().kind == TokenKind::String || self.peek().kind == TokenKind::Number {
            self.advance();

            return Box::new(
                LiteralExpression {
                    value: self.previous().literal.clone(),
                }
            );
        } else if self.peek().kind == TokenKind::LeftParen {
            // We don't capture any of the parentheses tokens. We only group the expression.

            self.advance();

            let expr: Box<dyn Expr<String>> = self.expression_rule();

            println!("{}", self.peek());

            // Check if the next token is a closing parenthesis.
            if self.peek().kind != TokenKind::RightParen {
                self.errors.push(
                    CompilerError::new(
                        format!("Expected ')' after expression"),
                        self.peek().line,
                        self.peek().column,
                        None,
                    )
                );
            }

            self.advance();

            return Box::new(
                GroupingExpression {
                    expression: expr,
                }
            );
        } else {
            self.errors.push(
                CompilerError::new(
                    format!("Expected expression"),
                    self.peek().line,
                    self.peek().column,
                    None,
                )
            );

            return Box::new(
                LiteralExpression {
                    value: None,
                }
            );
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
        return self.peek().kind == TokenKind::Eof;
    }

    /// Get the next token without advancing the current token.
    fn peek(&self) -> &Token {
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
    use super::*;
    use crate::literal_types::Literal;

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

        assert_eq!(expr.to_string(), "((* 123 45.67))");
    }
}