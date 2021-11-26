use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed due to IOError {0}")]
    IOError(#[from] io::Error),

    #[error("Failed to parse toml file {0}")]
    ParsingError(#[from] toml::de::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
