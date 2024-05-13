use colored::Colorize;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed due to IOError {0}")]
    IO(#[from] io::Error),

    #[error("Failed due to Utf8Error {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("Failed to parse TOML file {0}")]
    Parsing(#[from] toml::de::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn print_errors(err: Error) {
    match err {
        Error::Parsing(err) => {
            println!("{}: {}", "Error".bright_red(), err.message());
        }
        Error::Utf8(err) => {
            println!("{}: {:?}", "Error".bright_red(), err);
        }
        Error::IO(err) => {
            println!("{}: {:?}", "Error".bright_red(), err);
        }
    }
}
