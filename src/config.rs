use crate::cli::Opt;
use crate::error::Result;
use crate::theme;
use std::io;
use std::path::{Path, PathBuf};

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn read_config<S: AsRef<std::ffi::OsStr> + ?Sized>(path: &S) -> Result<Opt> {
    let path = Path::new(path);
    Ok(if Path::exists(path) {
        let config_buffer = std::fs::read(path)?;
        Ok(toml::from_slice(&config_buffer)?)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Could not locate the configuration file",
        ))
    }?)
}

pub fn get_config() -> Result<Opt> {
    if let Some(mut path) = theme::config_dir() {
        match cfg!(target_os = "macos") {
            true => {
                if let Ok(home) = std::env::var("HOME") {
                    path = PathBuf::from(home)
                        .join(".config")
                        .join(PKG_NAME)
                        .join(format!("{}.toml", PKG_NAME));
                }
            }
            false => {
                path.push(PKG_NAME);
                path.push(format!("{}.toml", PKG_NAME));
            }
        }

        return read_config(&path);
    }

    Ok(Opt::default())
}
