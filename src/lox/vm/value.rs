use crate::lox::vm::error;

/// A value in the virtual machine.
/// By using an enum, it's simple to define the different primitive types
/// supported by the language.
///
/// A Value cannot implement the `Copy` trait, as it can contain
/// heap-allocated data, such as strings, functions, or objects.
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Nil,
}

///////////////////////////////////////////////////////////////////////////////
// Arythmetic operations between Values
//
// This is the single place in the codebase where the arythmetic operations
// between Values are defined. This helps at decloutering the VM's run loop,
// and also makes it easier to add new operations in the future.
///////////////////////////////////////////////////////////////////////////////

impl std::ops::Neg for Value {
    type Output = Result<Value, error::RuntimeError>;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(error::RuntimeError::RuntimeError(
                "Attempted to negate a non-number value".to_string(),
            )),
        }
    }
}

impl std::ops::Add for Value {
    type Output = Result<Value, error::RuntimeError>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            _ => Err(error::RuntimeError::RuntimeError(
                "Attempted to add non-number values".to_string(),
            )),
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Result<Value, error::RuntimeError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            _ => Err(error::RuntimeError::RuntimeError(
                "Attempted to subtract non-number values".to_string(),
            )),
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Result<Value, error::RuntimeError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            _ => Err(error::RuntimeError::RuntimeError(
                "Attempted to multiply non-number values".to_string(),
            )),
        }
    }
}

impl std::ops::Div for Value {
    type Output = Result<Value, error::RuntimeError>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => {
                // TODO: Need to test!
                if b == 0.0 || b.is_nan() {
                    return Err(error::RuntimeError::RuntimeError(
                        "Division by zero".to_string(),
                    ));
                }
                Ok(Value::Number(a / b))
            }
            _ => Err(error::RuntimeError::RuntimeError(
                "Attempted to divide non-number values".to_string(),
            )),
        }
    }
}
