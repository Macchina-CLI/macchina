use crate::theme::components::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use toml;
use tui::style::Color;

/// This structure defines the skeleton of custom themes which are deserialized from TOML files.
/// See [https://github.com/Macchina-CLI/macchina/blob/main/theme/Carbon.toml](this) for an example
/// theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Theme {
    pub custom_ascii: ASCII,
    pub bar: Bar,
    pub r#box: Block,
    separator: String,
    pub randomize: Randomize,
    spacing: usize,
    padding: usize,
    palette: Option<Palette>,
    hide_ascii: bool,
    prefer_small_ascii: bool,
    pub keys: Keys,
    #[serde(with = "color_parser_tui")]
    key_color: Color,
    #[serde(with = "color_parser_tui")]
    separator_color: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            key_color: Color::Blue,
            separator_color: Color::Yellow,
            separator: String::from("-"),
            hide_ascii: false,
            prefer_small_ascii: false,
            palette: None,
            spacing: 2,
            padding: 2,
            randomize: Randomize::default(),
            custom_ascii: ASCII::default(),
            bar: Bar::default(),
            r#box: Block::default(),
            keys: Keys::default(),
        }
    }
}

impl Theme {
    pub fn new(custom: Theme) -> Self {
        Self {
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
        }
    }

    pub fn get_bar_style(&self) -> &Bar {
        &self.bar
    }

    pub fn set_bar_style(&mut self, new_bar: Bar) {
        self.bar = new_bar
    }

    pub fn get_separator(&self) -> &str {
        &self.separator
    }

    pub fn set_separator(&mut self, separator: impl ToString) {
        self.separator = separator.to_string()
    }

    pub fn get_separator_color(&self) -> Color {
        self.separator_color
    }

    pub fn set_separator_color(&mut self, color: Color) {
        self.separator_color = color
    }

    pub fn get_key_color(&self) -> Color {
        self.key_color
    }

    pub fn set_key_color(&mut self, color: Color) {
        self.key_color = color
    }

    pub fn prefers_small_ascii(&self) -> bool {
        self.prefer_small_ascii
    }

    pub fn is_ascii_hidden(&self) -> bool {
        self.hide_ascii
    }

    pub fn get_palette_type(&self) -> Option<&Palette> {
        self.palette.as_ref()
    }

    pub fn get_padding(&self) -> usize {
        self.padding
    }

    pub fn set_padding(&mut self, size: usize) {
        self.padding = size
    }

    pub fn get_spacing(&self) -> usize {
        self.spacing
    }

    pub fn set_spacing(&mut self, spacing: usize) {
        self.spacing = spacing;
    }

    /// Searches for and returns a theme from a given directory.
    pub fn get_theme(name: &str, dir: Option<PathBuf>) -> Result<Self, std::io::Error> {
        use std::io::Read;
        let mut theme_path = std::path::PathBuf::new();
        theme_path.push(dir.ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "$XDG_CONFIG_HOME was not found; fallback $HOME/.config also failed.",
            )
        })?);

        theme_path.push(std::path::Path::new(&format!(
            "macchina/themes/{}.toml",
            name
        )));

        let mut buffer: Vec<u8> = Vec::new();
        let mut theme = std::fs::File::open(theme_path)?;
        theme.read_to_end(&mut buffer)?;

        toml::from_slice(&buffer).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Could not parse theme.")
        })
    }
}
