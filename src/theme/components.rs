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

        false
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
    pub fn rand_key(&self) -> bool {
        if let Some(k) = self.key_color {
            return k;
        }

        false
    }

    pub fn rand_sep(&self) -> bool {
        if let Some(s) = self.separator_color {
            return s;
        }

        false
    }

    pub fn generate(&self) -> Color {
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
        self.color
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
    glyph: Option<String>,
    symbol_open: Option<char>,
    symbol_close: Option<char>,
    hide_delimiters: Option<bool>,
    visible: Option<bool>,
}

impl Default for Bar {
    fn default() -> Self {
        Bar {
            glyph: None,
            symbol_open: Some('('),
            symbol_close: Some(')'),
            hide_delimiters: None,
            visible: None,
        }
    }
}

impl Bar {
    pub fn is_visible(&self) -> bool {
        if let Some(v) = self.visible {
            return v;
        }

        false
    }

    pub fn get_glyph(&self) -> &str {
        if let Some(g) = &self.glyph {
            return g;
        }

        "●"
    }

    pub fn get_symbol_open(&self) -> char {
        if let Some(s) = self.symbol_open {
            return s;
        }

        '('
    }

    pub fn get_symbol_close(&self) -> char {
        if let Some(s) = self.symbol_close {
            return s;
        }

        ')'
    }

    pub fn hide_delimiters(&mut self) {
        if let Some(h) = self.hide_delimiters {
            if h {
                self.symbol_open = Some('\0');
                self.symbol_close = Some('\0');
            }
        }
    }

    pub fn are_delimiters_hidden(&self) -> bool {
        if let Some(h) = self.hide_delimiters {
            return h;
        }

        false
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
    pub gpu: Option<String>,
    pub disk_space: Option<String>,
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
            gpu: Some(String::from("GPU")),
            disk_space: Some(String::from("Disk Space")),
        }
    }
}

impl Keys {
    pub fn get_host(&self) -> &str {
        if let Some(h) = &self.host {
            return h;
        }

        "Host"
    }

    pub fn get_kernel(&self) -> &str {
        if let Some(k) = &self.kernel {
            return k;
        }

        "Kernel"
    }

    pub fn get_battery(&self) -> &str {
        if let Some(b) = &self.battery {
            return b;
        }

        "Battery"
    }

    pub fn get_os(&self) -> &str {
        if let Some(o) = &self.os {
            return o;
        }

        "OS"
    }

    pub fn get_de(&self) -> &str {
        if let Some(d) = &self.de {
            return d;
        }

        "DE"
    }

    pub fn get_wm(&self) -> &str {
        if let Some(w) = &self.wm {
            return w;
        }

        "WM"
    }

    pub fn get_distro(&self) -> &str {
        if let Some(d) = &self.distro {
            return d;
        }

        "Distro"
    }

    pub fn get_terminal(&self) -> &str {
        if let Some(t) = &self.terminal {
            return t;
        }

        "Terminal"
    }

    pub fn get_shell(&self) -> &str {
        if let Some(s) = &self.shell {
            return s;
        }

        "Shell"
    }

    pub fn get_packages(&self) -> &str {
        if let Some(p) = &self.packages {
            return p;
        }

        "Packages"
    }

    pub fn get_uptime(&self) -> &str {
        if let Some(u) = &self.uptime {
            return u;
        }

        "Uptime"
    }

    pub fn get_memory(&self) -> &str {
        if let Some(m) = &self.memory {
            return m;
        }

        "Memory"
    }

    pub fn get_machine(&self) -> &str {
        if let Some(m) = &self.machine {
            return m;
        }

        "Machine"
    }

    pub fn get_local_ip(&self) -> &str {
        if let Some(l) = &self.local_ip {
            return l;
        }

        "Local IP"
    }

    pub fn get_backlight(&self) -> &str {
        if let Some(b) = &self.backlight {
            return b;
        }

        "Brightness"
    }

    pub fn get_resolution(&self) -> &str {
        if let Some(r) = &self.resolution {
            return r;
        }

        "Resolution"
    }

    pub fn get_cpu_load(&self) -> &str {
        if let Some(c) = &self.cpu_load {
            return c;
        }

        "CPU Load"
    }

    pub fn get_cpu(&self) -> &str {
        if let Some(c) = &self.cpu {
            return c;
        }

        "CPU"
    }

    pub fn get_gpu(&self) -> &str {
        if let Some(m) = &self.gpu {
            return m;
        }

        "GPU"
    }

    pub fn get_disk_space(&self) -> &str {
        if let Some(d) = &self.disk_space {
            return d;
        }

        "Disk Space"
    }
}
