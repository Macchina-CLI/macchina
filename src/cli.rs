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

#[derive(StructOpt, Debug, Serialize, Deserialize)]
#[structopt(author = AUTHORS, about = ABOUT)]
#[serde(default, deny_unknown_fields)]
pub struct Opt {
    #[structopt(short = "p", long = "palette", help = "Displays color palette")]
    pub palette: bool,

    #[structopt(
        short = "P",
        long = "padding",
        help = "Specify the amount of left padding to use (when the box is hidden)"
    )]
    pub padding: Option<usize>,

    #[structopt(
        short = "s",
        long = "spacing",
        help = "Specify the amount of spacing between to use"
    )]
    pub spacing: Option<usize>,

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
    short = "c",
    long = "color",
    possible_values = & MacchinaColor::variants(),
    case_insensitive = true,
    help = "Specify the key color",
    conflicts_with = "no_color",
    )]
    pub color: Option<MacchinaColor>,

    #[structopt(
        short = "b",
        long = "bar",
        help = "Displays bars instead of numerical values"
    )]
    pub bar: bool,

    #[structopt(
    short = "C",
    long = "separator-color",
    possible_values = & MacchinaColor::variants(),
    case_insensitive = true,
    help = "Specify the separator color",
    conflicts_with = "no_color",
    )]
    pub separator_color: Option<MacchinaColor>,

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
    short = "H",
    long = "hide",
    possible_values = & data::ReadoutKey::variants(),
    case_insensitive = true,
    help = "Hides the specified elements",
    min_values = 1,
    conflicts_with = "show_only"
    )]
    pub hide: Option<Vec<data::ReadoutKey>>,

    #[structopt(
    short = "X",
    long = "show-only",
    possible_values = & data::ReadoutKey::variants(),
    case_insensitive = true,
    help = "Displays only the specified elements",
    min_values = 1,
    conflicts_with = "hide"
    )]
    pub show_only: Option<Vec<data::ReadoutKey>>,

    #[structopt(short = "d", long = "doctor", help = "Checks the system for failures")]
    #[serde(skip_serializing, skip_deserializing)]
    pub doctor: bool,

    #[structopt(short = "U", long = "short-uptime", help = "Shortens uptime output")]
    pub short_uptime: bool,

    #[structopt(short = "S", long = "short-shell", help = "Shortens shell output")]
    pub short_shell: bool,

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
        long = "box-title",
        help = "Overrides the title of the box",
        conflicts_with = "no_box"
    )]
    pub box_title: Option<String>,

    #[structopt(
        long = "custom-ascii",
        help = "Specify your own ASCII art from a file",
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
    pub list_themes: bool,
}

impl Default for Opt {
    fn default() -> Self {
        Opt {
            palette: false,
            padding: None,
            spacing: None,

            no_color: false,
            no_separator: false,
            no_bar_delimiter: false,
            no_title: false,
            no_ascii: false,
            no_box: false,

            color: None,
            bar: false,

            separator_color: None,
            random_color: false,
            random_sep_color: false,

            hide: None,
            show_only: None,

            doctor: false,

            short_uptime: false,
            short_shell: false,

            theme: None,

            box_title: None,

            custom_ascii: None,
            custom_ascii_color: None,
            small_ascii: false,

            box_inner_margin_x: 1,
            box_inner_margin_y: 0,
            export_config: false,
            list_themes: false,
        }
    }
}

#[allow(dead_code)]
pub fn build_cli() -> App<'static, 'static> {
    Opt::clap()
}
