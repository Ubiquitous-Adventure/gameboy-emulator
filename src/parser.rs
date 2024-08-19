use std::io::{Bytes, Read};

use gameboy_emulator::bits;
use itertools::Itertools;

use crate::{
    errors::EmulatorError,
    instructions::{Instruction, R16MemOperand, R16Operand},
};

fn get_prefix_instruction(bytes: &mut Bytes<impl Read>) -> Result<u8, EmulatorError> {
    let instruction_byte = bytes.next().expect("instruction after prefix")?;
    Ok(instruction_byte)
}

fn get_8bit_immediate(bytes: &mut Bytes<impl Read>) -> Result<u8, EmulatorError> {
    let immediate = bytes.next().expect("8-bit immediate")?;
    Ok(immediate)
}

fn get_16bit_immediate(bytes: &mut Bytes<impl Read>) -> Result<u16, EmulatorError> {
    let (imm_byte1, imm_byte2) = bytes.next_tuple().expect("16-bit immediate");
    let immediate: u16 = ((imm_byte1? as u16) << 8) + (imm_byte2? as u16);
    Ok(immediate)
}

pub fn parse_instructions(
    mut bytes: Bytes<impl Read>,
    size: usize,
    debug: bool,
) -> Result<Vec<Instruction>, EmulatorError> {
    let mut instructions: Vec<Instruction> = Vec::with_capacity(size);

    while let Some(byte_result) = bytes.next() {
        let byte = byte_result?;

        if debug {
            println!("Byte: '{byte:0>#8b}' ('{byte:0>#2x}')");
        }

        let instruction = match byte {
            bits!(00000000) => Instruction::Nop,
            bits!(00__0001) => {
                let operand = (byte >> 4) & 0b11;
                Instruction::LoadImm16 {
                    reg: R16Operand::from(operand),
                    imm: get_16bit_immediate(&mut bytes)?,
                }
            }
            bits!(00__0010) => {
                let operand = (byte >> 4) & 0b11;
                Instruction::StoreARegToMem {
                    mem: R16MemOperand::from(operand),
                }
            }
            bits!(00__1010) => {
                let operand = (byte >> 4) & 0b11;
                Instruction::LoadMemToAReg {
                    mem: R16MemOperand::from(operand),
                }
            }
            bits!(00001000) => Instruction::StoreSPToImmMem {
                imm: get_16bit_immediate(&mut bytes)?,
            },
            bits!(00__0011) => {
                let operand = (byte >> 4) & 0b11;
                Instruction::IncR16 {
                    reg: R16Operand::from(operand),
                }
            }
            bits!(00__1011) => {
                let operand = (byte >> 4) & 0b11;
                Instruction::DecR16 {
                    reg: R16Operand::from(operand),
                }
            }
            bits!(00__1001) => {
                let operand = (byte >> 4) & 0b11;
                Instruction::AddRegToHLReg {
                    reg: R16Operand::from(operand),
                }
            }
            _ => todo!("Instruction: '{byte:0>#8b}' ('{byte:0>#2x}')"),
        };

        instructions.push(instruction);
    }
    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn basic_input() {
        let null_byte = "\x00";
        let cursor = Cursor::new(null_byte);
        let instructions = parse_instructions(cursor.bytes(), null_byte.len(), false);
        assert!(instructions.is_ok());
        let instructions = instructions.unwrap();
        assert_eq!(instructions, vec![Instruction::Nop]);
    }
}
