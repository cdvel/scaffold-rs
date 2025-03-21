// ready-to-use error module template
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    // Add more error variants here
}

pub type Result<T> = std::result::Result<T, Error>;
