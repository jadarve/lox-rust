use std::fmt::Display;

use super::{Callable, EnvironmentImpl, Stmt, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionImpl {
    name: String,
    arguments: Vec<String>,
    body: Box<Stmt>,
}

impl FunctionImpl {
    pub fn new(name: String, arguments: Vec<String>, body: Box<Stmt>) -> Self {
        Self {
            name,
            arguments,
            body,
        }
    }
}

impl Callable for FunctionImpl {
    fn call(&self) -> Result<Value, String> {
        println!("FunctionImpl::call(): {}", self.name);

        // let mut environment = EnvironmentImpl::new();
        // environment.push_variable_stack();

        // for (name, value) in self.arguments.iter().zip(arguments.iter()) {
        //     environment.define_variable(name, value.clone());
        // }

        // let result = self.body.accept(&mut Interpreter::new(&mut environment));

        // environment.pop_variable_stack();

        // result

        Ok(Value::Nil)
    }

    fn get_arg_name(&self, arg_number: usize) -> Result<String, String> {
        if arg_number >= self.arguments.len() {
            return Err(format!(
                "Function '{}' has {} arguments, requested argument {}",
                self.name,
                self.arguments.len(),
                arg_number
            ));
        }

        Ok(self.arguments[arg_number].clone())
    }

    fn get_arg_count(&self) -> usize {
        self.arguments.len()
    }

    fn get_body(&self) -> &Box<Stmt> {
        &self.body
    }
}

impl Display for FunctionImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.name)
    }
}
