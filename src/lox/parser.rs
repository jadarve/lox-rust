use crate::lox::ExprIdentifier;

use super::{Expr, ExprAssign, ExprVisitor, ParseTreeId, Stmt, StmtVisitor, Token};

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
    current_parse_tree_id: ParseTreeId,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
            current_parse_tree_id: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            let expr = self.parse_statement()?;
            statements.push(expr);
        }

        Ok(statements)
    }

    ///////////////////////////////////////////////////////////////////////////
    fn get_next_parse_tree_id(&mut self) -> ParseTreeId {
        let id = self.current_parse_tree_id;
        self.current_parse_tree_id += 1;
        id
    }

    ///////////////////////////////////////////////////////////////////////////
    // Statement parsing
    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        match self.peek() {
            Token::Print => self.parse_statement_print(),
            Token::Var => self.parse_statement_var_declaration(),
            Token::LeftBrace => self.parse_statement_block(),
            Token::If => self.parse_statement_if(),
            Token::While => self.parse_statement_while(),
            Token::Fun => self.parse_statement_function_declaration(),
            _ => self.parse_statement_expression(),
        }
    }

    fn parse_statement_block(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // consume the left brace token

        let mut statements = Vec::new();

        while !self.is_at_end() && !self.check(&Token::RightBrace) {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }

        if !self.match_token(vec![Token::RightBrace]) {
            return Err(ParseError {
                message: "Expected '}' after block.".to_string(),
            });
        }

        Ok(Stmt::Block(statements))
    }

    fn parse_statement_print(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // consume the print token

        let expr = self.parse_expression()?;

        if !self.match_token(vec![Token::Semicolon]) {
            return Err(ParseError {
                message: "Expected ';' after expression.".to_string(),
            });
        }

        Ok(Stmt::Print(Box::new(expr)))
    }

    fn parse_statement_expression(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.parse_expression()?;

        if !self.match_token(vec![Token::Semicolon]) {
            return Err(ParseError {
                message: "Expected ';' after expression.".to_string(),
            });
        }

        Ok(Stmt::Expr(Box::new(expr)))
    }

    fn parse_statement_var_declaration(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // consume the var token

        let identifier = match self.advance() {
            Token::Identifier(s) => s.clone(),
            _ => {
                return Err(ParseError {
                    message: "Expected identifier after var.".to_string(),
                });
            }
        };

        let initializer = if self.match_token(vec![Token::Equal]) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        if !self.match_token(vec![Token::Semicolon]) {
            return Err(ParseError {
                message: "Expected ';' after variable declaration.".to_string(),
            });
        }

        Ok(Stmt::VarDeclaration(identifier.clone(), initializer))
    }

    fn parse_statement_if(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // consume the if token

        if !self.match_token(vec![Token::LeftParenthesis]) {
            return Err(ParseError {
                message: "Expected '(' after if.".to_string(),
            });
        }

        let condition = Box::new(self.parse_expression()?);

        if !self.match_token(vec![Token::RightParenthesis]) {
            return Err(ParseError {
                message: "Expected ')' after if condition.".to_string(),
            });
        }

        let then_branch = Box::new(self.parse_statement()?);

        let else_branch = if self.match_token(vec![Token::Else]) {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(Stmt::If(condition, then_branch, else_branch))
    }

    fn parse_statement_while(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // consume the while token

        if !self.match_token(vec![Token::LeftParenthesis]) {
            return Err(ParseError {
                message: "Expected '(' after while.".to_string(),
            });
        }

        let condition = Box::new(self.parse_expression()?);

        if !self.match_token(vec![Token::RightParenthesis]) {
            return Err(ParseError {
                message: "Expected ')' after while condition.".to_string(),
            });
        }

        let body = Box::new(self.parse_statement()?);

        Ok(Stmt::While(condition, body))
    }

    fn parse_statement_function_declaration(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // consume the fun token

        let name = match self.advance() {
            Token::Identifier(s) => s.clone(),
            _ => {
                return Err(ParseError {
                    message: "Expected identifier after fun.".to_string(),
                });
            }
        };

        if !self.match_token(vec![Token::LeftParenthesis]) {
            return Err(ParseError {
                message: "Expected '(' after function name.".to_string(),
            });
        }

        let mut arguments = Vec::new();

        while !self.is_at_end() && !self.check(&Token::RightParenthesis) {
            match self.advance() {
                Token::Identifier(s) => arguments.push(s.clone()),
                _ => {
                    return Err(ParseError {
                        message: "Expected identifier in function arguments.".to_string(),
                    });
                }
            }

            if !self.match_token(vec![Token::Comma]) {
                break;
            }
        }

        if !self.match_token(vec![Token::RightParenthesis]) {
            return Err(ParseError {
                message: "Expected ')' after function arguments.".to_string(),
            });
        }

        let body = Box::new(self.parse_statement()?);

        let body_wrapper = Stmt::Block(vec![*body]);

        Ok(Stmt::FunctionDeclaration(
            name,
            arguments,
            Box::new(body_wrapper),
        ))
    }

    ///////////////////////////////////////////////////////////////////////////
    // Expression parsing
    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_expression_assignment()
    }

    fn parse_expression_assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_expression_or()?;

        if self.match_token(vec![Token::Equal]) {
            let value = self.parse_expression_or()?;

            match expr {
                Expr::Identifier(s) => Ok(Expr::Assign(super::ExprAssign {
                    parse_tree_id: self.get_next_parse_tree_id(),
                    left: s.id,
                    right: Box::new(value),
                })),
                _ => Err(ParseError {
                    message: "Invalid assignment target.".to_string(),
                }),
            }
        } else {
            Ok(expr)
        }
    }

    fn parse_expression_or(&mut self) -> Result<Expr, ParseError> {
        let mut left_expr = self.parse_expression_and()?;

        while self.match_token(vec![Token::Or]) {
            let operator = self.previous().clone();
            let right_expr = self.parse_expression_and()?;

            left_expr = match operator {
                Token::Or => Expr::BinaryOr(Box::new(left_expr), Box::new(right_expr)),
                _ => {
                    return Err(ParseError {
                        message: format!("Unexpected token while parsing or: {:?}", operator),
                    });
                }
            };
        }

        Ok(left_expr)
    }

    fn parse_expression_and(&mut self) -> Result<Expr, ParseError> {
        let mut left_expr = self.parse_expression_equality()?;

        while self.match_token(vec![Token::And]) {
            let operator = self.previous().clone();
            let right_expr = self.parse_expression_equality()?;

            left_expr = match operator {
                Token::And => Expr::BinaryAnd(Box::new(left_expr), Box::new(right_expr)),
                _ => {
                    return Err(ParseError {
                        message: format!("Unexpected token while parsing and: {:?}", operator),
                    });
                }
            };
        }

        Ok(left_expr)
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
            _ => self.parse_expression_call(),
        }
    }

    fn parse_expression_call(&mut self) -> Result<Expr, ParseError> {
        let callee = self.parse_expression_primary()?;

        if !self.match_token(vec![Token::LeftParenthesis]) {
            return Ok(callee);
        }

        // match for empty argument list
        if self.match_token(vec![Token::RightParenthesis]) {
            return Ok(Expr::Call(Box::new(callee), Vec::new()));
        }

        let mut arguments = Vec::new();

        loop {
            arguments.push(self.parse_expression()?);

            if !self.match_token(vec![Token::Comma]) {
                break;
            }
        }

        if !self.match_token(vec![Token::RightParenthesis]) {
            return Err(ParseError {
                message: "Expected ')' for closing function call.".to_string(),
            });
        }

        Ok(Expr::Call(Box::new(callee), arguments))
    }

    fn parse_expression_primary(&mut self) -> Result<Expr, ParseError> {
        // had to clone the previous token because cannot borrow self mutably inside for get_next_parse_tree_id()
        let previous_token = self.previous().clone();

        match &previous_token {
            Token::NumberLiteral(n) => Ok(Expr::LiteralNumber(*n)),
            Token::StringLiteral(s) => Ok(Expr::LiteralString(s.clone())),
            Token::Identifier(s) => Ok(Expr::Identifier(ExprIdentifier {
                parse_tree_id: self.get_next_parse_tree_id(),
                id: s.clone(),
            })),
            Token::False => Ok(Expr::False),
            Token::True => Ok(Expr::True),
            Token::Nil => Ok(Expr::Nil),
            Token::LeftParenthesis => self.parse_expression_parenthesis(),
            _ => Err(ParseError {
                message: format!(
                    "Unexpected token while parsing primary: {:?}",
                    self.previous()
                ),
            }),
        }
    }

    fn parse_expression_parenthesis(&mut self) -> Result<Expr, ParseError> {
        // the left parenthesis has already been consumed

        let expr = self.parse_expression()?;

        if !self.match_token(vec![Token::RightParenthesis]) {
            return Err(ParseError {
                message: "Expected ')' after expression.".to_string(),
            });
        }

        Ok(expr)
    }

    ///////////////////////////////////////////////////////////////////////////
    // Auxiliary methods
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.peek() == &Token::Eof
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
    fn visit_assign(&mut self, assign: &ExprAssign) -> String {
        format!("{{{} = {}}}", assign.left, assign.right.accept(self))
    }

    fn visit_binary_or(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} or {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_and(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} and {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_equal(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} == {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_not_equal(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} != {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_less(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} < {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_less_equal(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} <= {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_greater(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} > {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_greater_equal(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} >= {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_add(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} + {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_sub(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} - {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_mul(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} * {}}}", left.accept(self), right.accept(self))
    }

    fn visit_binary_div(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> String {
        format!("{{{} / {}}}", left.accept(self), right.accept(self))
    }

    fn visit_unary_bang(&mut self, expr: &Box<Expr>) -> String {
        format!("{{!{}}}", expr.accept(self))
    }

    fn visit_unary_minus(&mut self, expr: &Box<Expr>) -> String {
        format!("{{-{}}}", expr.accept(self))
    }

    fn visit_call(&mut self, callee: &Box<Expr>, arguments: &Vec<Expr>) -> String {
        let mut call_str = format!("{{call {}(", callee.accept(self));

        for (i, arg) in arguments.iter().enumerate() {
            call_str.push_str(&arg.accept(self));

            if i < arguments.len() - 1 {
                call_str.push_str(", ");
            }
        }

        call_str.push_str(")}");

        call_str
    }

    fn visit_literal_string(&mut self, value: &String) -> String {
        format!("\"{}\"", value)
    }

    fn visit_literal_number(&mut self, value: &f64) -> String {
        value.to_string()
    }

    fn visit_false(&mut self) -> String {
        "false".to_string()
    }

    fn visit_true(&mut self) -> String {
        "true".to_string()
    }

    fn visit_nil(&mut self) -> String {
        "nil".to_string()
    }

    fn visit_identifier(&mut self, value: &ExprIdentifier) -> String {
        value.id.clone()
    }
}

impl StmtVisitor<String> for AstPrinter {
    fn visit_print(&mut self, expr: &Box<Expr>) -> String {
        format!("{{print {}}}", expr.accept(self))
    }

    fn visit_expr(&mut self, expr: &Box<Expr>) -> String {
        expr.accept(self)
    }

    fn visit_var_declaration(&mut self, name: &String, initializer: &Option<Box<Expr>>) -> String {
        match initializer {
            Some(expr) => format!("{{var {} = {}}}", name, expr.accept(self)),
            None => format!("{{var {}}}", name),
        }
    }

    fn visit_block(&mut self, stmts: &Vec<Stmt>) -> String {
        let mut block = String::from("{");

        for stmt in stmts {
            block.push_str(&stmt.accept(self));
        }

        block.push_str("}");

        block
    }

    fn visit_if(
        &mut self,
        condition: &Box<Expr>,
        then_branch: &Box<Stmt>,
        else_branch: &Option<Box<Stmt>>,
    ) -> String {
        let mut if_stmt = format!(
            "{{if {} then {} ",
            condition.accept(self),
            then_branch.accept(self)
        );

        if let Some(else_branch) = else_branch {
            if_stmt.push_str(&format!(" else {}", else_branch.accept(self)));
        }

        if_stmt.push_str("}");

        if_stmt
    }

    fn visit_while(&mut self, condition: &Box<Expr>, body: &Box<Stmt>) -> String {
        format!(
            "{{while {} then {}}}",
            condition.accept(self),
            body.accept(self)
        )
    }

    fn visit_function_declaration(
        &mut self,
        name: &String,
        arguments: &Vec<String>,
        body: &Box<Stmt>,
    ) -> String {
        let mut function_decl = format!("{{fun {}(", name);

        for (i, arg) in arguments.iter().enumerate() {
            function_decl.push_str(arg);

            if i < arguments.len() - 1 {
                function_decl.push_str(", ");
            }
        }

        function_decl.push_str(") ");
        function_decl.push_str(format!("{{ {} }}", body.accept(self)).as_str());

        function_decl
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
        let tokens = vec![Token::NumberLiteral(1.0), Token::Semicolon];

        let mut parser = Parser::new(tokens);

        ///////////////////////////////////////////////////////////////////////
        // When parsing the tokens
        let statements = parser.parse().map_err(|e| e.to_string())?;

        ///////////////////////////////////////////////////////////////////////
        // Then the result should be a single expression
        assert_eq!(statements.len(), 1);

        assert_eq!(
            statements[0],
            Stmt::Expr(Box::new(Expr::LiteralNumber(1.0)))
        );

        Ok(())
    }

    #[test]
    fn test_unary() -> Result<(), String> {
        ///////////////////////////////////////////////////////////////////////
        // Given a single unary minus token followed by a number literal token
        let tokens = vec![Token::Minus, Token::NumberLiteral(1.0), Token::Semicolon];

        let mut parser = Parser::new(tokens);

        ///////////////////////////////////////////////////////////////////////
        // When parsing the tokens
        let statements = parser.parse().map_err(|e| e.to_string())?;

        ///////////////////////////////////////////////////////////////////////
        // Then the result should be a single expression
        assert_eq!(statements.len(), 1);

        assert_eq!(
            statements[0],
            Stmt::Expr(Box::new(Expr::UnaryMinus(Box::new(Expr::LiteralNumber(
                1.0
            )))))
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
            Token::Semicolon,
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
            Stmt::Expr(Box::new(Expr::BinaryAdd(
                Box::new(Expr::LiteralNumber(1.0)),
                Box::new(Expr::LiteralNumber(2.0))
            )))
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
            Token::Semicolon,
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
            Stmt::Expr(Box::new(Expr::BinaryAdd(
                Box::new(Expr::LiteralNumber(1.0)),
                Box::new(Expr::BinaryDiv(
                    Box::new(Expr::LiteralNumber(2.0)),
                    Box::new(Expr::LiteralNumber(3.0))
                ))
            )),)
        );

        Ok(())
    }

    #[rstest]
    // #[case("nil;", "nil")]
    // #[case("\"my literal\";", "\"my literal\"")]
    // #[case("1.0 + 2.0 / 3.0;", "{1 + {2 / 3}}")]
    // #[case("(1.0 + 2.0) / 3.0;", "{{1 + 2} / 3}")]
    // #[case("var a = 2 + 2;", "{var a = {2 + 2}}")]
    #[case("say_hello();", "{call say_hello()}")]
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
        let mut ast_printer = AstPrinter {};
        let ast_string = statements[0].accept(&mut ast_printer);

        // the resulting string should be equal to the expected
        assert_eq!(ast_string, expected_ast);

        Ok(())
    }
}
