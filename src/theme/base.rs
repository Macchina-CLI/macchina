use crate::cli::{Opt, PKG_NAME};
use crate::data::ReadoutKey;
use crate::extra;
use crate::theme::components::*;
use crate::Result;
use colored::Colorize;
use dirs;
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

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

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

    pub fn set_active(&mut self, theme_name: Option<&String>) {
        if let Some(name) = theme_name {
            if self.name.eq(name) {
                self.active = true;
            }
        }
    }

    fn set_name(&mut self) {
        if let Some(f) = self.filepath.file_stem() {
            if let Some(s) = f.to_str() {
                self.name = s.to_string();
            }
        }
    }

    fn set_filepath(&mut self, p: PathBuf) {
        self.filepath = p
    }

    fn set_randomization(&mut self) {
        if self.randomize.rand_key() {
            self.key_color = self.randomize.generate();
        }

        if self.randomize.rand_sep() {
            self.separator_color = self.randomize.generate();
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
            write!(
                f,
                "- {} {}",
                self.name.bright_green().italic(),
                "(active)".cyan().bold()
            )
        } else {
            write!(f, "- {}", self.name.bright_green().italic())
        }
    }
}

impl PartialEq for Theme {
    fn eq(&self, other: &Self) -> bool {
        self.filepath == other.filepath || self.name == other.name
    }
}

/// Predefined locations where macchina should search.
pub fn locations() -> Vec<PathBuf> {
    let mut dirs = vec![];

    if cfg!(target_os = "linux") {
        dirs.push(extra::config_dir().unwrap_or_default());
    } else {
        dirs.push(dirs::config_dir().unwrap_or_default());
    }

    if cfg!(target_os = "linux") {
        dirs.push(extra::usr_share_dir().unwrap_or_default());
    }

    #[cfg(target_os = "netbsd")]
    dirs.push(libmacchina::dirs::localbase_dir().unwrap_or_default());

    dirs.retain(|x| x.exists());
    dirs.iter()
        .map(|x| x.join(PKG_NAME).join("themes"))
        .collect()
}

/// Searches for and returns a theme from a given directory.
pub fn get_theme(path: &Path) -> Result<Theme> {
    let buffer = std::fs::read(path)?;
    Ok(toml::from_slice(&buffer)?)
}

/// Searches for and returns the specified theme.
pub fn create_theme(opt: &Opt) -> Theme {
    let locations = locations();
    let mut theme = Theme::default();
    if let Some(th) = &opt.theme {
        locations.iter().find(|&d| {
            let theme_path = d.join(&format!("{}.toml", th));
            match get_theme(&theme_path) {
                Ok(t) => {
                    theme = t;
                    theme.set_randomization();
                    true
                }
                _ => false,
            }
        });
    }

    theme
}

/// Prints out a list of available themes.
pub fn list_themes(opt: &Opt) {
    // 1. Iterate over locations
    // 2. Print current location
    // 2. Iterate over TOML files
    // 3. Display themes.
    let locations = locations();
    locations.iter().for_each(|dir| {
        println!("{}:", dir.to_string_lossy());
        if let Some(mut entries) = extra::get_entries(dir) {
            entries.sort();
            entries
                .iter()
                .filter(|x| extra::path_extension(x).unwrap_or_default() == "toml")
                .for_each(|theme| {
                    if let Some(str) = theme.file_name() {
                        if let Some(name) = str.to_str() {
                            match get_theme(theme) {
                                Ok(mut t) => {
                                    t.set_filepath(dir.join(name));
                                    t.set_name();
                                    t.set_active(opt.theme.as_ref());
                                    println!("{}", t);
                                }
                                Err(e) => {
                                    println!(
                                        "- {}: {}",
                                        name.replace(".toml", ""),
                                        e.to_string().yellow()
                                    );
                                }
                            }
                        }
                    }
                });
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_additional_themes() -> Result<()> {
        for theme in std::fs::read_dir("contrib/themes")? {
            let theme = theme?.path();
            get_theme(&theme)?;
        }
        Ok(())
    }
}
