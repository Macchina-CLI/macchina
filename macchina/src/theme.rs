#![allow(dead_code)]
use crate::data::ReadoutKey;
use clap::arg_enum;
use std::collections::HashMap;
use tui::style::Color;

/// `Bar` contains several elements that make up a `Theme`, such as the: \
/// - `Bar` glyph
/// - `Bar` opening symbol
/// - `Bar` closing symbol
#[derive(Debug, Clone)]
pub struct BarStyle {
    /// This is the glyph/symbol that represents the usage.
    pub glyph: &'static str,
    /// This is used to indicate the beginning of a bar.
    pub symbol_open: char,
    /// This is used to indicate the end of a bar.
    pub symbol_close: char,
}

/// This implements all the different ways a `Bar` can look.
impl BarStyle {
    fn squared() -> BarStyle {
        BarStyle {
            glyph: "■",
            symbol_open: '[',
            symbol_close: ']',
        }
    }

    fn rounded() -> BarStyle {
        BarStyle {
            glyph: "●",
            symbol_open: '(',
            symbol_close: ')',
        }
    }

    fn angled() -> BarStyle {
        BarStyle {
            glyph: "×",
            symbol_open: '<',
            symbol_close: '>',
        }
    }
}

arg_enum! {
    #[derive(Debug, PartialEq)]
    pub enum Themes {
        Hydrogen,
        Helium,
        Lithium,
        EmojiTheme
    }
}

impl Themes {
    pub fn create_instance(&self) -> Box<dyn Theme> {
        match self {
            Themes::Hydrogen => HydrogenTheme::new(),
            Themes::Helium => HeliumTheme::new(),
            Themes::Lithium => LithiumTheme::new(),
            Themes::EmojiTheme => EmojiTheme::new(),
        }
    }
}

/// Defines the different ways a key can be named, let's take the _OperatingSystem variant_ for example: \
/// - `AbbreviationType::Classic` -> OS \
/// - `AbbreviationType::Alternative` -> Ope \
/// - `AbbreviationType::Long` -> Operating System
#[derive(Eq, PartialEq, Hash)]
pub enum AbbreviationType {
    Classic,
    Alternative,
    Long,
}

/// This implements all the different ways a `Key` can be named using \
/// the predefined variants found in the `AbbreviationType` enum.
impl ReadoutKey {
    fn get_common_name(&self) -> HashMap<&AbbreviationType, &'static str> {
        let mut values = HashMap::new();

        match self {
            ReadoutKey::Host => {
                values.insert(&AbbreviationType::Classic, "Host");
                values.insert(&AbbreviationType::Alternative, "Hos");
                values.insert(&AbbreviationType::Long, "Host");
            }
            ReadoutKey::Machine => {
                values.insert(&AbbreviationType::Classic, "Machine");
                values.insert(&AbbreviationType::Alternative, "Mac");
                values.insert(&AbbreviationType::Long, "Machine");
            }
            ReadoutKey::Kernel => {
                values.insert(&AbbreviationType::Classic, "Kernel");
                values.insert(&AbbreviationType::Alternative, "Ker");
                values.insert(&AbbreviationType::Long, "Kernel");
            }
            ReadoutKey::Distribution => {
                values.insert(&AbbreviationType::Classic, "Distro");
                values.insert(&AbbreviationType::Alternative, "Dis");
                values.insert(&AbbreviationType::Long, "Distribution");
            }
            ReadoutKey::OperatingSystem => {
                values.insert(&AbbreviationType::Classic, "OS");
                values.insert(&AbbreviationType::Alternative, "Ope");
                values.insert(&AbbreviationType::Long, "Operating System");
            }
            ReadoutKey::DesktopEnvironment => {
                values.insert(&AbbreviationType::Classic, "DE");
                values.insert(&AbbreviationType::Alternative, "Des");
                values.insert(&AbbreviationType::Long, "Desktop Environment");
            }
            ReadoutKey::WindowManager => {
                values.insert(&AbbreviationType::Classic, "WM");
                values.insert(&AbbreviationType::Alternative, "Win");
                values.insert(&AbbreviationType::Long, "Window Manager");
            }
            ReadoutKey::Packages => {
                values.insert(&AbbreviationType::Classic, "Packages");
                values.insert(&AbbreviationType::Alternative, "Pac");
                values.insert(&AbbreviationType::Long, "Packages");
            }
            ReadoutKey::Shell => {
                values.insert(&AbbreviationType::Classic, "Shell");
                values.insert(&AbbreviationType::Alternative, "She");
                values.insert(&AbbreviationType::Long, "Shell");
            }
            ReadoutKey::Terminal => {
                values.insert(&AbbreviationType::Classic, "Terminal");
                values.insert(&AbbreviationType::Alternative, "Ter");
                values.insert(&AbbreviationType::Long, "Terminal");
            }
            ReadoutKey::Uptime => {
                values.insert(&AbbreviationType::Classic, "Uptime");
                values.insert(&AbbreviationType::Alternative, "Upt");
                values.insert(&AbbreviationType::Long, "Uptime");
            }
            ReadoutKey::Processor => {
                values.insert(&AbbreviationType::Classic, "CPU");
                values.insert(&AbbreviationType::Alternative, "Cpu");
                values.insert(&AbbreviationType::Long, "Processor");
            }
            ReadoutKey::LocalIP => {
                values.insert(&AbbreviationType::Classic, "Local IP");
                values.insert(&AbbreviationType::Alternative, "Adr");
                values.insert(&AbbreviationType::Long, "Local IP");
            }
            ReadoutKey::Memory => {
                values.insert(&AbbreviationType::Classic, "Memory");
                values.insert(&AbbreviationType::Alternative, "Mem");
                values.insert(&AbbreviationType::Long, "Memory");
            }
            ReadoutKey::Battery => {
                values.insert(&AbbreviationType::Classic, "Battery");
                values.insert(&AbbreviationType::Alternative, "Bat");
                values.insert(&AbbreviationType::Long, "Battery");
            }
        }

