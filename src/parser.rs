use std::{
    fmt::Debug,
    io::{Bytes, Error, Read},
};

use crate::instructions::Instruction;

pub fn parse_instructions(
    bytes: Bytes<impl Debug + Read>,
    size: usize,
    debug: bool,
) -> Result<Vec<Instruction>, Error> {
    let mut enumerated_bytes = bytes.enumerate();
    let size_kb = size / 1024;
    let size_width = f64::log10(size_kb as f64).ceil() as usize;

    let mut instructions: Vec<Instruction> = Vec::with_capacity(size);
    // TODO: check how much this overallocates on average

    while let Some((byte_num, byte_result)) = enumerated_bytes.next() {
        let byte = byte_result?;

        let progress = byte_num as f64 / size as f64;
        let progress_percent = progress * 100.0;
        if debug {
            if byte_num % 1024 == 0 {
                // print progress every kilobyte
                let kb_num = byte_num / 1024;
                println!(
                    "Parsing instructions... {progress_percent:6.2}% ({kb_num:size_width$}KB/{size_kb}KB)",
                );
            }
            println!("Byte: '{:0>8b}' ('{:0>2x}')", byte, byte);
        }

        let instruction = match byte {
            0 => Instruction::Nop,
            _ => todo!("Instruction '{byte:0>8b}' ('{byte:0>2x}')"),
        };
        instructions.push(instruction);
    }
    Ok(instructions)
}
