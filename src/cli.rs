use crate::data;
use clap::{arg_enum, App};
use serde::{Deserialize, Serialize};
use std::default::Default;
use structopt::StructOpt;
use tui::style::Color;
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

arg_enum! {
    #[derive(Debug,Serialize, Deserialize)]
    pub enum MacchinaColor {
        Red,
        Green,
        Blue,
        Yellow,
        Cyan,
        Magenta,
        Black,
        White
    }
}

impl MacchinaColor {
    /// Convert the argument passed to a color flag to its respective color.
    pub fn get_color(&self) -> Color {
        match self {
            MacchinaColor::Red => Color::Red,
            MacchinaColor::Green => Color::Green,
            MacchinaColor::Blue => Color::Blue,
            MacchinaColor::Yellow => Color::Yellow,
            MacchinaColor::Cyan => Color::Cyan,
            MacchinaColor::Magenta => Color::Magenta,
            MacchinaColor::Black => Color::Black,
            MacchinaColor::White => Color::White,
        }
    }
}

arg_enum! {
    #[derive(Debug,Serialize, Deserialize)]
    pub enum PaletteType {
        Light,
        Dark,
        Full
    }
}

#[derive(StructOpt, Debug, Serialize, Deserialize)]
// #[structopt(version = concat!(env!("CARGO_PKG_VERSION"), " (", env!("VERGEN_GIT_SHA_SHORT"), ")", libmacchina::version()))]
// #[structopt(version = concat!(env!("CARGO_PKG_VERSION"), " (", env!("VERGEN_GIT_SHA_SHORT"), ")"))]
#[structopt(author = AUTHORS, about = ABOUT)]
#[serde(default, deny_unknown_fields)]
pub struct Opt {
    #[structopt(
        short = "p",
        long = "palette",
        possible_values = & PaletteType::variants(),
        case_insensitive = true,
        help = "Displays color palette",
    )]
    pub palette: Option<PaletteType>,

    #[structopt(
        short = "V",
        long = "version",
        help = "Prints version information",
        conflicts_with = "doctor"
    )]
    #[serde(skip_serializing, skip_deserializing)]
    pub version: bool,

    #[structopt(short = "n", long = "no-color", help = "Disables color")]
    pub no_color: bool,

    #[structopt(
        short = "K",
        long = "no-separator",
        help = "Hides the separator",
        conflicts_with = "separator_color"
    )]
    pub no_separator: bool,

    #[structopt(
        short = "D",
        long = "no-bar-delimiter",
        help = "Hides the bar's delimiters"
    )]
    pub no_bar_delimiter: bool,

    #[structopt(
        long = "no-title",
        help = "Hides the box title",
        conflicts_with = "box_title"
    )]
    pub no_title: bool,

    #[structopt(long = "no-ascii", help = "Removes the ascii art")]
    pub no_ascii: bool,

    #[structopt(
        long = "no-box",
        help = "Removes the box surrounding system information"
    )]
    pub no_box: bool,

    #[structopt(
        short = "b",
        long = "bar",
        help = "Displays bars instead of numerical values"
    )]
    pub bar: bool,

    #[structopt(
        short = "r",
        long = "random-color",
        help = "Picks a random key color for you"
    )]
    pub random_color: bool,

    #[structopt(
        short = "R",
        long = "random-sep-color",
        help = "Picks a random separator color for you"
    )]
    pub random_sep_color: bool,

    #[structopt(
    short = "X",
    long = "show-only",
    possible_values = & data::ReadoutKey::variants(),
    case_insensitive = true,
    help = "Displays only the specified readouts",
    min_values = 1,
    )]
    pub show: Option<Vec<data::ReadoutKey>>,

    #[structopt(short = "d", long = "doctor", help = "Checks the system for failures")]
    #[serde(skip_serializing, skip_deserializing)]
    pub doctor: bool,

    #[structopt(short = "U", long = "short-uptime", help = "Lengthens uptime output")]
    pub long_uptime: bool,

    #[structopt(short = "S", long = "long-shell", help = "Lengthens shell output")]
    pub long_shell: bool,

    #[structopt(short = "k", long = "long-kernel", help = "Lengthens kernel output")]
    pub long_kernel: bool,

    #[structopt(
        short = "W",
        long = "current-shell",
        help = "Toggles between the current shell or the default one"
    )]
    pub current_shell: bool,

    #[structopt(
    short = "t",
    long = "theme",
    // default_value = "Hydrogen",
    // possible_values = & theme::Themes::variants(),
    case_insensitive = true,
    help = "Specify the theme"
    )]
    pub theme: Option<String>,

    #[structopt(
        long = "custom-ascii",
        help = "Specify your own ASCII art from a text file (supports ANSI escape codes)",
        conflicts_with = "no_ascii"
    )]
    pub custom_ascii: Option<String>,

    #[structopt(
        short = "A",
        long = "custom-ascii-color",
        help = "Overrides all colors in the ASCII art with a specified one",
        requires = "custom-ascii",
        conflicts_with = "no_ascii"
    )]
    pub custom_ascii_color: Option<MacchinaColor>,

    #[structopt(
        long = "small-ascii",
        help = "Prefer smaller ASCII variants",
        conflicts_with = "no_ascii"
    )]
    pub small_ascii: bool,

    #[structopt(
        help = "Specify the horizontal inner margin value of the box",
        long = "box-inner-margin-x",
        short = "L",
        conflicts_with = "no_box",
        default_value = "1"
    )]
    pub box_inner_margin_x: u16,

    #[structopt(
        help = "Specify the vertical inner margin value of the box",
        long = "box-inner-margin-y",
        short = "J",
        conflicts_with = "no_box",
        default_value = "0"
    )]
    pub box_inner_margin_y: u16,

    #[structopt(
        long = "export-config",
        help = "Prints the config file to stdout",
        conflicts_with = "doctor"
    )]
    #[serde(skip_serializing, skip_deserializing)]
    pub export_config: bool,

    #[structopt(
        long = "list-themes",
        help = "Lists all available themes: built-in and custom"
    )]
    #[serde(skip_serializing, skip_deserializing)]
    pub list_themes: bool,

    #[structopt(
        long = "config",
        help = "Specify the config file",
        conflicts_with = "export_config"
    )]
    #[serde(skip_serializing, skip_deserializing)]
    pub config: Option<std::path::PathBuf>,

    #[structopt(
        long = "interface",
        short = "i",
        help = "Specify the network interface"
    )]
    pub interface: Option<String>,
}

impl Default for Opt {
    fn default() -> Self {
        Opt {
            palette: None,
            version: false,

            no_color: false,
            no_separator: false,
            no_bar_delimiter: false,
            no_title: false,
            no_ascii: false,
            no_box: false,

            random_color: false,
            random_sep_color: false,

            custom_ascii: None,
            custom_ascii_color: None,
            small_ascii: false,

            bar: false,
            show: None,
            doctor: false,
            long_uptime: false,
            long_shell: false,
            long_kernel: true,
            current_shell: false,
            interface: None,
            theme: None,
            box_inner_margin_x: 1,
            box_inner_margin_y: 0,
            export_config: false,
            list_themes: false,
            config: None,
        }
    }
}

#[allow(dead_code)]
pub fn build_cli() -> App<'static, 'static> {
    Opt::clap()
}
