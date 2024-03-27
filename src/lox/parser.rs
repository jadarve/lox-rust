use super::{Expr, ExprVisitor, Token};

pub struct Statement {}

pub struct ParseError {
    message: String,
}

impl ToString for ParseError {
    fn to_string(&self) -> String {
        self.message.clone()
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            let expr = self.parse_expression()?;
            statements.push(expr);
        }

        Ok(statements)
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_expression_equality()
    }

    fn parse_expression_equality(&mut self) -> Result<Expr, ParseError> {
        let mut left_expr = self.parse_expression_comparison()?;

        while self.match_token(vec![Token::EqualEqual, Token::BangEqual]) {
            let operator = self.previous().clone();
            let right_expr = self.parse_expression_comparison()?;

            left_expr = match operator {
                Token::EqualEqual => Expr::BinaryEqual(Box::new(left_expr), Box::new(right_expr)),
                Token::BangEqual => Expr::BinaryNotEqual(Box::new(left_expr), Box::new(right_expr)),
                _ => {
                    return Err(ParseError {
                        message: format!("Unexpected token while parsing equality: {:?}", operator),
                    });
                }
            };
        }

        Ok(left_expr)
    }

    fn parse_expression_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut left_expr = self.parse_expression_add_sub()?;

        while self.match_token(vec![
            Token::Less,
            Token::LessEqual,
            Token::Greater,
            Token::GreaterEqual,
        ]) {
            let operator = self.previous().clone();
            let right_expr = self.parse_expression_add_sub()?;

            left_expr = match operator {
                Token::Less => Expr::BinaryLess(Box::new(left_expr), Box::new(right_expr)),
                Token::LessEqual => {
                    Expr::BinaryLessEqual(Box::new(left_expr), Box::new(right_expr))
                }
                Token::Greater => Expr::BinaryGreater(Box::new(left_expr), Box::new(right_expr)),
                Token::GreaterEqual => {
                    Expr::BinaryGreaterEqual(Box::new(left_expr), Box::new(right_expr))
                }
                _ => {
                    return Err(ParseError {
                        message: format!(
                            "Unexpected token while parsing comparison: {:?}",
                            operator
                        ),
                    });
                }
            };
        }

        Ok(left_expr)
    }

    fn parse_expression_add_sub(&mut self) -> Result<Expr, ParseError> {
        let mut left_expr = self.parse_expression_mul_div()?;

        while self.match_token(vec![Token::Plus, Token::Minus]) {
            let operator = self.previous().clone();
            let right_expr = self.parse_expression_mul_div()?;

            left_expr = match operator {
                Token::Plus => Expr::BinaryAdd(Box::new(left_expr), Box::new(right_expr)),
                Token::Minus => Expr::BinarySub(Box::new(left_expr), Box::new(right_expr)),
                _ => {
                    return Err(ParseError {
                        message: format!("Unexpected token while parsing add/sub: {:?}", operator),
                    });
                }
            };
        }

        Ok(left_expr)
    }

    fn parse_expression_mul_div(&mut self) -> Result<Expr, ParseError> {
        let mut left_expr = self.parse_expression_unary()?;

        while self.match_token(vec![Token::Star, Token::Slash]) {
            let operator = self.previous().clone();
            let right_expr = self.parse_expression_unary()?;

            left_expr = match operator {
                Token::Star => Expr::BinaryMul(Box::new(left_expr), Box::new(right_expr)),
                Token::Slash => Expr::BinaryDiv(Box::new(left_expr), Box::new(right_expr)),
                _ => {
                    return Err(ParseError {
                        message: format!("Unexpected token while parsing mul/div: {:?}", operator),
                    });
                }
            };
        }

        Ok(left_expr)
    }

    fn parse_expression_unary(&mut self) -> Result<Expr, ParseError> {
        self.advance(); // FIXME: check if here I need to advance

        match self.previous() {
            Token::Bang => {
                let expr = self.parse_expression_unary()?;
                Ok(Expr::UnaryBang(Box::new(expr)))
            }
            Token::Minus => {
                let expr = self.parse_expression_unary()?;
                Ok(Expr::UnaryMinus(Box::new(expr)))
            }
            _ => self.parse_expression_primary(),
        }
    }

    fn parse_expression_primary(&mut self) -> Result<Expr, ParseError> {
        match self.previous() {
            Token::NumberLiteral(n) => {
                return Ok(Expr::LiteralNumber(*n));
            }
            Token::StringLiteral(s) => {
                return Ok(Expr::LiteralString(s.clone()));
            }
            Token::Identifier(s) => {
                return Ok(Expr::Identifier(s.clone()));
            }
            Token::False => {
                return Ok(Expr::False);
            }
            Token::True => {
                return Ok(Expr::True);
            }
            Token::Nil => {
                return Ok(Expr::Nil);
            }
            _ => {
                return Err(ParseError {
                    message: format!(
                        "Unexpected token while parsing primary: {:?}",
                        self.previous()
                    ),
                });
            }
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    // Auxiliary methods
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek() == token
    }

    fn match_token(&mut self, tokens: Vec<Token>) -> bool {
        // I would need a variadic function here
        if tokens.iter().any(|token| self.check(token)) {
            self.advance();
            return true;
        }

        false
    }
}

struct AstPrinter {}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_equal(&self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} == {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_not_equal(&self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} != {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_less(&self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} < {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_less_equal(&self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} <= {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_greater(&self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} > {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_greater_equal(&self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} >= {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_add(&self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} + {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_sub(&self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} - {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_mul(&self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} * {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_div(&self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} / {}}}", left.accept(self), right.accept(self))
    }

