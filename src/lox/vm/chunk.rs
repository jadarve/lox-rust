use crate::lox::vm::error;
use crate::lox::vm::value;

pub struct Chunk {
    /// The bytecode instructions is a contiguous vector of bytes interpreted by the virtual machine.
    /// Each operation code is extracted from the raw byte data, checking corrupted data.
    pub code: Vec<u8>,

    /// The constants defined for the chunk.
    pub constants: Vec<value::Value>,
}

impl Chunk {
    /// Would be nice to see the different in performance between inlining this method or not.
    #[inline(always)]
    pub fn get_byte(&self, index: usize) -> Result<u8, error::RuntimeError> {
        self.code
            .get(index)
            .copied()
            .ok_or(error::RuntimeError::InstructionPointerOutOfBounds(
                index,
                self.code.len(),
            ))
    }

    #[inline(always)]
    pub fn get_constant(&self, index: usize) -> Result<&value::Value, error::RuntimeError> {
        self.constants
            .get(index)
            .ok_or(error::RuntimeError::InvalidConstantIndex(index as u8))
    }
}
