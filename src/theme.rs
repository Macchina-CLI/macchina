use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use toml;
use tui::style::Color;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Randomize {
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
struct ASCII {
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

/// This structure defines the skeleton of custom themes which are deserialized from TOML files.
/// See [https://github.com/Macchina-CLI/macchina/blob/main/theme/Carbon.toml](this) for an example
/// theme.
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
    palette: Option<Palette>,
    hide_ascii: bool,
    prefer_small_ascii: bool,
    hide_bar_delimiters: bool,
    pub keys: Keys,
    #[serde(with = "ColorDef")]
    key_color: Color,
    #[serde(with = "ColorDef")]
    separator_color: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            key_color: Color::Blue,
            separator_color: Color::Yellow,
            separator: String::from("-"),
            hide_ascii: false,
            hide_bar_delimiters: false,
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
            hide_bar_delimiters: custom.hide_bar_delimiters,
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

    pub fn get_box_title(&self) -> String {
        self.r#box.title.to_owned()
    }

    pub fn is_box_visible(&self) -> bool {
        self.r#box.visible
    }

    pub fn is_key_color_randomized(&self) -> bool {
        self.randomize.key_color
    }

    pub fn is_separator_color_randomized(&self) -> bool {
        self.randomize.separator_color
    }

    pub fn get_horizontal_margin(&self) -> u16 {
        self.r#box.inner_margin.x
    }

    pub fn get_vertical_margin(&self) -> u16 {
        self.r#box.inner_margin.y
    }

    pub fn prefers_small_ascii(&self) -> bool {
        self.prefer_small_ascii
    }

    pub fn is_ascii_hidden(&self) -> bool {
        self.hide_ascii
    }

    pub fn get_custom_ascii_color(&self) -> Color {
        self.custom_ascii.color
    }

    pub fn get_palette_type(&self) -> Option<&Palette> {
        self.palette.as_ref()
    }

    pub fn using_custom_ascii_color(&self) -> bool {
        if self.custom_ascii.color == Color::Reset {
            return false;
        }

        true
    }

    pub fn get_custom_ascii_path(&self) -> Option<&PathBuf> {
        self.custom_ascii.path.as_ref()
    }

    pub fn is_using_bars(&self) -> bool {
        self.bar.visible
    }

    pub fn are_bar_delimiters_hidden(&self) -> bool {
        self.hide_bar_delimiters
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

    pub fn hide_bar_delimiters(&mut self) {
        self.bar.symbol_open = '\0';
        self.bar.symbol_close = '\0';
    }

    /// Searches for and returns a theme from `~/.config/macchina/themes`
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

    fn _print_theme_test() {
        let cust = Theme {
            bar: Bar::default(),
            separator: String::from("<-->"),
            spacing: 2,
            padding: 0,
            r#box: Block {
                title: String::from("Neon"),
                visible: true,
                inner_margin: InnerMargin::default(),
            },
            randomize: Randomize::default(),
            hide_ascii: false,
            hide_bar_delimiters: false,
            prefer_small_ascii: false,
            palette: Some(Palette::Full),
            key_color: Color::Rgb(10, 33, 51),
            custom_ascii: ASCII::default(),
            separator_color: Color::Indexed(100),
            keys: Keys::default(),
        };

        println!("{}", toml::to_string_pretty(&cust).unwrap());
    }
}
