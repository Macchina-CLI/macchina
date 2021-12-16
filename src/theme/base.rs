use crate::cli::Opt;
use crate::data::ReadoutKey;
use crate::theme;
use crate::theme::components::*;
use crate::Result;
use colored::Colorize;
use dirs;
use libmacchina::dirs as _dirs;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};
use tui::style::Color;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Theme {
    custom_ascii: ASCII,
    bar: Bar,
    r#box: Block,
    separator: String,
    randomize: Randomize,
    spacing: usize,
    padding: usize,
    palette: Palette,
    hide_ascii: bool,
    prefer_small_ascii: bool,
    keys: Keys,
    #[serde(with = "color_to_tui")]
    key_color: Color,
    #[serde(with = "color_to_tui")]
    separator_color: Color,
    #[serde(skip_serializing, skip_deserializing)]
    name: String,
    #[serde(skip_serializing, skip_deserializing)]
    filepath: PathBuf,
    #[serde(skip_serializing, skip_deserializing)]
    active: bool,
    // #[serde(skip_serializing, skip_deserializing)]
    // error: String,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            key_color: Color::Blue,
            separator_color: Color::Yellow,
            separator: String::from("-"),
            palette: Palette::default(),
            randomize: Randomize::default(),
            custom_ascii: ASCII::default(),
            bar: Bar::default(),
            r#box: Block::default(),
            keys: Keys::default(),
            name: String::new(),
            // error: String::new(),
            filepath: PathBuf::new(),
            active: false,
            hide_ascii: false,
            prefer_small_ascii: false,
            spacing: 2,
            padding: 2,
        }
    }
}

impl Theme {
    pub fn new(custom: Theme) -> Self {
        Theme {
            bar: custom.bar,
            key_color: custom.key_color,
            separator: custom.separator,
            separator_color: custom.separator_color,
            spacing: custom.spacing,
            padding: custom.padding,
            palette: custom.palette,
            hide_ascii: custom.hide_ascii,
            prefer_small_ascii: custom.prefer_small_ascii,
            r#box: custom.r#box,
            custom_ascii: custom.custom_ascii,
            randomize: custom.randomize,
            keys: custom.keys,
            name: custom.name,
            filepath: custom.filepath,
            active: custom.active,
            // error: custom.error,
        }
    }

