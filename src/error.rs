use colored::Colorize;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed due to IOError {0}")]
    IOError(#[from] io::Error),

    #[error("Failed due to Utf8Error {0}")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("Failed to parse TOML file {0}")]
    ParsingError(#[from] toml::de::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn print_errors(err: Error) {
    match err {
        Error::ParsingError(err) => {
            println!("{}: {}", "Error".bright_red(), err.message());
        }
        Error::Utf8Error(err) => {
            println!("{}: {:?}", "Error".bright_red(), err);
        }
        Error::IOError(err) => {
            println!("{}: {:?}", "Error".bright_red(), err);
        }
    }
}
