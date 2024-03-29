use super::{Environment, ExprVisitor, Parser, Scanner, StmtVisitor, Value};

pub struct Interpreter {
    environment: Box<dyn Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Box::new(super::EnvironmentImpl::new()),
        }
    }

    pub fn execute(&mut self, source: String) -> Result<Value, String> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;

        let mut parser = Parser::new(tokens);
        let statements = parser.parse().map_err(|e| e.to_string())?;

        match statements.len() {
            1 => statements[0].accept(self),
            _ => {
                for stmt in statements {
                    stmt.accept(self)?;
                }
                Ok(Value::Nil)
            }
        }
    }
}

impl StmtVisitor<Result<Value, String>> for Interpreter {
    fn visit_print(&self, expr: &Box<super::Expr>) -> Result<Value, String> {
        let value = expr.accept(self)?;
        println!("{}", value);
        Ok(value)
    }

    fn visit_expr(&self, expr: &Box<super::Expr>) -> Result<Value, String> {
        expr.accept(self)
    }
}

impl ExprVisitor<Result<Value, String>> for Interpreter {
    fn visit_binary_or(
        &self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<Value, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        // then evaluate the or
        match (left_result, right_result) {
            (Value::Boolean(left), Value::Boolean(right)) => Ok(Value::Boolean(left || right)),
            _ => Err("Or operator can only be applied to booleans".to_string()),
        }
    }

    fn visit_binary_and(
        &self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<Value, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        // then evaluate the and
        match (left_result, right_result) {
            (Value::Boolean(left), Value::Boolean(right)) => Ok(Value::Boolean(left && right)),
            _ => Err("And operator can only be applied to booleans".to_string()),
        }
    }