    pub fn is_ascii_visible(&self) -> bool {
        !self.hide_ascii
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_filepath(&self) -> PathBuf {
        self.filepath.to_owned()
    }

    // pub fn get_error(&self) -> String {
    //     self.error.to_owned()
    // }

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn set_name(&mut self) {
        if let Some(f) = self.filepath.file_stem() {
            if let Some(s) = f.to_str() {
                self.name = s.to_string();
            }
        }
    }

    pub fn set_filepath(&mut self, p: PathBuf) {
        self.filepath = p
    }

    // pub fn set_error<S>(&mut self, e: S)
    // where
    //     S: std::string::ToString,
    // {
    //     self.error = e.to_string();
    // }

    pub fn get_bar(&self) -> &Bar {
        &self.bar
    }

    pub fn get_keys(&self) -> &Keys {
        &self.keys
    }

    pub fn get_randomization(&self) -> &Randomize {
        &self.randomize
    }

    pub fn get_custom_ascii(&self) -> &ASCII {
        &self.custom_ascii
    }

    pub fn get_palette(&self) -> &Palette {
        &self.palette
    }

    pub fn get_block(&self) -> &Block {
        &self.r#box
    }

    pub fn get_separator(&self) -> &str {
        &self.separator
    }

    pub fn get_key_color(&self) -> Color {
        self.key_color
    }

    pub fn get_separator_color(&self) -> Color {
        self.separator_color
    }

    pub fn prefers_small_ascii(&self) -> bool {
        self.prefer_small_ascii
    }

    pub fn get_padding(&self) -> usize {
        self.padding
    }

    pub fn get_spacing(&self) -> usize {
        self.spacing
    }

    pub fn set_active(&mut self, opt: &Opt) {
        if let Some(t) = &opt.theme {
            if &self.name == t {
                self.active = true;
            }
        }
    }

    pub fn set_separator_color(&mut self, color: Color) {
        self.separator_color = color;
    }

    pub fn set_padding(&mut self, size: usize) {
        self.padding = size
    }

    pub fn set_spacing(&mut self, spacing: usize) {
        self.spacing = spacing;
    }

    pub fn set_key_color(&mut self, color: Color) {
        self.key_color = color;
    }

    pub fn set_separator(&mut self, separator: impl ToString) {
        self.separator = separator.to_string()
    }

    pub fn set_randomization(&mut self) {
        if self.randomize.rand_key() {
            self.key_color = self.randomize.generate();
        }

        if self.randomize.rand_sep() {
            self.separator_color = self.randomize.generate();
        }
    }

    pub fn key(&self, readout_key: &ReadoutKey) -> &str {
        match *readout_key {
            ReadoutKey::Host => self.keys.get_host(),
            ReadoutKey::Kernel => self.keys.get_kernel(),
            ReadoutKey::OperatingSystem => self.keys.get_os(),
            ReadoutKey::Machine => self.keys.get_machine(),
            ReadoutKey::Distribution => self.keys.get_distro(),
            ReadoutKey::LocalIP => self.keys.get_local_ip(),
            ReadoutKey::Resolution => self.keys.get_resolution(),
            ReadoutKey::Shell => self.keys.get_shell(),
            ReadoutKey::Terminal => self.keys.get_terminal(),
            ReadoutKey::WindowManager => self.keys.get_wm(),
            ReadoutKey::DesktopEnvironment => self.keys.get_de(),
            ReadoutKey::Packages => self.keys.get_packages(),
            ReadoutKey::Processor => self.keys.get_cpu(),
            ReadoutKey::ProcessorLoad => self.keys.get_cpu_load(),
            ReadoutKey::Battery => self.keys.get_battery(),
            ReadoutKey::Backlight => self.keys.get_backlight(),
            ReadoutKey::Uptime => self.keys.get_uptime(),
            ReadoutKey::Memory => self.keys.get_memory(),
        }
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_active() {
            write!(f, "- {} {}", self.name.green().italic(), "[active]".cyan())
        } else {
            write!(f, "- {}", self.name.green().italic())
        }
    }
}

impl PartialEq for Theme {
    fn eq(&self, other: &Self) -> bool {
        self.filepath == other.filepath || self.name == other.name
    }
}

/// Searches for and returns a theme from a given directory.
pub fn get_theme(name: &str, dir: &Path) -> Result<Theme> {
    let theme_path = dir.join(&name);
    let buffer = std::fs::read(theme_path)?;
    Ok(toml::from_slice(&buffer)?)
}

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
    dirs.iter().map(|x| x.join("macchina/themes")).collect()
}

pub fn create_theme(locations: &[PathBuf], opt: &Opt) -> Theme {
    let mut theme = Theme::default();
    if let Some(th) = &opt.theme {
        let name = &(th.to_owned() + ".toml");
        locations.iter().any(|d| match get_theme(name, &d) {
            Ok(mut t) => {
                t.set_randomization();
                theme = t;
                true
            }
            _ => false,
        });
    }

    theme
}

pub fn list_themes(locations: &[PathBuf]) {
    use libmacchina::extra::path_extension;
    for dir in locations {
        let entries = libmacchina::extra::list_dir_entries(&dir);
        println!("{}:", dir.to_string_lossy());
        entries
            .iter()
            .filter(|&x| path_extension(x).unwrap_or_default() == "toml")
            .for_each(|dir| {
                if let Some(name) = dir.file_name() {
                    if let Some(str) = name.to_str() {
                        if let Ok(theme) = get_theme(str, dir.parent().unwrap()) {
                            println!("{}", theme.to_string());
                        }
                    }
                }
            });
    }
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
