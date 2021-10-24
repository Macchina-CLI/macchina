use clap::arg_enum;
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
    #[serde(with = "ColorDef")]
    color: Color,

    path: Option<PathBuf>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    title: String,
    visible: bool,
    inner_margin: InnerMargin,
}

impl InnerMargin {
    fn new(a: u16, b: u16) -> Self {
        InnerMargin { x: a, y: b }
    }
}

impl Block {
    fn default() -> Self {
        Block {
            title: String::new(),
            visible: false,
            inner_margin: InnerMargin::new(1, 0),
        }
    }

    fn new(ti: &str, vi: bool) -> Self {
        Block {
            title: ti.to_string(),
            visible: vi,
            inner_margin: InnerMargin::new(1, 0),
        }
    }
}

/// This struct stores the BarStyle to display when --bar or bar config option is used.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarStyle {
    pub glyph: String,
    pub visible: bool,
    pub symbol_open: char,
    pub symbol_close: char,
}

/// This stores predefined `BarStyle` variations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BarStyles {
    Squared,
    Rounded,
    Angled,
    Hidden,
}

impl Default for BarStyle {
    fn default() -> Self {
        BarStyle {
            glyph: String::new(),
            symbol_open: '(',
            symbol_close: ')',
            visible: true,
        }
    }
}

impl BarStyle {
    fn new(style: BarStyles) -> Self {
        match style {
            BarStyles::Squared => BarStyle {
                glyph: "■".to_owned(),
                symbol_open: '[',
                symbol_close: ']',
                visible: true,
            },
            BarStyles::Rounded => BarStyle {
                glyph: "●".to_owned(),
                symbol_open: '(',
                symbol_close: ')',
                visible: true,
            },
            BarStyles::Angled => BarStyle {
                glyph: "×".to_owned(),
                symbol_open: '<',
                symbol_close: '>',
                visible: true,
            },
            BarStyles::Hidden => BarStyle {
                glyph: "\0".to_owned(),
                symbol_open: '\0',
                symbol_close: '\0',
                visible: true,
            },
        }
    }

    pub fn hide_delimiters(&self) -> BarStyle {
        BarStyle {
            glyph: self.glyph.to_owned(),
            symbol_open: '\0',
            symbol_close: '\0',
            visible: true,
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

arg_enum! {
    #[derive(Debug)]
    pub enum Themes {
    Hydrogen,
    Helium,
    Lithium,
    Beryllium,
    Boron,
    }
}

/// This is the struct which defines predefined as well as custom themes.
#[derive(Debug, Clone)]
pub struct Theme {
    bar: BarStyle,
    key_color: Color,
    separator_color: Color,
    separator: String,
    spacing: usize,
    padding: usize,
    hide_ascii: bool,
    prefer_small_ascii: bool,
    randomize: Randomize,
    custom_ascii: ASCII,
    r#box: Block,
    pub keys: Keys,
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            bar: BarStyle::new(BarStyles::Rounded),
            key_color: Color::Red,
            separator_color: Color::White,
            separator: "-".to_owned(),
            spacing: 2,
            padding: 0,
            hide_ascii: true,
            prefer_small_ascii: false,
            r#box: Block::default(),
            custom_ascii: ASCII::default(),
            randomize: Randomize::default(),
            keys: Keys::default(),
        }
    }
}

impl Theme {
    pub fn new(theme: Themes) -> Self {
        match theme {
            Themes::Hydrogen => Theme {
                bar: BarStyle::new(BarStyles::Rounded),
                key_color: Color::Red,
                separator_color: Color::White,
                separator: "-".to_owned(),
                spacing: 2,
                padding: 0,
                hide_ascii: true,
                prefer_small_ascii: false,
                r#box: Block::new(" Hydrogen ", true),
                custom_ascii: ASCII::default(),
                randomize: Randomize::default(),
                keys: Keys::default(),
            },
            Themes::Helium => Theme {
                bar: BarStyle::new(BarStyles::Squared),
                key_color: Color::Green,
                separator_color: Color::White,
                separator: "=>".to_owned(),
                spacing: 2,
                padding: 0,
                hide_ascii: true,
                prefer_small_ascii: false,
                r#box: Block::new(" Helium ", false),
                custom_ascii: ASCII::default(),
                randomize: Randomize::default(),
                keys: Keys::default(),
            },
            Themes::Lithium => Theme {
                bar: BarStyle::new(BarStyles::Angled),
                key_color: Color::Magenta,
                separator_color: Color::White,
                separator: "~".to_owned(),
                spacing: 2,
                padding: 0,
                hide_ascii: true,
                prefer_small_ascii: false,
                r#box: Block::new(" Lithium ", false),
                custom_ascii: ASCII::default(),
                randomize: Randomize::default(),
                keys: Keys::default(),
            },
            Themes::Beryllium => Theme {
                bar: BarStyle::new(BarStyles::Rounded),
                key_color: Color::Yellow,
                separator_color: Color::White,
                separator: "->".to_owned(),
                spacing: 2,
                padding: 0,
                hide_ascii: true,
                prefer_small_ascii: false,
                r#box: Block::new(" Beryllium ", true),
                custom_ascii: ASCII::default(),
                randomize: Randomize::default(),
                keys: Keys::default(),
            },
            Themes::Boron => Theme {
                bar: BarStyle::new(BarStyles::Rounded),
                key_color: Color::Blue,
                separator_color: Color::White,
                separator: "•".to_owned(),
                spacing: 2,
                padding: 0,
                hide_ascii: true,
                prefer_small_ascii: false,
                r#box: Block::new(" Boron ", false),
                custom_ascii: ASCII::default(),
                randomize: Randomize::default(),
                keys: Keys::default(),
            },
        }
    }
    pub fn get_bar_style(&self) -> &BarStyle {
        &self.bar
    }

