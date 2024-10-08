use std::{fs::File, io::Read, path::PathBuf, process::ExitCode, time::Instant};

use clap::Parser;

mod errors;
mod instructions;
mod parser;
mod registers;

use errors::EmulatorError;
use parser::parse_instructions;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    game_file: PathBuf,
    #[arg(short, long)]
    debug: bool,
}

fn main() -> ExitCode {
    let result = run();
    if let Err(error) = result {
        println!("{}", error);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> Result<(), EmulatorError> {
    if cfg!(target_endian = "big") {
        return Err(EmulatorError::PlatformError(
            "This emulator only supports little endianness.".to_string(),
        ));
    }

    println!("Hello, gameboys!");

    let cli = Cli::parse();
    println!(
        "Game file: {}",
        cli.game_file
            .to_str()
            .expect("Game file path should be valid string")
    );
    println!("Debug mode: {}", cli.debug);

    if !cli.game_file.is_file() {
        println!("Provided path is not a file");
        return Ok(());
    }

    let file_size = cli.game_file.metadata()?.len();
    if cli.debug {
        println!("Total size of file: {} bytes", file_size);
    }

    let file = File::open(cli.game_file)?;

    let mut time_start: Option<Instant> = None;
    if cli.debug {
        time_start = Some(Instant::now());
    }

    let instructions = parse_instructions(file.bytes(), file_size as usize, cli.debug)?;

    if cli.debug {
        if let Some(start) = time_start {
            println!("Time taken to parse the file: {:?}", start.elapsed());
            println!("Parsed {} instructions.", instructions.len());
        }
    }

    Ok(())
}
