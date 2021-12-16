use crate::cli::{Opt, PKG_NAME};
use crate::error::Result;
use crate::extra;
use std::io;
use std::path::{Path};

pub fn read_config_file(path: &std::path::PathBuf) -> Result<Opt> {
    let path = Path::new(path);
    Ok(if Path::exists(path) {
        let config_buffer = std::fs::read(path)?;
        Ok(toml::from_slice(&config_buffer)?)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Could not locate the configuration file.",
        ))
    }?)
}

pub fn get_config() -> Result<Opt> {
    if let Some(mut path) = extra::config_dir() {
        path.push(PKG_NAME);
        path.push(format!("{}.toml", PKG_NAME));
        if Path::exists(&path) {
            return read_config_file(&path);
        }
    }

    Ok(Opt::default())
}
