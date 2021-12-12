use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed due to IOError {0}")]
    IOError(#[from] io::Error),

    #[error("Failed to parse TOML file {0}")]
    ParsingError(#[from] toml::de::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn print_errors(err: Error) {
    match err {
        Error::ParsingError(err) => match err.line_col() {
            Some((line, col)) => {
                //  Indexes are 0-based, we increment
                //  them to make them less confusing
                println!(
                    "\x1b[31mError\x1b[0m: At line {} column {}\nCaused by: {}",
                    line + 1,
                    col + 1,
                    err
                )
            }
            None => println!("\x1b[31mError\x1b[0m: {:?}", err),
        },
        Error::IOError(err) => {
            println!("\x1b[31mError\x1b[0m: {:?}", err);
        }
    }
}
