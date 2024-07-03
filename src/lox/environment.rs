use std::rc::Rc;

use super::{new_value_box, Callable, Value, ValueBox};

type ValueStack = Vec<std::collections::HashMap<String, ValueBox>>;

// TODO: need to sort out the memory layout of the variables stored in the environment
//       till now, I clone the stored values everytime I access them, which is inneficient
pub trait Environment: std::fmt::Display + std::fmt::Debug {
    fn get_variable(&self, name: &str) -> Option<ValueBox>;
    fn set_variable(&mut self, name: &str, value: Value) -> Result<ValueBox, String>;
    fn define_variable(&mut self, name: &str, value: Value);

    fn push_variable_stack(&mut self);
    fn pop_variable_stack(&mut self);

    fn branch_push(&mut self);
    fn branch_pop(&mut self);

    fn define_function(&mut self, name: &str, value: Box<dyn Callable>);
}

#[derive(Debug)]
pub struct EnvironmentImpl {
    global_variables: std::collections::HashMap<String, ValueBox>,
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
            global_variables: std::collections::HashMap::new(),
            branch_stack: branch_stack,
        }
    }
}

impl Environment for EnvironmentImpl {
    fn get_variable(&self, name: &str) -> Option<ValueBox> {
        // search in the current stack, if there is any created
        if let Some(current_stack) = self.branch_stack.last() {
            for scope in current_stack.iter().rev() {
                if let Some(v) = scope.get(name) {
                    return Some(v.to_owned());
                }
            }
        }

        self.global_variables.get(name).map(|v| v.to_owned())
    }

    fn set_variable(&mut self, name: &str, value: Value) -> Result<ValueBox, String> {
        // if there is a branch stack, try to set the variable value there
        if let Some(current_stack) = self.branch_stack.last_mut() {
            for scope in current_stack.iter_mut().rev() {
                if let Some(v) = scope.get_mut(name) {
                    let mut guard = v.try_write().map_err(|e| {
                        format!("Error locking variable \"{name}\" for writing: {e:?}")
                    })?;
                    *guard.as_mut() = value;
                    return Ok(v.to_owned());
                }
            }
        }

        // if the variable is not found in the current stack, try to set it in the global variables
        if let Some(v) = self.global_variables.get_mut(name) {
            let mut guard = v.try_write().map_err(|e| {
                format!("Error locking global variable \"{name}\" for writing: {e:?}")
            })?;
            *guard.as_mut() = value;
            return Ok(v.to_owned());
        }

        Err(format!("Undefined variable '{}'", name))
    }

    fn define_variable(&mut self, name: &str, value: Value) {
        if let Some(current_stack) = self.branch_stack.last_mut() {
            if let Some(scope) = current_stack.last_mut() {
                scope.insert(name.to_string(), new_value_box(value));
                return;
            }
        }

        self.global_variables
            .insert(name.to_string(), new_value_box(value));

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
        self.global_variables.insert(
            name.to_string(),
            new_value_box(Value::Callable(Rc::new(value))),
        );
    }
}

impl std::fmt::Display for EnvironmentImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "EnvironmentImpl")
        write!(f, "EnvironmentImpl")
    }
}

#[cfg(test)]
mod tests {

    use crate::lox::{Value, ValueBox};

    use super::Environment;

    trait ManipulateVariable {
        fn get_variable(&self, name: &str) -> Result<ValueBox, String>;
        fn define_variable(&mut self, name: &str, value: Value);
    }

    struct EnvironmentHolder {
        environment: Box<dyn Environment>,
    }

    impl EnvironmentHolder {
        fn new(environment: Box<dyn Environment>) -> Self {
            Self { environment }
        }
    }

    impl ManipulateVariable for EnvironmentHolder {
        fn get_variable(&self, name: &str) -> Result<ValueBox, String> {
            self.environment
                .get_variable(name)
                .ok_or(format!("Variable not found: {name}"))
        }

        fn define_variable(&mut self, name: &str, value: Value) {
            self.environment.define_variable(name, value);
        }
    }

    #[test]
    fn test_value_allocation() -> Result<(), String> {
        let mut env = super::EnvironmentImpl::new();

        // create a variable
        env.define_variable("a", super::Value::Number(1.0));

        let a1 = env.get_variable("a").ok_or("Variable 'a' not found")?;
        let mut addr_a1: usize = 0;
        if let Ok(mut guard) = a1.write() {
            *guard.as_mut() = super::Value::Number(2.0);
            addr_a1 = &*guard.as_ref() as *const Value as usize;
        }
        // let addr_a1 = &*a1 as *const Value as usize;
        // *a1.as_mut() = super::Value::Number(2.0);

        let a2 = env.get_variable("a").ok_or("Variable 'a' not found")?;
        let mut addr_a2: usize = 0;
        if let Ok(mut guard) = a2.write() {
            *guard.as_mut() = super::Value::Number(3.0);
            addr_a2 = &*guard.as_ref() as *const Value as usize;
        }

        // lock a1 again and check the value
        if let Ok(guard) = a1.read() {
            assert_eq!(*guard.as_ref(), super::Value::Number(3.0));
        }
        // assert_eq!(*a1, super::Value::Number(3.0));
        assert_eq!(addr_a1, addr_a2);
        Ok(())
    }

    #[test]
    fn test_concurrent_access() -> Result<(), String> {
        let mut env_holder = EnvironmentHolder::new(Box::new(super::EnvironmentImpl::new()));

        env_holder.define_variable("a", Value::Number(1.0));

        let a1 = env_holder.get_variable("a")?;
        if let Ok(a_guard) = a1.read() {
            assert_eq!(*a_guard.as_ref(), Value::Number(1.0));
        }

        let a2 = env_holder.get_variable("a")?;

        let partial = if let Ok(a1_guard) = a1.try_read() {
            match a1_guard.as_ref() {
                Value::Number(ref a1_value) => *a1_value + 1.0,
                _ => 0.0,
            }
        } else {
            0.0
        };

        // then write the value
        if let Ok(mut a2_guard) = a2.try_write() {
            *a2_guard.as_mut() = Value::Number(partial);
        }

        if let Ok(a_guard) = a1.read() {
            assert_eq!(*a_guard.as_ref(), Value::Number(2.0));
        }

        Ok(())
    }
}