        values
    }
}

/// This trait provides the necessary functions for creating _themes_.
pub trait Theme {
    fn new() -> Box<dyn Theme>
    where
        Self: Sized;

    fn get_separator(&self) -> &'static str;
    fn set_separator(&mut self, separator: &'static str);

    fn get_separator_color(&self) -> Color;
    fn set_separator_color(&mut self, color: Color);

    fn get_color(&self) -> Color;
    fn set_color(&mut self, color: Color);

    fn get_padding(&self) -> usize;
    fn set_padding(&mut self, size: usize);

    fn get_spacing(&self) -> usize;
    fn set_spacing(&mut self, spacing: usize);

    fn default_abbreviation(&self) -> &AbbreviationType;

    fn key(&self, readout_key: &ReadoutKey, abbreviation: &AbbreviationType) -> &'static str {
        let abbreviated_names = readout_key.get_common_name();
        let name_entry = abbreviated_names.get(&abbreviation);

        if let Some(name) = name_entry {
            name
        } else {
            abbreviated_names.values().next().unwrap()
        }
    }
}

/// This structure's implementation utilizes the following:
/// - _Rounded_ bar through `Bar::rounded()`
/// - _Dash_ style through `Misc::dash()`
/// - _Classic_ abbreviation type through `AbbreviationType::Classic`
#[derive(Debug, Clone)]
pub struct HydrogenTheme {
    bar: BarStyle,
    color: Color,
    separator_color: Color,
    separator: &'static str,
    spacing: usize,
    padding: usize,
}

impl Theme for HydrogenTheme {
    fn new() -> Box<dyn Theme> {
        Box::new(HydrogenTheme {
            bar: BarStyle::rounded(),
            color: Color::Red,
            separator_color: Color::White,
            separator: "—",
            spacing: 2,
            padding: 4,
        })
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

    fn default_abbreviation(&self) -> &AbbreviationType {
        &AbbreviationType::Classic
    }
}
/// This structure's implementation utilizes the following:
/// - _Squared_ bar through `Bar::squared()`
/// - _Arrow_ style through `Misc::arrow()`
/// - _Alternative_ abbreviation type through `AbbreviationType::Alternative`
#[derive(Debug, Clone)]
pub struct HeliumTheme {
    bar: BarStyle,
    color: Color,
    separator_color: Color,
    separator: &'static str,
    spacing: usize,
    padding: usize,
}

impl Theme for HeliumTheme {
    fn new() -> Box<dyn Theme> {
        Box::new(HeliumTheme {
            bar: BarStyle::squared(),
            color: Color::Green,
            separator_color: Color::White,
            separator: "=>",
            spacing: 2,
            padding: 4,
        })
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

    fn default_abbreviation(&self) -> &AbbreviationType {
        &AbbreviationType::Alternative
    }
}

/// This structure's implementation utilizes the following:
/// - _Angled_ bar through `Bar::angled()`
/// - _Squiggly_ style through `Misc::squiggly()`
/// - _Long_ abbreviation type through `AbbreviationType::Long`
#[derive(Debug, Clone)]
pub struct LithiumTheme {
    bar: BarStyle,
    color: Color,
    separator_color: Color,
    separator: &'static str,
    spacing: usize,
    padding: usize,
}

impl Theme for LithiumTheme {
    fn new() -> Box<dyn Theme> {
        Box::new(LithiumTheme {
            bar: BarStyle::angled(),
            color: Color::Yellow,
            separator_color: Color::White,
            separator: "~",
            spacing: 2,
            padding: 4,
        })
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

    fn default_abbreviation(&self) -> &AbbreviationType {
        &AbbreviationType::Long
    }
}

#[derive(Debug, Clone)]
pub struct EmojiTheme {
    bar: BarStyle,
    color: Color,
    separator_color: Color,
    separator: &'static str,
    spacing: usize,
    padding: usize,
}

impl Theme for EmojiTheme {
    fn new() -> Box<dyn Theme> {
        Box::new(EmojiTheme {
            bar: BarStyle {
                glyph: "🔴",
                symbol_open: '﹙',
                symbol_close: '﹚',
            },
            color: Color::LightBlue,
            separator_color: Color::White,
            separator: "👉",
            spacing: 2,
            padding: 4,
        })
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

    fn default_abbreviation(&self) -> &AbbreviationType {
        &AbbreviationType::Long
    }
}
