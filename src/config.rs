use crate::cli::Opt;
use crate::error::Result;
use libmacchina::dirs as _dirs;
use std::io;
use std::path::{Path, PathBuf};

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn locations() -> Vec<PathBuf> {
    let mut dirs = vec![];

    if cfg!(target_os = "macos") {
        dirs.push(config_dir().unwrap_or_default());
    } else {
        dirs.push(dirs::config_dir().unwrap_or_default());
    }

    if cfg!(target_os = "linux") {
        dirs.push(usr_share_dir().unwrap_or_default());
    }

    if cfg!(target_os = "netbsd") {
        dirs.push(_dirs::localbase_dir().unwrap_or_default());
    }

    dirs.retain(|x| x.exists());
    dirs
}

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
    if let Some(mut path) = config_dir() {
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

pub fn config_dir() -> Option<PathBuf> {
    if let Ok(home) = std::env::var("HOME") {
        Some(PathBuf::from(home).join(".config"))
    } else {
        None
    }
}

pub fn usr_share_dir() -> Option<PathBuf> {
    Some(PathBuf::from("/usr/share"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_documentation_config() -> Result<()> {
        read_config("doc/macchina.toml")?;
        Ok(())
    }
}
