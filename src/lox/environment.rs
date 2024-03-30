use super::Value;

pub trait Environment {
    fn get(&self, name: &str) -> Option<&Value>;
    fn set(&mut self, name: &str, value: Value) -> Result<&Value, String>;
    fn define(&mut self, name: &str, value: Value);

    fn push(&mut self);
    fn pop(&mut self);
}

pub struct EnvironmentImpl {
    value_stack: Vec<std::collections::HashMap<String, Value>>,
}

impl EnvironmentImpl {
    pub fn new() -> Self {
        Self {
            value_stack: vec![std::collections::HashMap::new()],
        }
    }
}

impl Environment for EnvironmentImpl {
    fn get(&self, name: &str) -> Option<&Value> {
        for scope in self.value_stack.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v);
            }
        }

        None
    }

    fn set(&mut self, name: &str, value: Value) -> Result<&Value, String> {
        for scope in self.value_stack.iter_mut().rev() {
            if let Some(v) = scope.get_mut(name) {
                *v = value;
                return Ok(v);
            }
        }

        Err(format!("Undefined variable '{}'", name))
    }

    fn define(&mut self, name: &str, value: Value) {
        self.value_stack
            .last_mut()
            .unwrap()
            .insert(name.to_string(), value);
    }

    fn push(&mut self) {
        self.value_stack.push(std::collections::HashMap::new());
    }

    fn pop(&mut self) {
        if self.value_stack.len() > 1 {
            self.value_stack.pop();
        }
    }
}
