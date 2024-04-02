use std::{fmt::Display, rc::Rc};

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

pub trait Callable: std::fmt::Display + std::fmt::Debug {
    fn get_arg_name(&self, arg_number: usize) -> Result<String, String>;
    fn get_arg_count(&self) -> usize;
    fn call(&self) -> Result<Value, String>;
    fn get_body(&self) -> &Box<Stmt>;
}

impl PartialEq for dyn Callable {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
