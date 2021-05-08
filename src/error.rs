use std::io;

#[derive(Debug,Error)]
pub enum Error {
    ConfigError,
    IOError(io::Error),

}
