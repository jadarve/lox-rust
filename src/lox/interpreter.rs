use super::{
    new_value_box, value, Environment, ExprVisitor, Parser, Scanner, StmtVisitor, Value, ValueBox,
};

pub struct Interpreter {
    environment: Box<dyn Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Box::new(super::EnvironmentImpl::new()),
        }
    }

    pub fn execute(&mut self, source: String) -> Result<ValueBox, String> {
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
                Ok(new_value_box(Value::Nil))
            }
        }
    }
}

impl StmtVisitor<Result<ValueBox, String>> for Interpreter {
    fn visit_print(&mut self, expr: &Box<super::Expr>) -> Result<ValueBox, String> {
        let value = expr.accept(self)?;
        let value_guard = value.read().map_err(|e| e.to_string())?;
        println!("{}", value_guard.as_ref());
        Ok(new_value_box(Value::Nil))
    }

    fn visit_expr(&mut self, expr: &Box<super::Expr>) -> Result<ValueBox, String> {
        // This is the only statement that I need to return a value
        expr.accept(self)
    }

    fn visit_var_declaration(
        &mut self,
        name: &String,
        initializer: &Option<Box<super::Expr>>,
    ) -> Result<ValueBox, String> {
        match initializer {
            Some(expr) => {
                let value_result = expr.accept(self)?;
                let value_owned = {
                    let value_guard = value_result.read().map_err(|e| e.to_string())?;
                    value_guard.as_ref().to_owned()
                };

                self.environment.define_variable(name, value_owned);
                self.environment.get_variable(name).ok_or(format!(
                    "error defining variable \"{name}\". Variable not found after definition"
                ))
            }
            None => {
                self.environment.define_variable(name, Value::Nil);
                Ok(new_value_box(Value::Nil))
            }
        }
    }

    fn visit_block(&mut self, stmts: &Vec<super::Stmt>) -> Result<ValueBox, String> {
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
        Ok(new_value_box(Value::Nil))
    }

    fn visit_if(
        &mut self,
        condition: &Box<super::Expr>,
        then_branch: &Box<super::Stmt>,
        else_branch: &Option<Box<super::Stmt>>,
    ) -> Result<ValueBox, String> {
        // accept the condition and check if it is truthy, locking the result only for the condition evaluation
        if condition
            .accept(self)?
            .read()
            .map_err(|e| e.to_string())?
            .is_truthy()
        {
            then_branch.accept(self)
        } else {
            match else_branch {
                Some(stmt) => stmt.accept(self),
                None => Ok(new_value_box(Value::Nil)),
            }
        }
    }

    fn visit_while(
        &mut self,
        condition: &Box<super::Expr>,
        body: &Box<super::Stmt>,
    ) -> Result<ValueBox, String> {
        // while the condition is truthy, execute the body
        // Lock the result of the evaluation only while evaluating the condition of the while, then release
        // the lock for running the body
        while condition
            .accept(self)?
            .read()
            .map_err(|e| e.to_string())?
            .is_truthy()
        {
            match body.accept(self) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        Ok(new_value_box(Value::Nil))
    }

    fn visit_function_declaration(
        &mut self,
        name: &String,
        arguments: &Vec<String>,
        body: &Box<super::Stmt>,
    ) -> Result<ValueBox, String> {
        let function = super::FunctionImpl::new(name.clone(), arguments.clone(), body.clone());

        self.environment.define_function(name, Box::new(function));

        Ok(new_value_box(Value::Nil))
    }
}

impl ExprVisitor<Result<ValueBox, String>> for Interpreter {
    fn visit_assign(
        &mut self,
        left: &String,
        right: &Box<super::Expr>,
    ) -> Result<ValueBox, String> {
        if let Some(left_variable) = self.environment.get_variable(left) {
            let right_result = right.accept(self)?;
            let right_guard = right_result.read().map_err(|e| e.to_string())?;

            let mut left_guard = left_variable.write().map_err(|e| e.to_string())?;
            *left_guard.as_mut() = *right_guard.to_owned();

            Ok(left_variable.to_owned())
        } else {
            return Err(format!("Undefined variable '{}'", left));
        }
    }

