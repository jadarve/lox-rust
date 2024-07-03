use std::{fmt::Display, rc::Rc, sync::Arc, sync::RwLock};

use super::Stmt;

// Possible value types allowed in Lox
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Callable(Rc<Box<dyn Callable>>),
    Nil,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Nil => false,
            Value::Callable(_) => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Callable(c) => write!(f, "<callable> {}", c.to_string()),
        }
    }
}

// Type used to store a Value in a interpreter session.
pub type ValueBox = Arc<RwLock<Box<Value>>>;

pub fn new_value_box(value: Value) -> ValueBox {
    Arc::new(RwLock::new(Box::new(value)))
}

pub trait Callable: std::fmt::Display + std::fmt::Debug {
    fn get_arg_name(&self, arg_number: usize) -> Result<String, String>;
    fn get_arg_count(&self) -> usize;
    fn call(&self) -> Result<ValueBox, String>;
    fn get_body(&self) -> &Box<Stmt>;
}

impl PartialEq for dyn Callable {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[cfg(test)]
mod tests {

    use std::borrow::BorrowMut;

    use super::Value;

    #[test]
    fn test_value_truthiness() {
        let value = Value::Number(0.0);
        assert_eq!(value.is_truthy(), false);

        let value = Value::Number(1.0);
        assert_eq!(value.is_truthy(), true);

        let value = Value::String("".to_string());
        assert_eq!(value.is_truthy(), false);

        let value = Value::String("Hello".to_string());
        assert_eq!(value.is_truthy(), true);

        let value = Value::Boolean(false);
        assert_eq!(value.is_truthy(), false);

        let value = Value::Boolean(true);
        assert_eq!(value.is_truthy(), true);

        let value = Value::Nil;
        assert_eq!(value.is_truthy(), false);
    }
}
