use super::Value;

pub trait Environment {
    fn get(&self, name: &str) -> Option<&Value>;
    fn set(&mut self, name: &str, value: Value) -> Result<&Value, String>;
    fn define(&mut self, name: &str, value: Value);

    // TODO: get parent environment
}

pub struct EnvironmentImpl {
    values: std::collections::HashMap<String, Value>,
}

impl EnvironmentImpl {
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
        }
    }
}

impl Environment for EnvironmentImpl {
    fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }

    fn set(&mut self, name: &str, value: Value) -> Result<&Value, String> {
        match self.values.get_mut(name) {
            Some(v) => {
                *v = value;
                Ok(v)
            }
            None => Err(format!("Undefined variable '{}'", name)),
        }
    }

    fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value);
    }
}
