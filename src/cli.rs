use crate::{data, theme};
use clap::{arg_enum, App};
use structopt::StructOpt;
use tui::style::Color;
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

arg_enum! {
    #[derive(Debug)]
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

#[derive(StructOpt, Debug)]
#[structopt(author = AUTHORS, about = ABOUT)]
pub struct Opt {
    #[structopt(short = "p", long = "palette", help = "Displays color palette")]
    pub palette: bool,

    #[structopt(
        short = "P",
        long = "padding",
        help = "Specify the amount of left padding to use"
    )]
    pub padding: Option<usize>,

    #[structopt(
        short = "s",
        long = "spacing",
        help = "Specify the amount of spacing to use"
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
    pub doctor: bool,

    #[structopt(short = "U", long = "short-uptime", help = "Shortens uptime output")]
    pub short_uptime: bool,

    #[structopt(short = "S", long = "short-shell", help = "Shortens shell output")]
    pub short_shell: bool,

    #[structopt(
    short = "t",
    long = "theme",
    default_value = "Hydrogen",
    possible_values = & theme::Themes::variants(),
    case_insensitive = true,
    help = "Specify the theme"
    )]
    pub theme: theme::Themes,

    #[structopt(long = "no-ascii", help = "Removes the ascii art")]
    pub no_ascii: bool,

    #[structopt(
        long = "custom-ascii",
        help = "Specify your own ascii art from a file",
        conflicts_with = "no_ascii"
    )]
    pub custom_ascii: Option<String>,

    #[structopt(
        long = "custom-ascii-color",
        help = "Overrides all colors in the ascii art with a specified one",
        requires = "custom-ascii",
        conflicts_with = "no_ascii"
    )]
    pub custom_ascii_color: Option<MacchinaColor>,

    #[structopt(
        long = "no-box",
        help = "Removes the box surrounding system information"
    )]
    pub no_box: bool,

    #[structopt(
        long = "box-title",
        help = "Overrides the title of the box",
        conflicts_with = "no_box"
    )]
    pub box_title: Option<String>,

    #[structopt(
        long = "no-title",
        help = "Hides the box title",
        conflicts_with = "box_title"
    )]
    pub no_title: bool,
}

#[allow(dead_code)]
pub fn build_cli() -> App<'static, 'static> {
    Opt::clap()
}