    fn visit_binary_or(
        &mut self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<ValueBox, String> {
        // first, evaluate the left expression
        let left_result = left.accept(self)?;

        // lock left result only to check if it is truthy, then release before evaluating right, if needed
        let left_is_truthy = {
            let left_guard = left_result.read().map_err(|e| e.to_string())?;
            left_guard.is_truthy()
        };

        return if left_is_truthy {
            Ok(left_result)
        } else {
            right.accept(self)
        };
    }

    fn visit_binary_and(
        &mut self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<ValueBox, String> {
        // first, evaluate the left expression
        let left_result = left.accept(self)?;

        // lock left result only to check if it is truthy, then release before evaluating right, if needed
        let left_is_truthy = {
            let left_guard = left_result.read().map_err(|e| e.to_string())?;
            left_guard.is_truthy()
        };

        return if left_is_truthy {
            right.accept(self)
        } else {
            Ok(left_result)
        };
    }

    fn visit_binary_equal(
        &mut self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<ValueBox, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        let left_guard = left_result.read().map_err(|e| e.to_string())?;
        let right_guard = right_result.read().map_err(|e| e.to_string())?;

        // then evaluate the comparison
        match (left_guard.as_ref(), right_guard.as_ref()) {
            (Value::Number(left), Value::Number(right)) => {
                Ok(new_value_box(Value::Boolean(left == right)))
            }
            (Value::String(left), Value::String(right)) => {
                Ok(new_value_box(Value::Boolean(left == right)))
            }
            (Value::Boolean(left), Value::Boolean(right)) => {
                Ok(new_value_box(Value::Boolean(left == right)))
            }
            (Value::Nil, Value::Nil) => Ok(new_value_box(Value::Boolean(true))),
            // TODO: compare objects
            _ => Ok(new_value_box(Value::Boolean(false))),
        }
    }

    fn visit_binary_not_equal(
        &mut self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<ValueBox, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        let left_guard = left_result.read().map_err(|e| e.to_string())?;
        let right_guard = right_result.read().map_err(|e| e.to_string())?;

        // then evaluate the comparison
        match (left_guard.as_ref(), right_guard.as_ref()) {
            (Value::Number(left), Value::Number(right)) => {
                Ok(new_value_box(Value::Boolean(left != right)))
            }
            (Value::String(left), Value::String(right)) => {
                Ok(new_value_box(Value::Boolean(left != right)))
            }
            (Value::Boolean(left), Value::Boolean(right)) => {
                Ok(new_value_box(Value::Boolean(left != right)))
            }
            (Value::Nil, Value::Nil) => Ok(new_value_box(Value::Boolean(false))),
            // TODO: compare objects
            _ => Ok(new_value_box(Value::Boolean(true))),
        }
    }

    fn visit_binary_less(
        &mut self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<ValueBox, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        let left_guard = left_result.read().map_err(|e| e.to_string())?;
        let right_guard = right_result.read().map_err(|e| e.to_string())?;

        // then evaluate the comparison
        match (left_guard.as_ref(), right_guard.as_ref()) {
            (Value::Number(left), Value::Number(right)) => {
                Ok(new_value_box(Value::Boolean(left < right)))
            }
            (Value::String(left), Value::String(right)) => {
                Ok(new_value_box(Value::Boolean(left < right)))
            }
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
    ) -> Result<ValueBox, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        let left_guard = left_result.read().map_err(|e| e.to_string())?;
        let right_guard = right_result.read().map_err(|e| e.to_string())?;

        // then evaluate the comparison
        match (left_guard.as_ref(), right_guard.as_ref()) {
            (Value::Number(left), Value::Number(right)) => Ok(new_value_box(Value::Boolean(left <= right))),
            (Value::String(left), Value::String(right)) => Ok(new_value_box(Value::Boolean(left <= right))),
            _ => Err(
                "Less or equal comparison can only be applied to operands both numbers or both strings".to_string(),
            ),
        }
    }

    fn visit_binary_greater(
        &mut self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<ValueBox, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        let left_guard = left_result.read().map_err(|e| e.to_string())?;
        let right_guard = right_result.read().map_err(|e| e.to_string())?;

        // then evaluate the comparison
        match (left_guard.as_ref(), right_guard.as_ref()) {
            (Value::Number(left), Value::Number(right)) => {
                Ok(new_value_box(Value::Boolean(left > right)))
            }
            (Value::String(left), Value::String(right)) => {
                Ok(new_value_box(Value::Boolean(left > right)))
            }
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
    ) -> Result<ValueBox, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        let left_guard = left_result.read().map_err(|e| e.to_string())?;
        let right_guard = right_result.read().map_err(|e| e.to_string())?;

        // then evaluate the comparison
        match (left_guard.as_ref(), right_guard.as_ref()) {
            (Value::Number(left), Value::Number(right)) => Ok(new_value_box(Value::Boolean(left >= right))),
            (Value::String(left), Value::String(right)) => Ok(new_value_box(Value::Boolean(left >= right))),
            _ => Err(
                "Greater or equal comparison can only be applied to operands both numbers or both strings".to_string(),
            ),
        }
    }

    fn visit_binary_add(
        &mut self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<ValueBox, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        let left_guard = left_result.read().map_err(|e| e.to_string())?;
        let right_guard = right_result.read().map_err(|e| e.to_string())?;

        // then evaluate the addition
        match (left_guard.as_ref(), right_guard.as_ref()) {
            (Value::Number(left), Value::Number(right)) => {
                Ok(new_value_box(Value::Number(left + right)))
            }
            (Value::String(left), Value::String(right)) => {
                Ok(new_value_box(Value::String(format!("{left}{right}"))))
            }
            (Value::String(left), Value::Number(right)) => Ok(new_value_box(Value::String(
                left.to_owned() + &right.to_string(),
            ))),
            (Value::Number(left), Value::String(right)) => {
                Ok(new_value_box(Value::String(left.to_string() + &right)))
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
    ) -> Result<ValueBox, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        let left_guard = left_result.read().map_err(|e| e.to_string())?;
        let right_guard = right_result.read().map_err(|e| e.to_string())?;

        // then evaluate the subtraction
        match (left_guard.as_ref(), right_guard.as_ref()) {
            (Value::Number(left), Value::Number(right)) => {
                Ok(new_value_box(Value::Number(left - right)))
            }
            _ => Err("Subtraction can only be applied to numbers".to_string()),
        }
    }

    fn visit_binary_mul(
        &mut self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<ValueBox, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        let left_guard = left_result.read().map_err(|e| e.to_string())?;
        let right_guard = right_result.read().map_err(|e| e.to_string())?;

        // then evaluate the multiplication
        match (left_guard.as_ref(), right_guard.as_ref()) {
            (Value::Number(left), Value::Number(right)) => {
                Ok(new_value_box(Value::Number(left * right)))
            }
            _ => Err("Multiplication can only be applied to numbers".to_string()),
        }
    }

    fn visit_binary_div(
        &mut self,
        left: &Box<super::Expr>,
        right: &Box<super::Expr>,
    ) -> Result<ValueBox, String> {
        // first, evaluate the left and right expressions
        let left_result = left.accept(self)?;
        let right_result = right.accept(self)?;

        let left_guard = left_result.read().map_err(|e| e.to_string())?;
        let right_guard = right_result.read().map_err(|e| e.to_string())?;

        // then evaluate the division
        match (left_guard.as_ref(), right_guard.as_ref()) {
            (Value::Number(left), Value::Number(right)) => {
                if *right == 0.0 {
                    return Err("Division by zero".to_string());
                }
                Ok(new_value_box(Value::Number(left / right)))
            }
            _ => Err("Division can only be applied to numbers".to_string()),
        }
    }

    fn visit_unary_bang(&mut self, expr: &Box<super::Expr>) -> Result<ValueBox, String> {
        let expr_result = expr.accept(self)?;
        let result_guard = expr_result.read().map_err(|e| e.to_string())?;

        match result_guard.as_ref() {
            Value::Boolean(boolean_value) => Ok(new_value_box(Value::Boolean(!boolean_value))),
            Value::Number(_) => Err("Unary bang cannot be applied to a number".to_string()),
            Value::String(_) => Err("Unary bang cannot be applied to a string".to_string()),
            Value::Nil => Err("Unary bang cannot be applied to nil".to_string()),
            Value::Callable(_s) => Err("Unary bang cannot be applied to a function".to_string()),
        }
    }

    fn visit_unary_minus(&mut self, expr: &Box<super::Expr>) -> Result<ValueBox, String> {
        let expr_result = expr.accept(self)?;
        let result_guard = expr_result.read().map_err(|e| e.to_string())?;

        match result_guard.as_ref() {
            Value::Number(number_value) => Ok(new_value_box(Value::Number(-number_value))),
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
    ) -> Result<ValueBox, String> {
        // evaluate the callee expression
        let callee_result = callee.accept(self)?;
        let callee_guard = callee_result.read().map_err(|e| e.to_string())?;

        match callee_guard.as_ref() {
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

                    let arg_guard = arg
                        .try_read()
                        .map_err(|e| format!("Error reading argument {arg_name}: {e}"))?;

                    self.environment
                        .define_variable(&arg_name, arg_guard.as_ref().to_owned());
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

    fn visit_literal_string(&mut self, value: &String) -> Result<ValueBox, String> {
        // FIXME: Is it possible to avoid the string clone?
        Ok(new_value_box(Value::String(value.clone())))
    }

    fn visit_literal_number(&mut self, value: &f64) -> Result<ValueBox, String> {
        Ok(new_value_box(Value::Number(*value)))
    }

    fn visit_false(&mut self) -> Result<ValueBox, String> {
        Ok(new_value_box(Value::Boolean(false)))
    }

    fn visit_true(&mut self) -> Result<ValueBox, String> {
        Ok(new_value_box(Value::Boolean(true)))
    }

    fn visit_nil(&mut self) -> Result<ValueBox, String> {
        Ok(new_value_box(Value::Nil))
    }

    fn visit_identifier(&mut self, value: &String) -> Result<ValueBox, String> {
        // FIXME: need to avoid cloning the value
        match self.environment.get_variable(value) {
            Some(value) => Ok(value.clone()),
            None => Err(format!("Undefined variable '{}'", value)),
        }

        // self.environment
        //     .get_variable(value.as_str())
        //     .ok_or(format!("Undefined variable '{}'", value))
    }
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use crate::lox::new_value_box;

    use super::{Value, ValueBox};
    use rstest::*;

    #[rstest]
    #[case::addition("1 + (2);", new_value_box(Value::Number(3.0)))]
    #[case::arithmetic("(2 + 3) * (2 * 2);", new_value_box(Value::Number(20.0)))]
    #[case::comparison("1 < 2;", new_value_box(Value::Boolean(true)))]
    #[case::comparison_equal("1 == 1;", new_value_box(Value::Boolean(true)))]
    #[case::comparison_equal_nil("nil == nil;", new_value_box(Value::Boolean(true)))]
    #[case::comparison_equal_string(
        "\"my string\" == \"my string\";",
        new_value_box(Value::Boolean(true))
    )]
    #[case::comparison_not_equal_nil("nil != nil;", new_value_box(Value::Boolean(false)))]
    fn test_interpreter_expressions(
        #[case] source: String,
        #[case] expected: ValueBox,
    ) -> Result<(), String> {
        ///////////////////////////////////////////////////////////////////////
        // Given an interpreter, the source code to run and the expected result
        let mut interpreter = super::Interpreter::new();

        ///////////////////////////////////////////////////////////////////////
        // When executing the source code
        let result = interpreter.execute(source)?;

        ///////////////////////////////////////////////////////////////////////
        // Then the result should be the expected value
        let result_guard = result.try_read().map_err(|e| e.to_string())?;
        let expected_guard = expected.try_read().map_err(|e| e.to_string())?;
        assert_eq!(*result_guard, *expected_guard);

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
