use crate::lox::vm::error;

/// Operation codes.
///
/// Each operation is a single byte. Depending on the operation, it may require additional data
/// from the chunk code. This is indicated in the documentation of each operation using
///
/// [CODE, data:type, ...]
#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    /// [CONSTANT, index:u8]
    /// Loads a constant value from the chunk's constant array onto the stack.
    /// The `index` is the index of the constant in the chunk's constant array.
    Constant = 0x00,

    /// [RETURN]
    Return = 0x01,

    /// [NEGATE]
    /// Pops the top value on the stack, and negates it if it is a numerical value.
    /// The result is pushed back onto the stack.
    Negate = 0x02,

    /// [ADD]
    Add = 0x03,

    /// [SUBTRACT]
    Subtract = 0x04,

    /// [MULTIPLY]
    Multiply = 0x05,

    /// [DIVIDE]
    Divide = 0x06,
}

impl TryFrom<&u8> for OpCode {
    type Error = error::RuntimeError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(OpCode::Constant),
            0x01 => Ok(OpCode::Return),
            0x02 => Ok(OpCode::Negate),
            0x03 => Ok(OpCode::Add),
            0x04 => Ok(OpCode::Subtract),
            0x05 => Ok(OpCode::Multiply),
            0x06 => Ok(OpCode::Divide),
            _ => Err(error::RuntimeError::InvalidInstruction(*value)),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(op_code: OpCode) -> Self {
        op_code as u8
    }
}

pub fn try_from_with_offset(value: &u8) -> Result<(OpCode, usize), error::RuntimeError> {
    let op_code = OpCode::try_from(value)?;

    let op_with_offset = match op_code {
        // Single byte instructions, thus the next instruction offset is 1
        OpCode::Return
        | OpCode::Negate
        | OpCode::Add
        | OpCode::Subtract
        | OpCode::Multiply
        | OpCode::Divide => (op_code, 1),
        OpCode::Constant => {
            // The next byte after the instruction code is the index of the constant in the
            // constant array in the chunk, hence the next instruction offset is 2
            (op_code, 2)
        }
    };

    Ok(op_with_offset)
}
