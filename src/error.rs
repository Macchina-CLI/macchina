use colored::Colorize;
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
                println!(
                    "{}: At line {} column {}.\n{}: {}",
                    "Error".bright_red(),
                    (line + 1).to_string().yellow(),
                    (col + 1).to_string().yellow(),
                    "Caused by".bold(),
                    err
                )
            }
            None => println!("{}: {:?}", "Error".bright_red(), err),
        },
        Error::IOError(err) => {
            println!("{}: {:?}", "Error".bright_red(), err);
        }
    }
}
