use crate::expressions::{BinaryExpression, Expr, GroupingExpression, LiteralExpression, UnaryExpression};
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
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        return Parser {
            tokens,
            current: 0,
        };
    }

    fn expression_rule(&mut self) -> Box<dyn Expr::<String>> {
        return self.equality_rule();
    }

    fn equality_rule(&mut self) -> Box<dyn Expr::<String>> {
        let mut expr = self.comparison_rule();

        while self.peek().kind == TokenKind::BangEqual || self.peek().kind == TokenKind::EqualEqual {
            let current_token = self.tokens.get(self.current).unwrap_or_else(|| {
                println!("Error getting token at index {}", self.current);
                std::process::exit(1);
            });

            expr = Box::new(BinaryExpression {
                left: expr,
                operator: current_token.clone(),
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
            let current_token = self.tokens.get(self.current).unwrap_or_else(|| {
                println!("Error getting token at index {}", self.current);
                std::process::exit(1);
            });

            expr = Box::new(BinaryExpression {
                left: expr,
                operator: current_token.clone(),
                right: self.term_rule(),
            });
        }

        return expr;
    }

    fn term_rule(&mut self) -> Box<dyn Expr::<String>> {
        let mut expr = self.factor_rule();

        while self.peek().kind == TokenKind::Minus || self.peek().kind == TokenKind::Plus {
            let current_token = self.tokens.get(self.current).unwrap_or_else(|| {
                println!("Error getting token at index {}", self.current);
                std::process::exit(1);
            });

            expr = Box::new(BinaryExpression {
                left: expr,
                operator: current_token.clone(),
                right: self.factor_rule(),
            });
        }

        return expr;
    }

    fn factor_rule(&mut self) -> Box<dyn Expr::<String>> {
        let mut expr = self.unary_rule();

        while self.peek().kind == TokenKind::Slash || self.peek().kind == TokenKind::Star {
            let current_token = self.tokens.get(self.current).unwrap_or_else(|| {
                println!("Error getting token at index {}", self.current);
                std::process::exit(1);
            });

            expr = Box::new(BinaryExpression {
                left: expr,
                operator: current_token.clone(),
                right: self.unary_rule(),
            });
        }

        return expr;
    }

    fn unary_rule(&mut self) -> Box<dyn Expr::<String>> {
        // TODO: This currently doesn't support multiple unary operators in a row like `!!true`.
        if self.peek().kind == TokenKind::Bang || self.peek().kind == TokenKind::Minus {
            let current_token = self.tokens.get(self.current).unwrap_or_else(|| {
                println!("Error getting token at index {}", self.current);
                std::process::exit(1);
            });

            let expr = Box::new(UnaryExpression {
                operator: current_token.clone(),
                right: self.unary_rule(),
            });

            return expr;
        }

        return self.primary_rule();
    }

    fn primary_rule(&mut self) -> Box<dyn Expr::<String>> {
        while self.peek().kind == TokenKind::String || self.peek().kind == TokenKind::Number {
            let current_token = self.tokens.get(self.current).unwrap_or_else(|| {
                println!("Error getting token at index {}", self.current);
                std::process::exit(1);
            });

            return Box::new(
                LiteralExpression {
                    value: current_token.literal.clone(),
                }
            );
        }

        match self.peek().kind {
            TokenKind::True => Box::new(
                LiteralExpression {
                    value: Some(Literal::Boolean(true)),
                }
            ),
            TokenKind::False => Box::new(
                LiteralExpression {
                    value: Some(Literal::Boolean(false)),
                }
            ),
            TokenKind::Nil => Box::new(
                LiteralExpression {
                    value: Some(Literal::Nil),
                }
            ),
            TokenKind::LeftParen => {
                self.advance();
                let expr: Box<dyn Expr<String>> = self.expression_rule();

                if self.peek().kind != TokenKind::RightParen {
                    println!("Error: Expected ')' after expression.");
                    std::process::exit(1);
                }

                self.advance();

                return Box::new(
                    GroupingExpression {
                        expression: expr,
                    }
                );
            }
            _ => {
                println!("Error: Expected expression.");
                std::process::exit(1);
            }
        }
    }

    /// Checks AND advances the current token if it matches any of the passed tokens.
    fn match_token(&mut self, token_kinds: &Vec<TokenKind>) -> bool {
        if self.is_at_end() {
            return false;
        }

        for token_kind in token_kinds {
            if &self.peek().kind == token_kind {
                self.advance();

                return true;
            }
        }

        return false;
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

    /// peek to next token without advancing the current token.
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
                kind: TokenKind::Number,
                lexeme: "123".into(),
                line: 1,
                literal: Some(Literal::Number(123.into())),
            },
            Token {
                kind: TokenKind::Star,
                lexeme: "*".into(),
                line: 1,
                literal: None,
            },
            Token {
                kind: TokenKind::Number,
                lexeme: "45.67".into(),
                line: 1,
                literal: Some(Literal::Number(45.67.into())),
            },
            Token {
                kind: TokenKind::Eof,
                lexeme: "".into(),
                line: 1,
                literal: None,
            },
        ];

        let mut parser = Parser::new(&tokens);

        let expr = parser.expression_rule();

        println!("{}", expr.to_string());

        assert_eq!(expr.to_string(), "(* 123 45.67)");
    }
}