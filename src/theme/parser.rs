use crate::theme::{AbbreviationType, BarStyle, Color, Theme};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(remote = "Color")]
enum ColorDef {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomTheme {
    name: &'static str,
    bar: BarStyle,
    #[serde(with = "ColorDef")]
    color: Color,
    #[serde(with = "ColorDef")]
    separator_color: Color,
    separator: &'static str,
    spacing: usize,
    padding: usize,
    block_title: String,
}

impl Theme for CustomTheme {
    fn new() -> Box<dyn Theme> {
        Box::new(CustomTheme {
            name: "CustomTheme",
            bar: BarStyle::rounded(),
            color: Color::Blue,
            separator_color: Color::White,
            separator: "•",
            spacing: 2,
            padding: 0,
            block_title: "theme".to_string(),
        })
    }

    fn get_bar_style(&self) -> &BarStyle {
        &self.bar
    }

    fn set_bar_style(&mut self, new_bar: BarStyle) {
        self.bar = new_bar
    }

    fn get_separator(&self) -> &'static str {
        self.separator
    }

    fn set_separator(&mut self, separator: &'static str) {
        self.separator = separator
    }

    fn get_separator_color(&self) -> Color {
        self.separator_color
    }

    fn set_separator_color(&mut self, color: Color) {
        self.separator_color = color
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color
    }

    fn get_padding(&self) -> usize {
        self.padding
    }

    fn set_padding(&mut self, size: usize) {
        self.padding = size
    }

    fn get_spacing(&self) -> usize {
        self.spacing
    }

    fn set_spacing(&mut self, spacing: usize) {
        self.spacing = spacing;
    }

    fn get_block_title(&self) -> &str {
        &self.block_title
    }

    fn set_block_title(&mut self, s: &str) {
        self.block_title = s.into()
    }

    fn default_abbreviation(&self) -> &AbbreviationType {
        &AbbreviationType::Long
    }
}

impl CustomTheme {

}
