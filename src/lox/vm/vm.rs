use crate::lox::vm::{chunk, disassembler, error, opcodes, value};

const DEFAULT_STACK_SIZE: usize = 256;

pub trait VirtualMachine {
    fn run(&mut self, chunk: &chunk::Chunk) -> Result<(), error::RuntimeError>;
}

pub struct VmState {
    /// The position to the next byte in the chunk to be executed.
    pub instruction_pointer: usize,

    /// The machine's stack
    stack: Vec<value::Value>,
    max_stack_size: usize,

    /// If true, the VM will print the disassembled instructions as they are executed.
    pub tracing: bool,
}

pub struct VirtualMachineImpl {
    pub state: VmState,
}

impl VirtualMachineImpl {
    pub fn new() -> Self {
        VirtualMachineImpl {
            state: VmState {
                instruction_pointer: 0,
                stack: Vec::new(),
                max_stack_size: DEFAULT_STACK_SIZE,
                tracing: false, // Default to not tracing
            },
        }
    }

    fn stack_push(&mut self, value: value::Value) -> Result<(), error::RuntimeError> {
        if self.state.stack.len() >= self.state.max_stack_size {
            return Err(error::RuntimeError::StackOverflow(
                self.state.max_stack_size,
            ));
        }

        self.state.stack.push(value);
        Ok(())
    }

    fn stack_pop(&mut self) -> Result<value::Value, error::RuntimeError> {
        self.state
            .stack
            .pop()
            .ok_or(error::RuntimeError::StackUnderflow)
    }
}

impl VirtualMachine for VirtualMachineImpl {
    fn run(&mut self, chunk: &chunk::Chunk) -> Result<(), error::RuntimeError> {
        // Reset the instruction pointer to the start of the chunk
        self.state.instruction_pointer = 0;

        // Loop through the chunk's bytecode instructions
        loop {
            ///////////////////////////////////////////////////////////////////
            // Tracing
            //
            // If enabled, print the disassembled instruction
            // at the current instruction pointer, before executing it.
            if self.state.tracing {
                // TODO: Print the stack contents as well

                let stack_content = self
                    .state
                    .stack
                    .iter()
                    .enumerate()
                    .map(|(i, v)| format!("  {i:<3}: {v:?}"))
                    .collect::<Vec<_>>()
                    .join("\n");
                println!("\nSTACK: {}\n{stack_content}", self.state.stack.len());

                let (tracing, _) =
                    disassembler::dissasemble_instruction(chunk, self.state.instruction_pointer)?;
                print!("{tracing}");
            }

            // first retrieve the instruction code from the chunk as a u8, checking if the instruction pointer
            // is within bounds.
            let byte = chunk.code.get(self.state.instruction_pointer).ok_or(
                error::RuntimeError::InstructionPointerOutOfBounds(
                    self.state.instruction_pointer,
                    chunk.code.len(),
                ),
            )?;

            // Then convert it to an OpCode enum variant, checking for invalid instructions codes.
            let (op_code, next_instruction_offset) = opcodes::try_from_with_offset(&byte)?;

            match op_code {
                opcodes::OpCode::Return => {
                    // Handle return operation
                    self.state.instruction_pointer += next_instruction_offset; // Move to the next instruction

                    let return_value = self.stack_pop()?;
                    println!("return: {:?}", return_value);

                    break; // Exit the loop
                }
                opcodes::OpCode::Constant => {
                    // Handle constant operation
                    let constant_index = chunk.get_byte(self.state.instruction_pointer + 1)?;

                    // Retrieve the constant from the chunk's constant array, and push it onto the stack.
                    let constant = chunk.get_constant(constant_index as usize)?;

                    // As the constant is a referent to the chunk's constant array, it needs to be cloned
                    // to be pushed onto the stack.
                    self.stack_push(constant.clone())?;

                    // TOTHINK: Should moving to the next instruction be always done at the end of the match arm?
                    // finally, move to the next instruction. If so, I could have some form of RAII struct that
                    // automatically moves the instruction pointer forward when it goes out of scope.
                    self.state.instruction_pointer += next_instruction_offset;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::lox::vm::chunk;
    use crate::lox::vm::value;

    #[test]
    fn test_run_valid_chunk() {
        let mut vm = VirtualMachineImpl::new();
        let chunk = chunk::Chunk {
            code: vec![0x01], // OpCode::Return
            constants: vec![],
        };

        assert!(vm.run(&chunk).is_ok());
    }

    #[test]
    fn test_run_invalid_instruction() {
        let mut vm = VirtualMachineImpl::new();
        let chunk = chunk::Chunk {
            code: vec![0xFF], // Invalid OpCode
            constants: vec![],
        };

        let result = vm.run(&chunk);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            error::RuntimeError::InvalidInstruction(0xFF)
        );
    }

    #[test]
    fn test_chunk_with_constants() {
        let mut vm = VirtualMachineImpl::new();
        vm.state.tracing = true; // Enable tracing for this test
        let chunk = chunk::Chunk {
            code: vec![0x00, 0x00, 0x00, 0x01, 0x01],
            constants: vec![value::Value::Number(42.0), value::Value::Number(3.14)],
        };

        assert!(vm.run(&chunk).is_ok());
    }
}
