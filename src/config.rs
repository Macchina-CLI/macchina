use crate::cli::Opt;
use crate::Result;
use std::path::Path;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(target_os = "macos")]
fn get_config() -> Result<Opt> {
    use std::path::PathBuf;

    if let Ok(home) = std::env::var("HOME") {
        let path = PathBuf::from(home)
            .join(".config")
            .join(PKG_NAME)
            .join(format!("{}.toml", PKG_NAME));

        if Path::exists(&path) {
            return Opt::from_config_file(&path);
        }
    }

    Ok(Opt::default())
}

#[cfg(not(target_os = "macos"))]
fn get_config() -> Result<Opt> {
    if let Some(conf_dir) = dirs::config_dir() {
        let path = conf_dir.join(PKG_NAME).join(format!("{}.toml", PKG_NAME));

        return Opt::from_config_file(&path);
    }

    Ok(Opt::default())
}

impl Opt {
    pub fn from_config_file<S: AsRef<std::ffi::OsStr> + ?Sized>(path: &S) -> Result<Opt> {
        let path = Path::new(path);
        Ok(if Path::exists(path) {
            let config_buffer = std::fs::read(path)?;
            Ok(toml::from_slice(&config_buffer)?)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Config file was not found",
            ))
        }?)
    }

    /// Reads config file specified by MACCHINA_CONF environment variable (or from a hardcoded
    /// directory if MACCHINA_CONF is not set)
    pub fn from_config() -> Result<Opt> {
        if let Some(path) = std::env::var_os("MACCHINA_CONF") {
            Opt::from_config_file(&path)
        } else {
            get_config()
        }
    }

    /// Patches `opt` with the flags provided in the command-line.
    pub fn patch_args(&mut self, args: Self) {
        if args.version {
            self.version = true;
        }

        if args.doctor {
            self.doctor = true;
        }

        if args.export_config {
            self.export_config = true;
        }

        if args.current_shell {
            self.current_shell = true;
        }

        if args.long_shell {
            self.long_shell = true;
        }

        if args.long_uptime {
            self.long_uptime = true;
        }

        if args.list_themes {
            self.list_themes = true;
        }

        if args.long_kernel {
            self.long_shell = true;
        }

        if args.physical_cores {
            self.physical_cores = true;
        }

        if args.ascii_artists {
            self.ascii_artists = true;
        }

        if args.config.is_some() {
            self.config = args.config;
        }

        if args.theme.is_some() {
            self.theme = args.theme;
        }

        if args.show.is_some() {
            self.show = args.show;
        }

        if args.interface.is_some() {
            self.interface = args.interface;
        }
    }
}
