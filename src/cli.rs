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

    #[structopt(long = "no-ascii", help = "Removes the ascii art")]
    pub no_ascii: bool,

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
        long = "small-ascii",
        help = "Prefer smaller ASCII variants",
        conflicts_with = "no_ascii"
    )]
    pub small_ascii: bool,

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

            no_ascii: false,

            custom_ascii: None,
            small_ascii: false,

            show: None,
            doctor: false,
            long_uptime: false,
            long_shell: false,
            long_kernel: true,
            current_shell: false,
            interface: None,
            theme: None,
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
