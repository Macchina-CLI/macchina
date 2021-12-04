use crate::theme::borders::Border;
use crate::theme::color::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tui::style::Color;
use tui::widgets::BorderType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Palette {
    r#type: Option<PaletteType>,
    glyph: Option<String>,
    visible: Option<bool>,
}

impl Default for Palette {
    fn default() -> Self {
        Palette {
            r#type: Some(PaletteType::Dark),
            glyph: Some(String::from("   ")),
            visible: None,
        }
    }
}

impl Palette {
    pub fn get_type(&self) -> PaletteType {
        if let Some(t) = &self.r#type {
            return t.to_owned();
        }

        PaletteType::Dark
    }

    pub fn get_glyph(&self) -> Option<&String> {
        self.glyph.as_ref()
    }

    pub fn is_visible(&self) -> bool {
        if let Some(v) = self.visible {
            return v;
        }

        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Randomize {
    key_color: Option<bool>,
    separator_color: Option<bool>,
    pool: Option<ColorTypes>,
}

impl Default for Randomize {
    fn default() -> Self {
        Randomize {
            key_color: None,
            separator_color: None,
            pool: Some(ColorTypes::Base),
        }
    }
}

impl Randomize {
    pub fn is_key_color_randomized(&self) -> bool {
        if let Some(k) = self.key_color {
            return k;
        }

        false
    }

    pub fn is_separator_color_randomized(&self) -> bool {
        if let Some(s) = self.separator_color {
            return s;
        }

        false
    }

    pub fn get_pool(&self) -> Color {
        if let Some(pool) = &self.pool {
            match pool {
                ColorTypes::Base => return make_random_color(),
                ColorTypes::Indexed => {
                    let mut rng = rand::thread_rng();
                    return Color::Indexed(rng.gen_range(0..=127));
                }
                ColorTypes::Hexadecimal => {
                    let mut rng = rand::thread_rng();
                    let rgb = (
                        rng.gen_range(0..=255),
                        rng.gen_range(0..=255),
                        rng.gen_range(0..=255),
                    );
                    return Color::Rgb(rgb.0, rgb.1, rgb.2);
                }
            };

        }

        make_random_color()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ASCII {
    path: Option<PathBuf>,

    #[serde(default)]
    #[serde(with = "color_to_tui::optional")]
    color: Option<Color>,
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
    visible: Option<bool>,
    inner_margin: Option<InnerMargin>,
    border: Option<Border>,
}

impl Default for Block {
    fn default() -> Self {
        Block {
            title: None,
            visible: Some(false),
            inner_margin: Some(InnerMargin::default()),
            border: Some(Border::Plain),
        }
    }
}

impl Block {
    pub fn get_title(&self) -> String {
        if let Some(t) = &self.title {
            return t.to_owned();
        }

        String::new()
    }

    pub fn get_border_type(&self) -> BorderType {
        if let Some(b) = &self.border {
            match b {
                Border::Plain => return BorderType::Plain,
                Border::Rounded => return BorderType::Rounded,
                Border::Double => return BorderType::Double,
                Border::Thick => return BorderType::Thick,
            };
        }

        BorderType::Plain
    }

    pub fn is_visible(&self) -> bool {
        if let Some(v) = self.visible {
            return v;
        }

        false
    }

    pub fn get_horizontal_margin(&self) -> u16 {
        if let Some(marg) = &self.inner_margin {
            return marg.x;
        }

        1
    }

    pub fn get_vertical_margin(&self) -> u16 {
        if let Some(marg) = &self.inner_margin {
            return marg.y;
        }

        0
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
    pub host: Option<String>,
    pub kernel: Option<String>,
    pub battery: Option<String>,
    pub os: Option<String>,
    pub de: Option<String>,
    pub wm: Option<String>,
    pub distro: Option<String>,
    pub terminal: Option<String>,
    pub shell: Option<String>,
    pub packages: Option<String>,
    pub uptime: Option<String>,
    pub memory: Option<String>,
    pub machine: Option<String>,
    pub local_ip: Option<String>,
    pub backlight: Option<String>,
    pub resolution: Option<String>,
    pub cpu_load: Option<String>,
    pub cpu: Option<String>,
}

impl Default for Keys {
    fn default() -> Self {
        Self {
            host: Some(String::from("Host")),
            kernel: Some(String::from("Kernel")),
            battery: Some(String::from("Battery")),
            os: Some(String::from("OS")),
            de: Some(String::from("DE")),
            wm: Some(String::from("WM")),
            distro: Some(String::from("Distro")),
            terminal: Some(String::from("Terminal")),
            shell: Some(String::from("Shell")),
            packages: Some(String::from("Packages")),
            uptime: Some(String::from("Uptime")),
            memory: Some(String::from("Memory")),
            machine: Some(String::from("Machine")),
            local_ip: Some(String::from("Local IP")),
            backlight: Some(String::from("Brightness")),
            resolution: Some(String::from("Resolution")),
            cpu_load: Some(String::from("CPU Load")),
            cpu: Some(String::from("CPU")),
        }
    }
}

impl Keys {
    pub fn get_host(&self) -> String {
        if let Some(h) = &self.host {
            return h.to_owned();
        }

        String::from("Host")
    }

    pub fn get_kernel(&self) -> String {
        if let Some(h) = &self.kernel {
            return h.to_owned();
        }

        String::from("Kernel")
    }

    pub fn get_battery(&self) -> String {
        if let Some(h) = &self.battery {
            return h.to_owned();
        }

        String::from("Battery")
    }

    pub fn get_os(&self) -> String {
        if let Some(h) = &self.os {
            return h.to_owned();
        }

        String::from("OS")
    }

    pub fn get_de(&self) -> String {
        if let Some(h) = &self.de {
            return h.to_owned();
        }

        String::from("DE")
    }

    pub fn get_wm(&self) -> String {
        if let Some(h) = &self.wm {
            return h.to_owned();
        }

        String::from("WM")
    }

    pub fn get_distro(&self) -> String {
        if let Some(h) = &self.distro {
            return h.to_owned();
        }

        String::from("Distro")
    }

    pub fn get_terminal(&self) -> String {
        if let Some(h) = &self.terminal {
            return h.to_owned();
        }

        String::from("Terminal")
    }

    pub fn get_shell(&self) -> String {
        if let Some(h) = &self.shell {
            return h.to_owned();
        }

        String::from("Shell")
    }

    pub fn get_packages(&self) -> String {
        if let Some(h) = &self.packages {
            return h.to_owned();
        }

        String::from("Packages")
    }

    pub fn get_uptime(&self) -> String {
        if let Some(h) = &self.uptime {
            return h.to_owned();
        }

        String::from("Uptime")
    }

    pub fn get_memory(&self) -> String {
        if let Some(h) = &self.memory {
            return h.to_owned();
        }

        String::from("Memory")
    }

    pub fn get_machine(&self) -> String {
        if let Some(h) = &self.machine {
            return h.to_owned();
        }

        String::from("Machine")
    }

    pub fn get_local_ip(&self) -> String {
        if let Some(h) = &self.local_ip {
            return h.to_owned();
        }

        String::from("Local IP")
    }

    pub fn get_backlight(&self) -> String {
        if let Some(h) = &self.backlight {
            return h.to_owned();
        }

        String::from("Brightness")
    }

    pub fn get_resolution(&self) -> String {
        if let Some(h) = &self.resolution {
            return h.to_owned();
        }

        String::from("Resolution")
    }

    pub fn get_cpu_load(&self) -> String {
        if let Some(h) = &self.cpu_load {
            return h.to_owned();
        }

        String::from("CPU Load")
    }

    pub fn get_cpu(&self) -> String {
        if let Some(h) = &self.cpu {
            return h.to_owned();
        }

        String::from("CPU")
    }
}
