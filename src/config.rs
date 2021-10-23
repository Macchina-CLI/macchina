use crate::cli::Opt;
use dirs::config_dir;
use std::io::Read;
use std::path::{Path, PathBuf};

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

impl Opt {
    pub fn from_config_file<S: AsRef<std::ffi::OsStr> + ?Sized>(
        path: &S,
    ) -> Result<Opt, &'static str> {
        let path = Path::new(path);
        if Path::exists(path) {
            if let Ok(mut file) = std::fs::File::open(path) {
                let mut buffer: Vec<u8> = Vec::new();
                if file.read_to_end(&mut buffer).is_ok() {
                    toml::from_slice(&buffer).or(Err("Failed to parse config"))
                } else {
                    Err("Failed to read config file")
                }
            } else {
                Err("Failed to open file")
            }
        } else {
            Err("File doesn't exist")
        }
    }
    pub fn from_config() -> Result<Opt, &'static str> {
        if let Some(path) = std::env::var_os("MACCHINA_CONF") {
            return Opt::from_config_file(&path);
        } else if let Some(mut path) = config_dir() {
            path.push(PKG_NAME);
            path.push(format!("{}.toml", PKG_NAME));
            if Path::exists(&path) {
                return Opt::from_config_file(&path);
            } else if cfg!(target_os = "macos") {
                if let Ok(home) = std::env::var("HOME") {
                    let path = PathBuf::from(home)
                        .join(".config")
                        .join(PKG_NAME)
                        .join(format!("{}.toml", PKG_NAME));

                    if Path::exists(&path) {
                        return Opt::from_config_file(&path);
                    }
                }
            }
        }
        Ok(Opt::default())
    }

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
        if args.no_ascii {
            self.no_ascii = true;
        }
        if args.palette.is_some() {
            self.palette = args.palette;
        }
        if args.long_shell {
            self.long_shell = true;
        }
        if args.interface.is_some() {
            self.interface = args.interface;
        }
        if args.long_uptime {
            self.long_uptime = true;
        }
        if args.show.is_some() {
            self.show = args.show;
        }
        if args.theme.is_some() {
            self.theme = args.theme;
        }
        if args.list_themes {
            self.list_themes = true;
        }
    }
}
