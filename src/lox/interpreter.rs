use super::{function, Environment, ExprVisitor, Parser, Scanner, StmtVisitor, Value};

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
    fn visit_print(&mut self, expr: &Box<super::Expr>) -> Result<Value, String> {
        let value = expr.accept(self)?;
        println!("{}", value);
        Ok(Value::Nil)
    }

    fn visit_expr(&mut self, expr: &Box<super::Expr>) -> Result<Value, String> {
        // This is the only statement that I need to return a value
        expr.accept(self)
    }

    fn visit_var_declaration(
        &mut self,
        name: &String,
        initializer: &Option<Box<super::Expr>>,
    ) -> Result<Value, String> {
        match initializer {
            Some(expr) => {
                let value = expr.accept(self)?;
                self.environment.define_variable(name, value.clone());
                Ok(value)
            }
            None => {
                self.environment.define_variable(name, Value::Nil);
                Ok(Value::Nil)
            }
        }
    }

    fn visit_block(&mut self, stmts: &Vec<super::Stmt>) -> Result<Value, String> {
        self.environment.push_variable_stack();
        for stmt in stmts {
            match stmt.accept(self) {
                Ok(_) => {}
                Err(e) => {
                    // ugly, better to have some form of RAII for popping the environment
                    self.environment.pop_variable_stack();
                    return Err(e);
                }
            }
        }

        // all statements in the block were executed successfully
        self.environment.pop_variable_stack();
        Ok(Value::Nil)
    }

    fn visit_if(
        &mut self,
        condition: &Box<super::Expr>,
        then_branch: &Box<super::Stmt>,
        else_branch: &Option<Box<super::Stmt>>,
    ) -> Result<Value, String> {
        if condition.accept(self)?.is_truthy() {
            then_branch.accept(self)
        } else {
            match else_branch {
                Some(stmt) => stmt.accept(self),
                None => Ok(Value::Nil),
            }
        }
    }

    fn visit_while(
        &mut self,
        condition: &Box<super::Expr>,
        body: &Box<super::Stmt>,
    ) -> Result<Value, String> {
        while condition.accept(self)?.is_truthy() {
            match body.accept(self) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        Ok(Value::Nil)
    }

    fn visit_function_declaration(
        &mut self,
        name: &String,
        arguments: &Vec<String>,
        body: &Box<super::Stmt>,
    ) -> Result<Value, String> {
        let function = super::FunctionImpl::new(name.clone(), arguments.clone(), body.clone());

        self.environment.define_function(name, Box::new(function));

        Ok(Value::Nil)
    }
}

impl ExprVisitor<Result<Value, String>> for Interpreter {
    fn visit_assign(&mut self, left: &String, right: &Box<super::Expr>) -> Result<Value, String> {
        match self.environment.get_variable(left) {
            Some(_) => {
                let value = right.accept(self)?;
                self.environment.set_variable(left, value)?;

                // FIXME: need to avoid cloning the value
                Ok(self.environment.get_variable(left).unwrap().clone())
            }
            None => Err(format!("Undefined variable '{}'", left)),
        }
    }

