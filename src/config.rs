use crate::cli::Opt;
use dirs::config_dir;
use std::io::Read;
use std::path::Path;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

impl Opt {
    pub fn from_config() -> Result<Opt, &'static str> {
        if let Some(mut path) = config_dir() {
            path.push(PKG_NAME);
            path.push(format!("{}.toml", PKG_NAME));
            if Path::exists(&path) {
                if let Ok(mut file) = std::fs::File::open(&path) {
                    let mut buffer: String = String::new();
                    if file.read_to_string(&mut buffer).is_ok() {
                        return toml::from_str(&buffer).or(Err("Failed to parse config"));
                    }
                } else {
                    return Err("Failed to open config file");
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
        if args.palette {
            self.palette = true;
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
        if args.short_shell {
            self.short_shell = true;
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

        if self.separator_color.is_some() && self.no_separator {
            conflicts.push("separator_color and no_separator");
        }

        if self.color.is_some() && self.no_color {
            conflicts.push("color and no_color");
        }

        if self.separator_color.is_some() && self.no_color {
            conflicts.push("separator_color and no_color");
        }

        if self.custom_ascii.is_some() && self.no_ascii {
            conflicts.push("custom_ascii and no_ascii");
        }

        if self.custom_ascii_color.is_some() && self.no_ascii {
            conflicts.push("custom_ascii_color and no_ascii");
        }

        if self.small_ascii && self.no_ascii {
            conflicts.push("small_ascii and no_ascii");
        }

        if self.box_title.is_some() && self.no_box {
            conflicts.push("box_title and no_box");
        }

        if self.box_title.is_some() && self.no_title {
            conflicts.push("box_title and no_title");
        }

        conflicts
    }
}