    fn visit_binary_equal(
        &self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<Value, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        // then evaluate the comparison
        match (left_result, right_result) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Boolean(left == right)),
            (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left == right)),
            (Value::Boolean(left), Value::Boolean(right)) => Ok(Value::Boolean(left == right)),
            (Value::Nil, Value::Nil) => Ok(Value::Boolean(true)),
            // TODO: compare objects
            _ => Ok(Value::Boolean(false)),
        }
    }

    fn visit_binary_not_equal(
        &self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<Value, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        // then evaluate the comparison
        match (left_result, right_result) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Boolean(left != right)),
            (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left != right)),
            (Value::Boolean(left), Value::Boolean(right)) => Ok(Value::Boolean(left != right)),
            (Value::Nil, Value::Nil) => Ok(Value::Boolean(false)),
            // TODO: compare objects
            _ => Ok(Value::Boolean(true)),
        }
    }

    fn visit_binary_less(
        &self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<Value, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        // then evaluate the comparison
        match (left_result, right_result) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Boolean(left < right)),
            (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left < right)),
            _ => Err(
                "Less comparison can only be applied to operands both numbers or both strings"
                    .to_string(),
            ),
        }
    }

    fn visit_binary_less_equal(
        &self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<Value, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        // then evaluate the comparison
        match (left_result, right_result) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Boolean(left <= right)),
            (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left <= right)),
            _ => Err(
                "Less or equal comparison can only be applied to operands both numbers or both strings".to_string(),
            ),
        }
    }

    fn visit_binary_greater(
        &self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<Value, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        // then evaluate the comparison
        match (left_result, right_result) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Boolean(left > right)),
            (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left > right)),
            _ => Err(
                "Greater comparison can only be applied to operands both numbers or both strings"
                    .to_string(),
            ),
        }
    }

    fn visit_binary_greater_equal(
        &self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<Value, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        // then evaluate the comparison
        match (left_result, right_result) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Boolean(left >= right)),
            (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left >= right)),
            _ => Err(
                "Greater or equal comparison can only be applied to operands both numbers or both strings".to_string(),
            ),
        }
    }

    fn visit_binary_add(
        &self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<Value, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        // then evaluate the addition
        match (left_result, right_result) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left + right)),
            (Value::String(left), Value::String(right)) => Ok(Value::String(left + &right)),
            _ => Err(
                "Addition can only be applied to operands both numbers or both strings".to_string(),
            ),
        }
    }

    fn visit_binary_sub(
        &self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<Value, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        // then evaluate the subtraction
        match (left_result, right_result) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left - right)),
            _ => Err("Subtraction can only be applied to numbers".to_string()),
        }
    }

    fn visit_binary_mul(
        &self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<Value, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        // then evaluate the multiplication
        match (left_result, right_result) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left * right)),
            _ => Err("Multiplication can only be applied to numbers".to_string()),
        }
    }

    fn visit_binary_div(
        &self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<Value, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        // then evaluate the division
        match (left_result, right_result) {
            (Value::Number(left), Value::Number(right)) => {
                if right == 0.0 {
                    return Err("Division by zero".to_string());
                }
                Ok(Value::Number(left / right))
            }
            _ => Err("Division can only be applied to numbers".to_string()),
        }
    }

    fn visit_unary_bang(&self, expr: &Box<super::Expr>) -> Result<Value, String> {
        match expr.accept(self)? {
            Value::Boolean(boolean_value) => Ok(Value::Boolean(!boolean_value)),
            Value::Number(_) => Err("Unary bang cannot be applied to a number".to_string()),
            Value::String(_) => Err("Unary bang cannot be applied to a string".to_string()),
            Value::Nil => Err("Unary bang cannot be applied to nil".to_string()),
        }
    }

    fn visit_unary_minus(&self, expr: &Box<super::Expr>) -> Result<Value, String> {
        match expr.accept(self)? {
            Value::Number(number_value) => Ok(Value::Number(-number_value)),
            Value::String(_) => Err("Unary minus cannot be applied to a string".to_string()),
            Value::Boolean(_) => Err("Unary minus cannot be applied to a boolean".to_string()),
            Value::Nil => Err("Unary minus cannot be applied to nil".to_string()),
        }
    }

    fn visit_literal_string(&self, value: &String) -> Result<Value, String> {
        // FIXME: Is it possible to avoid the string clone?
        Ok(Value::String(value.clone()))
    }

    fn visit_literal_number(&self, value: &f64) -> Result<Value, String> {
        Ok(Value::Number(*value))
    }

    fn visit_false(&self) -> Result<Value, String> {
        Ok(Value::Boolean(false))
    }

    fn visit_true(&self) -> Result<Value, String> {
        Ok(Value::Boolean(true))
    }

    fn visit_nil(&self) -> Result<Value, String> {
        Ok(Value::Nil)
    }

    fn visit_identifier(&self, _value: &String) -> Result<Value, String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::Value;
    use rstest::*;

    #[rstest]
    #[case::addition("1 + (2);", Value::Number(3.0))]
    #[case::arithmetic("(2 + 3) * (2 * 2);", Value::Number(20.0))]
    #[case::comparison("1 < 2;", Value::Boolean(true))]
    #[case::comparison_equal("1 == 1;", Value::Boolean(true))]
    #[case::comparison_equal_nil("nil == nil;", Value::Boolean(true))]
    #[case::comparison_equal_string("\"my string\" == \"my string\";", Value::Boolean(true))]
    #[case::comparison_not_equal_nil("nil != nil;", Value::Boolean(false))]
    fn test_interpreter_expressions(
        #[case] source: String,
        #[case] expected: Value,
    ) -> Result<(), String> {
        ///////////////////////////////////////////////////////////////////////
        // Given an interpreter, the source code to run and the expected result
        let mut interpreter = super::Interpreter::new();

        ///////////////////////////////////////////////////////////////////////
        // When executing the source code
        let result = interpreter.execute(source)?;

        ///////////////////////////////////////////////////////////////////////
        // Then the result should be the expected value
        assert_eq!(result, expected);

        Ok(())
    }
}
