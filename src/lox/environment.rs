use std::{cell::RefCell, rc::Rc};

use super::{Callable, Value};

// TODO: need to sort out the memory layout of the variables stored in the environment
//       till now, I clone the stored values everytime I access them, which is inneficient
pub trait Environment: std::fmt::Display + std::fmt::Debug {
    fn get_variable(&self, name: &str) -> Option<Value>;
    fn set_variable(&mut self, name: &str, value: Value) -> Result<Value, String>;
    fn define_variable(&mut self, name: &str, value: Value);

    fn push_variable_stack(&mut self);
    fn pop_variable_stack(&mut self);

    fn branch_push(&mut self);
    fn branch_pop(&mut self);

    fn define_function(&mut self, name: &str, value: Box<dyn Callable>);
}

type ValueStack = Vec<std::collections::HashMap<String, Value>>;

#[derive(Debug)]
pub struct EnvironmentImpl {
    global_variables: Rc<RefCell<std::collections::HashMap<String, Value>>>,
    // current_stack: ValueStack,

    // a stack of environments, used across function calls
    branch_stack: Vec<ValueStack>,
}

impl EnvironmentImpl {
    pub fn new() -> Self {
        // TODO create an empty
        // let branch_stack = vec![vec![std::collections::HashMap::new()]];
        let branch_stack = vec![vec![]];

        Self {
            global_variables: Rc::new(RefCell::new(std::collections::HashMap::new())),
            branch_stack: branch_stack,
        }
    }
}

impl Environment for EnvironmentImpl {
    fn get_variable(&self, name: &str) -> Option<Value> {
        // search in the current stack, if there is any created
        if let Some(current_stack) = self.branch_stack.last() {
            for scope in current_stack.iter().rev() {
                if let Some(v) = scope.get(name) {
                    return Some(v.clone());
                }
            }
        }

        self.global_variables.borrow().get(name).cloned()
    }

    fn set_variable(&mut self, name: &str, value: Value) -> Result<Value, String> {
        // if there is a branch stack, try to set the variable value there
        if let Some(current_stack) = self.branch_stack.last_mut() {
            for scope in current_stack.iter_mut().rev() {
                if let Some(v) = scope.get_mut(name) {
                    *v = value;
                    return Ok(v.clone());
                }
            }
        }

        // if the variable is not found in the current stack, try to set it in the global variables
        if let Some(v) = self.global_variables.borrow_mut().get_mut(name) {
            *v = value;
            // return the value
            return Ok(v.clone());
        }

        Err(format!("Undefined variable '{}'", name))
    }

    fn define_variable(&mut self, name: &str, value: Value) {
        if let Some(current_stack) = self.branch_stack.last_mut() {
            if let Some(scope) = current_stack.last_mut() {
                scope.insert(name.to_string(), value);
                return;
            }
        }

        self.global_variables
            .borrow_mut()
            .insert(name.to_string(), value);

        // let current_stack = self.branch_stack.last_mut().unwrap();
        // match current_stack.last_mut() {
        //     Some(scope) => {
        //         scope.insert(name.to_string(), value);
        //     }
        //     None => {
        //         self.global_variables
        //             .borrow_mut()
        //             .insert(name.to_string(), value);
        //     }
        // }
    }

    fn push_variable_stack(&mut self) {
        let current_stack = self.branch_stack.last_mut().unwrap();
        current_stack.push(std::collections::HashMap::new());
    }

    fn pop_variable_stack(&mut self) {
        let current_stack = self.branch_stack.last_mut().unwrap();
        if current_stack.len() > 1 {
            current_stack.pop();
        }
    }

    fn branch_push(&mut self) {
        self.branch_stack
            .push(vec![std::collections::HashMap::new()]);
    }

    fn branch_pop(&mut self) {
        if self.branch_stack.len() > 1 {
            self.branch_stack.pop();
        }
    }

    fn define_function(&mut self, name: &str, value: Box<dyn Callable>) {
        self.global_variables
            .borrow_mut()
            .insert(name.to_string(), Value::Callable(Rc::new(value)));
    }
}

impl std::fmt::Display for EnvironmentImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "EnvironmentImpl")
        write!(f, "EnvironmentImpl")
    }
}
