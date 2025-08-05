use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum RuntimeError {
    #[error("Instruction pointer out of bounds: {0}, size: {1}")]
    InstructionPointerOutOfBounds(usize, usize),

    #[error("Invalid instruction code {0:0x}")]
    InvalidInstruction(u8),

    #[error("Invalid constant index {0}")]
    InvalidConstantIndex(u8),

    #[error("Attempted to pop from an empty stack")]
    StackUnderflow,

    #[error("Stack overflow: attempted to push to a full stack of size {0}")]
    StackOverflow(usize),
}
