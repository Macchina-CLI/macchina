use crate::cli::Opt;
use dirs::config_dir;
use std::io::Read;
use std::path::{Path, PathBuf};

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

impl Opt {
    pub fn read_config<S: AsRef<std::ffi::OsStr> + ?Sized>(path: &S) -> Result<Opt, &'static str> {
        let path = Path::new(path);
        if !path.exists() {
            return Err("Failed to locate the configuration file.");
        }

        if let Ok(mut file) = std::fs::File::open(path) {
            let mut buffer: Vec<u8> = Vec::new();
            if file.read_to_end(&mut buffer).is_ok() {
                return toml::from_slice(&buffer).or(Err("Failed to parse configuration file."));
            }

            return Err("Failed to read configuration file.");
        }

        Err("Failed to open configuration file.")
    }

    pub fn get_config() -> Result<Opt, &'static str> {
        if let Some(path) = std::env::var_os("MACCHINA_CONF") {
            return Opt::read_config(&path);
        } else if let Some(mut path) = config_dir() {
            match cfg!(target_os = "macos") {
                true => {
                    if let Ok(home) = std::env::var("HOME") {
                        path = PathBuf::from(home)
                            .join(".config")
                            .join(PKG_NAME)
                            .join(format!("{}.toml", PKG_NAME));
                    }
                }
                _ => {
                    path.push(PKG_NAME);
                    path.push(format!("{}.toml", PKG_NAME));
                }
            }

            return Opt::read_config(&path);
        }

        Ok(Opt::default())
    }

    /// Patches `opt` struct with the values provided in the command-line
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
