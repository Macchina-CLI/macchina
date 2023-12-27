use crate::cli::{Opt, PKG_NAME};
use crate::error::Result;
use std::path::{Path, PathBuf};

pub fn read_config<S: AsRef<std::ffi::OsStr> + ?Sized>(path: &S) -> Result<Opt> {
    let path = Path::new(path);
    if path.exists() {
        let buffer = std::fs::read(path)?;
        let contents = std::str::from_utf8(buffer.as_slice())?;
        Ok(toml::from_str(&contents)?)
    } else {
        Ok(Opt::default())
    }
}

pub fn get_config() -> Result<Opt> {
    if cfg!(target_os = "macos") {
        if let Ok(home) = std::env::var("HOME") {
            let path = PathBuf::from(home)
                .join(".config")
                .join(PKG_NAME)
                .join(format!("{PKG_NAME}.toml"));

            return read_config(&path);
        }
    } else if let Some(mut path) = dirs::config_dir() {
        path.push(PKG_NAME);
        path.push(format!("{PKG_NAME}.toml"));

        return read_config(&path);
    }

    Ok(Opt::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn documentation_config() -> Result<()> {
        let opt = read_config("macchina.toml")?;

        assert!(opt.long_uptime);
        assert!(!opt.long_shell);
        assert!(!opt.long_kernel);
        assert!(opt.current_shell);
        assert!(opt.physical_cores);
        assert_eq!(opt.interface, Some(String::from("wlan0")));
        Ok(())
    }
}
