mod bars;
mod display;
mod format;
mod theme;

use clap::arg_enum;
use clap::crate_authors;
use colored::Color;
use display::{Elements, Fail};
use macchina_read::Readouts;
use structopt::StructOpt;

#[macro_use]
extern crate lazy_static;

use macchina_read::traits::*;

pub const AUTHORS: &str = crate_authors!();
pub const ABOUT: &str = "System information fetcher";

lazy_static! {
    pub(crate) static ref READOUTS: Readouts = Readouts {
        battery: macchina_read::BatteryReadout::new(),
        kernel: macchina_read::KernelReadout::new(),
        memory: macchina_read::MemoryReadout::new(),
        general: macchina_read::GeneralReadout::new(),
        product: macchina_read::ProductReadout::new(),
        packages: macchina_read::PackageReadout::new()
    };
}

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
    /// Convert arguments passed to `--color` to their respective color.
    fn get_color(&self) -> Color {
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
    palette: bool,

    #[structopt(
        short = "P",
        long = "padding",
        default_value = "4",
        help = "Specifies the amount of left padding to use"
    )]
    padding: usize,

    #[structopt(
        short = "s",
        long = "spacing",
        help = "Specifies the amount of spacing to use"
    )]
    spacing: Option<usize>,

    #[structopt(short = "n", long = "no-color", help = "Disables color")]
    no_color: bool,

    #[structopt(
        short = "c",
        long = "color",
        possible_values = &MacchinaColor::variants(),
        case_insensitive = true,
        default_value = "Blue",
        help = "Specifies the key color"
    )]
    color: MacchinaColor,

    #[structopt(
        short = "b",
        long = "bar",
        help = "Displays bars instead of numerical values"
    )]
    bar: bool,

    #[structopt(
        short = "C",
        long = "separator-color",
        possible_values = &MacchinaColor::variants(),
        case_insensitive = true,
        default_value = "White",
        help = "Specifies the separator color"
    )]
    separator_color: MacchinaColor,

    #[structopt(
        short = "r",
        long = "random-color",
        help = "Picks a random key color for you"
    )]
    random_color: bool,

    #[structopt(
        short = "R",
        long = "random-sep-color",
        help = "Picks a random separator color for you"
    )]
    random_sep_color: bool,

    #[structopt(
        short = "H",
        long = "hide",
        possible_values = &theme::ReadoutKey::variants(),
        case_insensitive = true,
        help = "Hides the specified elements"
    )]
    hide: Option<Vec<theme::ReadoutKey>>,

    #[structopt(
        short = "X",
        long = "show-only",
        possible_values = &theme::ReadoutKey::variants(),
        case_insensitive = true,
        help = " Displays only the specified elements"
    )]
    show_only: Option<Vec<theme::ReadoutKey>>,

    #[structopt(short = "d", long = "debug", help = "Prints debug information")]
    debug: bool,

    #[structopt(short = "U", long = "short-uptime", help = "Shortens uptime output")]
    short_uptime: bool,

    #[structopt(short = "S", long = "short-shell", help = "Shortens shell output")]
    short_shell: bool,

    #[structopt(
        short = "t",
        long = "theme",
        default_value = "Hydrogen",
        possible_values = &theme::Themes::variants(),
        help = "Specifies the theme to use"
    )]
    theme: theme::Themes,
}

fn main() {
    let opt = Opt::from_args();

    // Instantiate Macchina's elements.
    let mut elems = Elements::new();
    let mut fail = Fail::new();
    elems.set_theme(opt.theme.create_instance(), &mut fail);

    let longest_key = elems.longest_key(&mut fail);
    let mut misc = elems.theme.misc_mut();

    misc.longest_key = longest_key;
    misc.padding = opt.padding;
    misc.color = opt.color.get_color();
    misc.separator_color = opt.separator_color.get_color();

    if let Some(spacing) = opt.spacing {
        misc.spacing = spacing;
    }

    if opt.no_color {
        misc.color = Color::White;
        misc.separator_color = Color::White;
    }

    if let Some(ref elements_to_hide) = opt.hide {
        display::hide(elems, &opt, &mut fail, elements_to_hide);
        std::process::exit(0); //todo: refactor, don't make display::hide() also print_info...
    }

    if let Some(ref show_only) = opt.show_only {
        elems.hide_all();
        display::unhide(elems, &opt, &mut fail, show_only);
        std::process::exit(0); //todo: refactor, don't make display::unhide() also print_info...
    }

    if opt.debug {
        elems.init_elements_for_debug(&mut fail, &opt);
        display::debug(&mut fail);
        std::process::exit(0);
    }

    if opt.random_color {
        misc.color = display::randomize_color();
    }

    if opt.random_sep_color {
        misc.separator_color = display::randomize_color();
    }

    display::print_info(elems, &opt, &mut fail);
}