    fn visit_unary_bang(&self, expr: &Box<Expr>) -> String {
        format!("{{!{}}}", expr.accept(self))
    }

    fn visit_unary_minus(&self, expr: &Box<Expr>) -> String {
        format!("{{-{}}}", expr.accept(self))
    }

    fn visit_literal_string(&self, value: &String) -> String {
        format!("\"{}\"", value)
    }

    fn visit_literal_number(&self, value: &f64) -> String {
        value.to_string()
    }

    fn visit_false(&self) -> String {
        "false".to_string()
    }

    fn visit_true(&self) -> String {
        "true".to_string()
    }

    fn visit_nil(&self) -> String {
        "nil".to_string()
    }

    fn visit_identifier(&self, value: &String) -> String {
        value.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::lox::{scanner, Token};
    use rstest::*;

    use super::*;

    #[test]
    fn test_primary() -> Result<(), String> {
        ///////////////////////////////////////////////////////////////////////
        // Given a single literal number token
        let tokens = vec![Token::NumberLiteral(1.0)];

        let mut parser = Parser::new(tokens);

        ///////////////////////////////////////////////////////////////////////
        // When parsing the tokens
        let statements = parser.parse().map_err(|e| e.to_string())?;

        ///////////////////////////////////////////////////////////////////////
        // Then the result should be a single expression
        assert_eq!(statements.len(), 1);

        assert_eq!(statements[0], Expr::LiteralNumber(1.0));

        Ok(())
    }

    #[test]
    fn test_unary() -> Result<(), String> {
        ///////////////////////////////////////////////////////////////////////
        // Given a single unary minus token followed by a number literal token
        let tokens = vec![Token::Minus, Token::NumberLiteral(1.0)];

        let mut parser = Parser::new(tokens);

        ///////////////////////////////////////////////////////////////////////
        // When parsing the tokens
        let statements = parser.parse().map_err(|e| e.to_string())?;

        ///////////////////////////////////////////////////////////////////////
        // Then the result should be a single expression
        assert_eq!(statements.len(), 1);

        assert_eq!(
            statements[0],
            Expr::UnaryMinus(Box::new(Expr::LiteralNumber(1.0)))
        );

        Ok(())
    }

    #[test]
    fn test_binary_add() -> Result<(), String> {
        ///////////////////////////////////////////////////////////////////////
        // Given a single number literal token followed by a plus token and another number literal token
        let tokens = vec![
            Token::NumberLiteral(1.0),
            Token::Plus,
            Token::NumberLiteral(2.0),
        ];

        let mut parser = Parser::new(tokens);

        ///////////////////////////////////////////////////////////////////////
        // When parsing the tokens
        let statements = parser.parse().map_err(|e| e.to_string())?;

        ///////////////////////////////////////////////////////////////////////
        // Then the result should be a single expression
        assert_eq!(statements.len(), 1);

        assert_eq!(
            statements[0],
            Expr::BinaryAdd(
                Box::new(Expr::LiteralNumber(1.0)),
                Box::new(Expr::LiteralNumber(2.0))
            )
        );

        Ok(())
    }

    #[test]
    fn test_binary_add_div() -> Result<(), String> {
        ///////////////////////////////////////////////////////////////////////
        // Given tokens for "1.0 + 2.0 / 3.0"
        let tokens = vec![
            Token::NumberLiteral(1.0),
            Token::Plus,
            Token::NumberLiteral(2.0),
            Token::Slash,
            Token::NumberLiteral(3.0),
        ];

        let mut parser = Parser::new(tokens);

        ///////////////////////////////////////////////////////////////////////
        // When parsing the tokens
        let statements = parser.parse().map_err(|e| e.to_string())?;

        ///////////////////////////////////////////////////////////////////////
        // Then the result should be a single expression
        assert_eq!(statements.len(), 1);

        assert_eq!(
            statements[0],
            Expr::BinaryAdd(
                Box::new(Expr::LiteralNumber(1.0)),
                Box::new(Expr::BinaryDiv(
                    Box::new(Expr::LiteralNumber(2.0)),
                    Box::new(Expr::LiteralNumber(3.0))
                )),
            )
        );

        Ok(())
    }

    #[rstest]
    #[case("nil", "nil")]
    #[case("\"my literal\"", "\"my literal\"")]
    #[case("1.0 + 2.0 / 3.0", "{1 + {2 / 3}}")]
    fn test_ast_printer(
        #[case] source: String,
        #[case] expected_ast: String,
    ) -> Result<(), String> {
        ///////////////////////////////////////////////////////////////////////
        // Given the tokens produced by the scanner
        let mut scanner = scanner::Scanner::new(source);
        let tokens = scanner
            .scan_tokens()?
            .into_iter()
            .filter(|t| t != &Token::Eof)
            .collect();

        println!("{:?}", tokens);

        ///////////////////////////////////////////////////////////////////////
        // When parsing the tokens
        // FIXME: parser does no support EOF token
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().map_err(|e| e.to_string())?;

        ///////////////////////////////////////////////////////////////////////
        // Then the result should be a single expression
        assert_eq!(statements.len(), 1);

        // and when printing the AST
        let ast_printer = AstPrinter {};
        let ast_string = statements[0].accept(&ast_printer);

        // the resulting string should be equal to the expected
        assert_eq!(ast_string, expected_ast);

        Ok(())
    }
}
