use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tui::style::Color;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Randomize {
    key_color: bool,
    separator_color: bool,
}

impl Default for Randomize {
    fn default() -> Self {
        Randomize {
            key_color: false,
            separator_color: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ASCII {
    path: Option<PathBuf>,
    #[serde(with = "ColorDef")]
    color: Color,
}

impl Default for ASCII {
    fn default() -> Self {
        ASCII {
            color: Color::Reset,
            path: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Palette {
    // Light color variants
    Light,
    // Dark color variants
    Dark,
    // Entire palette (16-colors)
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(remote = "Color")]
pub enum ColorDef {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerMargin {
    x: u16,
    y: u16,
}

impl Default for InnerMargin {
    fn default() -> Self {
        InnerMargin { x: 1, y: 0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    title: String,
    visible: bool,
    inner_margin: InnerMargin,
}

impl Default for Block {
    fn default() -> Self {
        Block {
            title: String::new(),
            visible: false,
            inner_margin: InnerMargin::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar {
    pub glyph: String,
    pub symbol_open: char,
    pub symbol_close: char,
    pub visible: bool,
}

impl Default for Bar {
    fn default() -> Self {
        Bar {
            glyph: String::new(),
            symbol_open: '(',
            symbol_close: ')',
            visible: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keys {
    pub host: String,
    pub kernel: String,
    pub battery: String,
    pub os: String,
    pub de: String,
    pub wm: String,
    pub distro: String,
    pub terminal: String,
    pub shell: String,
    pub packages: String,
    pub uptime: String,
    pub memory: String,
    pub machine: String,
    pub local_ip: String,
    pub backlight: String,
    pub resolution: String,
    pub cpu_load: String,
    pub cpu: String,
}

impl Default for Keys {
    fn default() -> Self {
        Self {
            host: String::from("Host"),
            kernel: String::from("Kernel"),
            battery: String::from("Battery"),
            os: String::from("OS"),
            de: String::from("DE"),
            wm: String::from("WM"),
            distro: String::from("Distro"),
            terminal: String::from("Terminal"),
            shell: String::from("Shell"),
            packages: String::from("Packages"),
            uptime: String::from("Uptime"),
            memory: String::from("Memory"),
            machine: String::from("Machine"),
            local_ip: String::from("Local IP"),
            backlight: String::from("Brightness"),
            resolution: String::from("Resolution"),
            cpu_load: String::from("CPU Load"),
            cpu: String::from("CPU"),
        }
    }
}
