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
        if args.bar {
            self.bar = true;
        }
        if args.box_title.is_some() {
            self.box_title = args.box_title;
        }
        if args.color.is_some() {
            self.color = args.color;
        }
        if args.custom_ascii.is_some() {
            self.custom_ascii = args.custom_ascii;
        }
        if args.custom_ascii_color.is_some() {
            self.custom_ascii_color = args.custom_ascii_color;
        }
        if args.version {
            self.version = true;
        }
        if args.doctor {
            self.doctor = true;
        }
        if args.export_config {
            self.export_config = true;
        }
        if args.hide.is_some() {
            self.hide = args.hide;
        }
        if args.no_ascii {
            self.no_ascii = true;
        }
        if args.no_bar_delimiter {
            self.no_bar_delimiter = true;
        }
        if args.no_box {
            self.no_box = true;
        }
        if args.no_color {
            self.no_color = true;
        }
        if args.no_separator {
            self.no_separator = true;
        }
        if args.no_title {
            self.no_title = true;
        }
        if args.padding.is_some() {
            self.padding = args.padding;
        }
        if args.palette.is_some() {
            self.palette = args.palette;
        }
        if args.random_color {
            self.random_color = true;
        }
        if args.random_sep_color {
            self.random_sep_color = true;
        }
        if args.separator_color.is_some() {
            self.separator_color = args.separator_color;
        }
        if args.long_shell {
            self.long_shell = true;
        }
        if args.short_uptime {
            self.short_uptime = true;
        }
        if args.show_only.is_some() {
            self.show_only = args.show_only;
        }
        if args.spacing.is_some() {
            self.spacing = args.spacing;
        }
        if args.theme.is_some() {
            self.theme = args.theme;
        }
        if args.list_themes {
            self.list_themes = true;
        }
    }

    // Checks for conflicts between the different flags.
    pub fn check_conflicts(&self) -> Vec<&str> {
        let mut conflicts = vec![];

        if self.hide.is_some() && self.show_only.is_some() {
            conflicts.push("hide and show_only");
        }

        conflicts
    }
}
