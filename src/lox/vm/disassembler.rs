use crate::lox::vm::{chunk, error, opcodes};

const INSTRUCTION_PADDING: usize = 10;

pub fn disassemble_chunk(chunk: &chunk::Chunk) -> Result<String, error::RuntimeError> {
    let mut output = String::new();

    let mut instruction_pointer: usize = 0;

    while instruction_pointer < chunk.code.len() {
        let (dissasembled_instruction, next_instruction_offset) =
            dissasemble_instruction(chunk, instruction_pointer)?;

        output.push_str(&dissasembled_instruction);
        instruction_pointer += next_instruction_offset;
    }

    Ok(output)
}

/// Dissasemble a single instruction at the given instruction pointer
pub fn dissasemble_instruction(
    chunk: &chunk::Chunk,
    instruction_pointer: usize,
) -> Result<(String, usize), error::RuntimeError> {
    let byte = chunk.code.get(instruction_pointer).ok_or(
        error::RuntimeError::InstructionPointerOutOfBounds(instruction_pointer, chunk.code.len()),
    )?;
    let (op_code, next_instruction_offset) = opcodes::try_from_with_offset(&byte)?;

    let mut output = format!("{instruction_pointer:04} ");

    match op_code {
        opcodes::OpCode::Return => {
            output
                .push_str(format!("{:<width$}\n", "RETURN", width = INSTRUCTION_PADDING).as_str());
        }
        opcodes::OpCode::Constant => {
            let constant_index = chunk.get_byte(instruction_pointer + 1)?;
            let value = chunk.get_constant(constant_index as usize)?;

            output.push_str(
                format!(
                    "{:<width$} {constant_index:03} : {value:?}\n",
                    "CONSTANT",
                    width = INSTRUCTION_PADDING
                )
                .as_str(),
            );
        }
    }

    Ok((output, next_instruction_offset))
}

#[cfg(test)]
mod tests {

    use crate::lox::vm::chunk::Chunk;
    use crate::lox::vm::value;

    use super::*;

    use anyhow::Result;

    #[test]
    fn test_disassemble_chunk() -> Result<()> {
        let chunk = Chunk {
            code: vec![0x00, 0x00, 0x00, 0x01, 0x01],
            constants: vec![value::Value::Number(42.0), value::Value::Number(3.14)],
        };

        let disassembled = disassemble_chunk(&chunk)?;
        println!("{disassembled}");
        Ok(())
    }
}
