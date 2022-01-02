use crate::cli::{Opt, PKG_NAME};
use crate::error::Result;
use std::path::{Path, PathBuf};

pub fn read_config<S: AsRef<std::ffi::OsStr> + ?Sized>(path: &S) -> Result<Opt> {
    let path = Path::new(path);
    if path.exists() {
        let config_buffer = std::fs::read(path)?;
        Ok(toml::from_slice(&config_buffer)?)
    } else {
        Ok(Opt::default())
    }
}

pub fn get_config() -> Result<Opt> {
    if let Some(mut path) = dirs::config_dir() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn documentation_config() -> Result<()> {
        let opt = read_config("doc/macchina.toml")?;

        assert!(opt.long_uptime);
        assert!(!opt.long_shell);
        assert!(!opt.long_kernel);
        assert!(opt.current_shell);
        assert!(opt.physical_cores);
        assert_eq!(opt.interface,Some(String::from("wlan0")));
        Ok(())
    }
}
