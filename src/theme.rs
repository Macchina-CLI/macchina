use clap::arg_enum;
use serde::{Deserialize, Serialize};
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
}

impl Default for ASCII {
    fn default() -> Self {
        ASCII {
            color: Color::Reset,
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

/// This stores the predefined BarStyle's
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BarStyles {
    Squared,
    Rounded,
    Angled,
    Hidden,
    Custom(BarStyle),
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
            BarStyles::Custom(barstyle) => barstyle,
        }
    }

    pub fn hide_delimiters(&self) -> BarStyle {
        BarStyle {
            glyph: self.glyph.clone(),
            symbol_open: '\0',
            symbol_close: '\0',
            visible: true,
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
    color: Color,
    separator_color: Color,
    separator: String,
    spacing: usize,
    padding: usize,
    randomize: Randomize,
    custom_ascii: ASCII,
    r#box: Block,
    pub keys: Keys,
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            bar: BarStyle::new(BarStyles::Rounded),
            color: Color::Red,
            separator_color: Color::White,
            separator: "-".to_owned(),
            spacing: 2,
            padding: 0,
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
                color: Color::Red,
                separator_color: Color::White,
                separator: "-".to_owned(),
                spacing: 2,
                padding: 0,
                r#box: Block::new(" Hydrogen ", true),
                custom_ascii: ASCII::default(),
                randomize: Randomize::default(),
                keys: Keys::default(),
            },
            Themes::Helium => Theme {
                bar: BarStyle::new(BarStyles::Squared),
                color: Color::Green,
                separator_color: Color::White,
                separator: "=>".to_owned(),
                spacing: 2,
                padding: 0,
                r#box: Block::new(" Helium ", false),
                custom_ascii: ASCII::default(),
                randomize: Randomize::default(),
                keys: Keys::default(),
            },
            Themes::Lithium => Theme {
                bar: BarStyle::new(BarStyles::Angled),
                color: Color::Magenta,
                separator_color: Color::White,
                separator: "~".to_owned(),
                spacing: 2,
                padding: 0,
                r#box: Block::new(" Lithium ", false),
                custom_ascii: ASCII::default(),
                randomize: Randomize::default(),
                keys: Keys::default(),
            },
            Themes::Beryllium => Theme {
                bar: BarStyle::new(BarStyles::Rounded),
                color: Color::Yellow,
                separator_color: Color::White,
                separator: "->".to_owned(),
                spacing: 2,
                padding: 0,
                r#box: Block::new(" Beryllium ", true),
                custom_ascii: ASCII::default(),
                randomize: Randomize::default(),
                keys: Keys::default(),
            },
            Themes::Boron => Theme {
                bar: BarStyle::new(BarStyles::Rounded),
                color: Color::Blue,
                separator_color: Color::White,
                separator: "•".to_owned(),
                spacing: 2,
                padding: 0,
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

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color
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

    pub fn get_custom_ascii_color(&self) -> Color {
        self.custom_ascii.color
    }

    pub fn using_custom_ascii_color(&self) -> bool {
        if self.custom_ascii.color == Color::Reset {
            return false; 
        }

        true
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

impl From<CustomTheme> for Theme {
    fn from(custom: CustomTheme) -> Self {
        Self {
            bar: BarStyle::new(custom.bar),
            color: custom.color,
            separator: custom.separator,
            separator_color: custom.separator_color,
            spacing: custom.spacing,
            padding: custom.padding,
            r#box: custom.r#box,
            custom_ascii: custom.custom_ascii,
            randomize: custom.randomize,
            keys: custom.keys,
        }
    }
}

/// This is the struct which stores the CustomThemes which is serialized from a JSON file.  See
/// [https://github.com/Macchina-CLI/macchina/blob/main/theme/Carbon.json](this) for an example
/// theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CustomTheme {
    #[serde(with = "ColorDef")]
    color: Color,
    #[serde(with = "ColorDef")]
    separator_color: Color,

    custom_ascii: ASCII,
    bar: BarStyles,
    r#box: Block,
    separator: String,
    randomize: Randomize,
    spacing: usize,
    padding: usize,
    keys: Keys,
}

impl Default for CustomTheme {
    fn default() -> Self {
        Self {
            bar: BarStyles::Squared,
            color: Color::Red,
            separator: "->".to_string(),
            separator_color: Color::White,
            spacing: 0,
            padding: 2,
            r#box: Block::new("", false),
            custom_ascii: ASCII::default(),
            randomize: Randomize::default(),
            keys: Keys::default(),
        }
    }
}

impl CustomTheme {
    /// Get custom theme from ~/.local/share/macchina/themes/{name}.json
    /// Check the repo for example themes
    pub fn get_theme(name: &str) -> Result<Self, std::io::Error> {
        use std::io::Read;
        // check if the name exists in ~/.local/share/macchina/themes/{name}.json
        // need to add other data paths later ( /usr/share/macchina/themes/{name}.json )
        let mut theme_path = std::path::PathBuf::new();
        theme_path.push(dirs::data_local_dir().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "data_local_dir (e.g. ~/.local/share) not found",
            )
        })?);
        theme_path.push(std::path::Path::new(&format!(
            "macchina/themes/{}.json",
            name
        )));

        let mut buffer: Vec<u8> = Vec::new();
        let mut theme = std::fs::File::open(theme_path)?;
        theme.read_to_end(&mut buffer)?;

        serde_json::from_slice(&buffer).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Unable to parse theme")
        })
    }

    // private function to print a custom theme for testing
    fn __print_theme_test() {
        let cust = CustomTheme {
            bar: BarStyles::Custom(BarStyle {
                glyph: "x".to_string(),
                symbol_open: '[',
                symbol_close: ']',
                visible: true,
            }),
            separator: "=====>".to_string(),
            spacing: 2,
            padding: 0,
            r#box: Block::new("SomeTitle", true),
            randomize: Randomize::default(),

            color: Color::Rgb(10, 33, 51),
            custom_ascii: ASCII::default(),
            separator_color: Color::Indexed(100),
            keys: Keys::default(),
        };
        println!("{}", serde_json::to_string_pretty(&cust).unwrap());
    }
}
