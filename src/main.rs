mod parse;
use parse::read_file;
use std::env;
use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Usage: {} <file_path>", args[0]),
        ));
    }

    let argument = &args[1];

    let meta = fs::metadata(argument)?;
    if !meta.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("File {} not found", argument),
        ));
    }
    let bytes = read_file(argument)?;

    Ok(())
}