    fn visit_binary_or(
        &mut self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<Value, String> {
        // first, evaluate the left expression
        let left_result = left.accept(self)?;

        return if left_result.is_truthy() {
            Ok(left_result)
        } else {
            right.accept(self)
        };
    }

    fn visit_binary_and(
        &mut self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<Value, String> {
        // first, evaluate the left expression
        let left_result = left.accept(self)?;

        return if left_result.is_truthy() {
            right.accept(self)
        } else {
            Ok(left_result)
        };
    }

    fn visit_binary_equal(
        &mut self,
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
        &mut self,
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
        &mut self,
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
        &mut self,
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
        &mut self,
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
        &mut self,
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
        &mut self,
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
            (Value::String(left), Value::Number(right)) => {
                Ok(Value::String(left + &right.to_string()))
            }
            (Value::Number(left), Value::String(right)) => {
                Ok(Value::String(left.to_string() + &right))
            }
            _ => Err(
                "Addition can only be applied to operands both numbers or both strings".to_string(),
            ),
        }
    }

    fn visit_binary_sub(
        &mut self,
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
        &mut self,
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
        &mut self,
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

    fn visit_unary_bang(&mut self, expr: &Box<super::Expr>) -> Result<Value, String> {
        match expr.accept(self)? {
            Value::Boolean(boolean_value) => Ok(Value::Boolean(!boolean_value)),
            Value::Number(_) => Err("Unary bang cannot be applied to a number".to_string()),
            Value::String(_) => Err("Unary bang cannot be applied to a string".to_string()),
            Value::Nil => Err("Unary bang cannot be applied to nil".to_string()),
            Value::Callable(_s) => Err("Unary bang cannot be applied to a function".to_string()),
        }
    }

    fn visit_unary_minus(&mut self, expr: &Box<super::Expr>) -> Result<Value, String> {
        match expr.accept(self)? {
            Value::Number(number_value) => Ok(Value::Number(-number_value)),
            Value::String(_) => Err("Unary minus cannot be applied to a string".to_string()),
            Value::Boolean(_) => Err("Unary minus cannot be applied to a boolean".to_string()),
            Value::Nil => Err("Unary minus cannot be applied to nil".to_string()),
            Value::Callable(_s) => Err("Unary minus cannot be applied to a function".to_string()),
        }
    }

    fn visit_call(
        &mut self,
        callee: &Box<super::Expr>,
        arguments: &Vec<super::Expr>,
    ) -> Result<Value, String> {
        // evaluate the callee expression
        let callee_value = callee.accept(self)?;

        match callee_value {
            Value::Callable(callable) => {
                // validate if the number of arguments is correct
                if callable.get_arg_count() != arguments.len() {
                    return Err(format!(
                        "Expected {} arguments, but got {}",
                        callable.get_arg_count(),
                        arguments.len()
                    ));
                }

                // evaluate the arguments
                let mut evaluated_arguments = Vec::new();
                for arg in arguments {
                    evaluated_arguments.push(arg.accept(self)?);
                }

                // create the environment to call the function
                // self.environment.branch_push();
                self.environment.push_variable_stack();

                // bind the arguments to the new function environment
                for (i, arg) in evaluated_arguments.iter().enumerate() {
                    // TODO: pop environment if there is an error
                    let arg_name = callable.get_arg_name(i)?;
                    self.environment.define_variable(&arg_name, arg.clone());
                }

                let body = callable.get_body();
                let body_result = body.accept(self);

                // self.environment.branch_pop();
                self.environment.pop_variable_stack();
                body_result
            }
            _ => Err("Can only call functions and classes".to_string()),
        }
    }

    fn visit_literal_string(&mut self, value: &String) -> Result<Value, String> {
        // FIXME: Is it possible to avoid the string clone?
        Ok(Value::String(value.clone()))
    }

    fn visit_literal_number(&mut self, value: &f64) -> Result<Value, String> {
        Ok(Value::Number(*value))
    }

    fn visit_false(&mut self) -> Result<Value, String> {
        Ok(Value::Boolean(false))
    }

    fn visit_true(&mut self) -> Result<Value, String> {
        Ok(Value::Boolean(true))
    }

    fn visit_nil(&mut self) -> Result<Value, String> {
        Ok(Value::Nil)
    }

    fn visit_identifier(&mut self, value: &String) -> Result<Value, String> {
        // FIXME: need to avoid cloning the value
        match self.environment.get_variable(value) {
            Some(value) => Ok(value.clone()),
            None => Err(format!("Undefined variable '{}'", value)),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;

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

    #[rstest]
    fn test_from_file(
        #[files("test-data/interpreter/*.lox")] base_path: PathBuf,
    ) -> Result<(), String> {
        ///////////////////////////////////////////////////////////////////////
        // Given the source code in the file
        let input_source = std::fs::read_to_string(base_path).map_err(|e| e.to_string())?;

        // and given an interpreters

        let mut interpreter = super::Interpreter::new();

        ///////////////////////////////////////////////////////////////////////
        // When executing the source code
        // Then there should be no error
        _ = interpreter.execute(input_source)?;

        Ok(())
    }
}
