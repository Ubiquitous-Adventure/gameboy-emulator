use crate::instructions::Instruction;
extern crate regex;
use regex::Regex;
use std::{
    fs::File,
    io::{BufReader, Read, Result},
};

pub fn read_file_to_bytes(path: &str) -> Result<Vec<u8>> {
    // open file and transform into bufreader
    let mut reader = BufReader::new(File::open(path)?);
    let mut bytes: Vec<u8> = Vec::new();

    reader.read_to_end(&mut bytes)?;
    Ok(bytes)
}

pub fn parse_bytes_to_intr(mut bytes: Vec<u8>) -> Result<Vec<Instruction>> {
    let mut enumerated_bytes = bytes.iter().enumerate();
    let mut instr_list: Vec<Instruction> = Vec::new();
    while let Some((byte_num, byte_value)) = enumerated_bytes.next() {
        let byte = *byte_value;
        // there has to be a better way maybe preprocess them and throw them in a list or something
        let ld16im_p = Regex::new(r"^00[01][01]0001$").unwrap();
        let ld16a_p = Regex::new(r"^00[01][01]0010$").unwrap();
        let lda16_p = Regex::new(r"^00[01][01]1010$").unwrap();
        let inc16_p = Regex::new(r"^00[01][01]0011$").unwrap();
        let dec16_p = Regex::new(r"^00[01][01]1011$").unwrap();
        let addhl16_p = Regex::new(r"^00[01][01]1001$").unwrap();
        match byte {
            // nop
            0b00000000 => instr_list.push(Instruction::Nop),
            // ld r16 n16
            n if ld16im_p.is_match(&format!("{:08b}", n)) => {
                // fetch the register
                let register16: u8 = byte >> 4;
                // this is unsafe for now, use and if or unwrap_or idk whatever (get a method?)
                // take the next two bytes two build the imm16
                let (_, byte1) = enumerated_bytes.next().unwrap();
                let (_, byte2) = enumerated_bytes.next().unwrap();
                // careful with little endian imm16
                let immidiate16: u16 = (*byte2 as u16) << 8 | (*byte1 as u16);
                // push actual instruction with correct arguments
                instr_list.push(Instruction::Ld16im {
                    reg: (register16),
                    imm: (immidiate16),
                });
                // push the placeholder bytes
                instr_list.push(Instruction::Const { val: (*byte1) });
                instr_list.push(Instruction::Const { val: (*byte1) });
            }
            // ld r16 a
            n if ld16a_p.is_match(&format!("{:08b}", n)) => {
                let register16: u8 = byte >> 4;
                instr_list.push(Instruction::Ld16a { reg: (register16) })
            }
            // ld a r16
            n if lda16_p.is_match(&format!("{:08b}", n)) => {
                let register16: u8 = byte >> 4;
                instr_list.push(Instruction::Lda16 { reg: (register16) })
            }
            // ld n16
            0b00001000 => {
                let (_, byte1) = enumerated_bytes.next().unwrap();
                let (_, byte2) = enumerated_bytes.next().unwrap();
                // careful with little endian imm16
                let immidiate16: u16 = (*byte2 as u16) << 8 | (*byte1 as u16);
                // push actual instruction with correct arguments
                instr_list.push(Instruction::Ldrn16sp { imm: (immidiate16) });
                // push the placeholder bytes
                instr_list.push(Instruction::Const { val: (*byte1) });
                instr_list.push(Instruction::Const { val: (*byte1) });
            }
            // Inc r16
            n if inc16_p.is_match(&format!("{:08b}", n)) => {
                let register16: u8 = byte >> 4;
                instr_list.push(Instruction::Inc16 { reg: (register16) })
            }
            //
            n if dec16_p.is_match(&format!("{:08b}", n)) => {
                let register16: u8 = byte >> 4;
                instr_list.push(Instruction::Dec16 { reg: (register16) })
            }
            //
            n if addhl16_p.is_match(&format!("{:08b}", n)) => {
                let register16: u8 = byte >> 4;
                instr_list.push(Instruction::Addhl16 { reg: (register16) })
            }
            _ => todo!("Oh oh"),
        };

        return Ok(instr_list);
    }
    return Ok(instr_list);
}
