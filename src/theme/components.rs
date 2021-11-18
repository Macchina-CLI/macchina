use crate::theme::borders::Border;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tui::style::Color;
use tui::widgets::BorderType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Palette {
    r#type: Option<PaletteType>,
    glyph: Option<String>,
}

impl Default for Palette {
    fn default() -> Self {
        Palette {
            r#type: None,
            glyph: Some(String::from("   ")),
        }
    }
}

impl Palette {
    pub fn get_type(&self) -> Option<&PaletteType> {
        self.r#type.as_ref()
    }

    pub fn get_glyph(&self) -> Option<&String> {
        self.glyph.as_ref()
    }
}

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

impl Randomize {
    pub fn is_key_color_randomized(&self) -> bool {
        self.key_color
    }

    pub fn is_separator_color_randomized(&self) -> bool {
        self.separator_color
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ASCII {
    path: Option<PathBuf>,

    #[serde(default)]
    #[serde(with = "color_to_tui::optional")]
    color: Option<Color>,
}

impl Default for ASCII {
    fn default() -> Self {
        ASCII {
            color: None,
            path: None,
        }
    }
}

impl ASCII {
    pub fn get_color(&self) -> Option<Color> {
        if let Some(col) = &self.color {
            return Some(*col);
        }

        None
    }

    pub fn get_path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaletteType {
    Light,
    Dark,
    Full,
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
    title: Option<String>,
    visible: bool,
    inner_margin: InnerMargin,
    border: Border,
}

impl Default for Block {
    fn default() -> Self {
        Block {
            title: None,
            visible: false,
            inner_margin: InnerMargin::default(),
            border: Border::Plain,
        }
    }
}

impl Block {
    pub fn get_title(&self) -> Option<String> {
        if let Some(t) = &self.title {
            return Some(t.to_owned());
        }

        None
    }

    pub fn get_border_type(&self) -> BorderType {
        match self.border {
            Border::Plain => BorderType::Plain,
            Border::Rounded => BorderType::Rounded,
            Border::Double => BorderType::Double,
            Border::Thick => BorderType::Thick,
        }
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn get_horizontal_margin(&self) -> u16 {
        self.inner_margin.x
    }

    pub fn get_vertical_margin(&self) -> u16 {
        self.inner_margin.y
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar {
    glyph: String,
    symbol_open: char,
    symbol_close: char,
    hide_delimiters: bool,
    visible: bool,
}

impl Default for Bar {
    fn default() -> Self {
        Bar {
            glyph: String::new(),
            symbol_open: '(',
            symbol_close: ')',
            hide_delimiters: false,
            visible: false,
        }
    }
}

impl Bar {
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn get_glyph(&self) -> &str {
        &self.glyph
    }

    pub fn get_symbol_open(&self) -> char {
        self.symbol_open
    }

    pub fn get_symbol_close(&self) -> char {
        self.symbol_close
    }

    pub fn hide_delimiters(&mut self) {
        self.symbol_open = '\0';
        self.symbol_close = '\0';
    }

    pub fn are_delimiters_hidden(&self) -> bool {
        self.hide_delimiters
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

impl Keys {
    pub fn get_host(&self) -> &String {
        &self.host
    }

    pub fn get_kernel(&self) -> &String {
        &self.kernel
    }

    pub fn get_battery(&self) -> &String {
        &self.battery
    }

    pub fn get_os(&self) -> &String {
        &self.os
    }

    pub fn get_de(&self) -> &String {
        &self.de
    }

    pub fn get_wm(&self) -> &String {
        &self.wm
    }

    pub fn get_distro(&self) -> &String {
        &self.distro
    }

    pub fn get_terminal(&self) -> &String {
        &self.terminal
    }

    pub fn get_shell(&self) -> &String {
        &self.shell
    }

    pub fn get_packages(&self) -> &String {
        &self.packages
    }

    pub fn get_uptime(&self) -> &String {
        &self.uptime
    }

    pub fn get_memory(&self) -> &String {
        &self.memory
    }

    pub fn get_machine(&self) -> &String {
        &self.machine
    }

    pub fn get_local_ip(&self) -> &String {
        &self.local_ip
    }

    pub fn get_backlight(&self) -> &String {
        &self.backlight
    }

    pub fn get_resolution(&self) -> &String {
        &self.resolution
    }

    pub fn get_cpu_load(&self) -> &String {
        &self.cpu_load
    }

    pub fn get_cpu(&self) -> &String {
        &self.cpu
    }
}
