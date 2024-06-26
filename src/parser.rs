use std::io::{Bytes, Read};

use crate::{errors::EmulatorError, instructions::Instruction};

pub fn parse_instructions(
    bytes: Bytes<impl Read>,
    size: usize,
    debug: bool,
) -> Result<Vec<Instruction>, EmulatorError> {
    let size_kb = size / 1024;
    let size_digit_count = f64::log10(size_kb as f64).ceil() as usize;

    let mut instructions: Vec<Instruction> = Vec::with_capacity(size);
    // TODO: check how much this overallocates on average

    let mut enumerated_bytes = bytes.enumerate();
    while let Some((byte_num, byte_result)) = enumerated_bytes.next() {
        let byte = byte_result?;

        if debug {
            let progress = byte_num as f64 / size as f64;
            let progress_percent = progress * 100.0;
            if byte_num % 1024 == 0 {
                // print progress every kilobyte
                let kb_num = byte_num / 1024;
                println!(
                    "Parsing instructions... {progress_percent:6.2}% ({kb_num:size_digit_count$}KB/{size_kb}KB)",
                );
            }
            println!("Byte: '{byte:0>8b}' ('{byte:0>2x}')");
        }

        let instruction = match byte {
            0 => Instruction::Nop,
            _ => todo!("Instruction '{byte:0>8b}' ('{byte:0>2x}')"),
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
