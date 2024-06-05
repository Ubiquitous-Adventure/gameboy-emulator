use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmulatorError {
    #[error("IO-Error: {0}")]
    IoError(#[from] io::Error),
}
