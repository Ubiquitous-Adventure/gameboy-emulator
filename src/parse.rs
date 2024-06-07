use std::{
    fs::File,
    io::{BufReader, Read, Result},
};

pub fn read_file(path: &str) -> Result<Vec<u8>> {
    // open file and transform into bufreader
    let mut reader = BufReader::new(File::open(path)?);
    let mut bytes: Vec<u8> = Vec::new();

    reader.read_to_end(&mut bytes)?;
    Ok(bytes)
}