    pub fn set_bar_style(&mut self, new_bar: BarStyle) {
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

    pub fn get_padding(&self) -> usize {
        self.padding
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

    pub fn using_custom_ascii_color(&self) -> bool {
        if self.custom_ascii.color == Color::Reset {
            return false;
        }

        true
    }

    pub fn get_custom_ascii_path(&self) -> Option<&PathBuf> {
        self.custom_ascii.path.as_ref()
    }

    pub fn using_bars(&self) -> bool {
        self.bar.visible
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
}

impl From<CustomTheme> for Theme {
    fn from(custom: CustomTheme) -> Self {
        Self {
            bar: custom.bar,
            key_color: custom.key_color,
            separator: custom.separator,
            separator_color: custom.separator_color,
            spacing: custom.spacing,
            padding: custom.padding,
            hide_ascii: custom.hide_ascii,
            prefer_small_ascii: custom.prefer_small_ascii,
            r#box: custom.r#box,
            custom_ascii: custom.custom_ascii,
            randomize: custom.randomize,
            keys: custom.keys,
        }
    }
}

/// This structure defines the skeleton of custom themes which are deserialized from TOML files.
/// See [https://github.com/Macchina-CLI/macchina/blob/main/theme/Carbon.toml](this) for an example
/// theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CustomTheme {
    #[serde(with = "ColorDef")]
    key_color: Color,
    #[serde(with = "ColorDef")]
    separator_color: Color,

    custom_ascii: ASCII,
    bar: BarStyle,
    r#box: Block,
    separator: String,
    randomize: Randomize,
    spacing: usize,
    padding: usize,
    hide_ascii: bool,
    prefer_small_ascii: bool,
    keys: Keys,
}

impl Default for CustomTheme {
    fn default() -> Self {
        Self {
            bar: BarStyle::default(),
            key_color: Color::Red,
            separator: "->".to_string(),
            separator_color: Color::White,
            spacing: 0,
            padding: 2,
            hide_ascii: true,
            prefer_small_ascii: false,
            r#box: Block::new("", false),
            custom_ascii: ASCII::default(),
            randomize: Randomize::default(),
            keys: Keys::default(),
        }
    }
}

impl CustomTheme {
    /// Reads custom themes from $XDG_CONFIG_HOME/macchina/themes/{name}.toml
    pub fn get_theme(name: &str) -> Result<Self, std::io::Error> {
        use std::io::Read;
        // check if theme exists in ~/.config/macchina/themes/{name}.toml
        // TODO: look at more data paths?
        let mut theme_path = std::path::PathBuf::new();
        theme_path.push(dirs::config_dir().ok_or_else(|| {
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

    // private function to print a custom theme for testing
    fn __print_theme_test() {
        let cust = CustomTheme {
            bar: BarStyle::new(BarStyles::Squared),
            separator: "=====>".to_string(),
            spacing: 2,
            padding: 0,
            r#box: Block::new("SomeTitle", true),
            randomize: Randomize::default(),
            hide_ascii: false,
            prefer_small_ascii: false,
            key_color: Color::Rgb(10, 33, 51),
            custom_ascii: ASCII::default(),
            separator_color: Color::Indexed(100),
            keys: Keys::default(),
        };
        println!("{}", toml::to_string_pretty(&cust).unwrap());
    }
}
