use std::io::{self, Bytes, Read};

use gameboy_emulator::bits;
use itertools::Itertools;

use crate::{
    errors::EmulatorError,
    instructions::{CondOperand, Instruction, R16MemOperand, R16Operand, R8Operand},
};

fn get_prefix_instruction(bytes: &mut Bytes<impl Read>) -> Result<u8, io::Error> {
    let instruction_byte = bytes.next().expect("instruction after prefix")?;
    Ok(instruction_byte)
}

fn get_8bit_immediate(bytes: &mut Bytes<impl Read>) -> Result<u8, io::Error> {
    let immediate = bytes.next().expect("8-bit immediate")?;
    Ok(immediate)
}

fn get_16bit_immediate(bytes: &mut Bytes<impl Read>) -> Result<u16, io::Error> {
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
            bits!(00___100) => {
                let operand = (byte >> 3) & 0b111;
                Instruction::IncR8 {
                    reg: R8Operand::from(operand),
                }
            }
            bits!(00___101) => {
                let operand = (byte >> 3) & 0b111;
                Instruction::DecR8 {
                    reg: R8Operand::from(operand),
                }
            }
            bits!(00___110) => {
                let operand = (byte >> 3) & 0b111;
                Instruction::LoadImm8 {
                    dst: R8Operand::from(operand),
                    imm: get_8bit_immediate(&mut bytes)?,
                }
            }
            bits!(00000111) => Instruction::RotARegLeftSetC,
            bits!(00001111) => Instruction::RotARegRightSetC,
            bits!(00010111) => Instruction::RotARegLeftThroughC,
            bits!(00011111) => Instruction::RotARegRightThroughC,
            bits!(00100111) => Instruction::DecAdjAccum,
            bits!(00101111) => Instruction::InvA,
            bits!(00110111) => Instruction::SetC,
            bits!(00111111) => Instruction::InvC,
            bits!(00011000) => {
                let immediate = get_8bit_immediate(&mut bytes)?;
                Instruction::JumpRelativeImm {
                    imm: i8::from_le_bytes(immediate.to_le_bytes()),
                }
            }
            bits!(001__000) => {
                let operand = (byte >> 3) & 0b11;
                let immediate = get_8bit_immediate(&mut bytes)?;
                Instruction::JumpRelativeImmUnderCond {
                    cond: CondOperand::from(operand),
                    imm: i8::from_le_bytes(immediate.to_le_bytes()),
                }
            }
            bits!(00010000) => Instruction::Stop,
            bits!(01______) if byte != 0b01110110 => {
                let dst_operand = (byte >> 3) & 0b111;
                let src_operand = byte & 0b111;
                Instruction::LoadR8ToR8 {
                    dst: R8Operand::from(dst_operand),
                    src: R8Operand::from(src_operand),
                }
            }
            bits!(01110110) => Instruction::Halt,
            bits!(10000___) => Instruction::AddRegToAReg {
                reg: R8Operand::from(byte & 0b111),
            },
            bits!(10001___) => Instruction::AddRegCToAReg {
                reg: R8Operand::from(byte & 0b111),
            },
            bits!(10010___) => Instruction::SubRegFromAReg {
                reg: R8Operand::from(byte & 0b111),
            },
            bits!(10011___) => Instruction::SubRegCFromAReg {
                reg: R8Operand::from(byte & 0b111),
            },
            bits!(10100___) => Instruction::AndRegToAReg {
                reg: R8Operand::from(byte & 0b111),
            },
            bits!(10101___) => Instruction::XorRegToAReg {
                reg: R8Operand::from(byte & 0b111),
            },
            bits!(10110___) => Instruction::OrRegToAReg {
                reg: R8Operand::from(byte & 0b111),
            },
            bits!(10111___) => Instruction::CmpRegToAReg {
                reg: R8Operand::from(byte & 0b111),
            },
            bits!(11000___) => Instruction::AddImmToAReg {
                imm: get_8bit_immediate(&mut bytes)?,
            },
            bits!(11001___) => Instruction::AddImmCToAReg {
                imm: get_8bit_immediate(&mut bytes)?,
            },
            bits!(11010___) => Instruction::SubImmFromAReg {
                imm: get_8bit_immediate(&mut bytes)?,
            },
            bits!(11011___) => Instruction::SubImmCFromAReg {
                imm: get_8bit_immediate(&mut bytes)?,
            },
            bits!(11100___) => Instruction::AndImmToAReg {
                imm: get_8bit_immediate(&mut bytes)?,
            },
            bits!(11101___) => Instruction::XorImmToAReg {
                imm: get_8bit_immediate(&mut bytes)?,
            },
            bits!(11110___) => Instruction::OrImmToAReg {
                imm: get_8bit_immediate(&mut bytes)?,
            },
            bits!(11111___) => Instruction::CmpImmToAReg {
                imm: get_8bit_immediate(&mut bytes)?,
            },

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
