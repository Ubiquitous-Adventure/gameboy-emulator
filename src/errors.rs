use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmulatorError {
    #[error("IOError: {0}")]
    IoError(#[from] io::Error),
    #[error("PlatformError: {0}")]
    PlatformError(String),
}
