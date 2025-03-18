// ready-to-use error module template
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),
    // Add more error variants here
}

pub type Result<T> = std::result::Result<T, Error>;
