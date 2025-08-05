use crate::lox::vm::error;

#[repr(u8)]
pub enum OpCode {
    /// [CONSTANT, index:u8]
    Constant = 0x00,

    /// [RETURN]
    Return = 0x01,
}

impl TryFrom<&u8> for OpCode {
    type Error = error::RuntimeError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(OpCode::Constant),
            0x01 => Ok(OpCode::Return),
            _ => Err(error::RuntimeError::InvalidInstruction(*value)),
        }
    }
}

pub fn try_from_with_offset(value: &u8) -> Result<(OpCode, usize), error::RuntimeError> {
    let op_code = OpCode::try_from(value)?;

    let op_with_offset = match op_code {
        OpCode::Return => {
            // Return is a single-byte instruction, hence the next instruction offset is 1
            (op_code, 1)
        }
        OpCode::Constant => {
            // The next byte after the instruction code is the index of the constant in the
            // constant array in the chunk, hence the next instruction offset is 2
            (op_code, 2)
        }
    };

    Ok(op_with_offset)
}
