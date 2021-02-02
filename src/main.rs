extern crate clap;
mod display;
mod extra;
mod format;
mod memory;
mod read;
use clap::{App, Arg};
use display::Options;
use display::{choose_color, Elements};

pub const VERSION: &str = "0.1.2";
pub const DEFAULT_COLOR: colored::Color = colored::Color::Magenta;
pub const DEFAULT_SEPARATOR_COLOR: colored::Color = colored::Color::White;
pub const DEFAULT_PADDING: usize = 4;
pub const PATH_TO_BATTERY_PERCENTAGE: &str = "/sys/class/power_supply/BAT0/capacity";
pub const PATH_TO_BATTERY_STATUS: &str = "/sys/class/power_supply/BAT0/status";

fn main() {
    let matches = App::new("Macchina")
        .version("1.0.0")
        .author("grtcdr <ba.tahaaziz@gmail.com>")
        .about("System information fetcher")
        .arg(
            Arg::with_name("palette")
                .short("p")
                .long("palette")
                .takes_value(false)
                .multiple(false),
        )
        .arg(
            Arg::with_name("no-color")
                .short("n")
                .long("no-color")
                .takes_value(false)
                .multiple(false),
        )
        .arg(
            Arg::with_name("color")
                .short("c")
                .long("color")
                .takes_value(true)
                .multiple(false)
                .max_values(1)
                .possible_values(&[
                    "red", "green", "blue", "yellow", "cyan", "magenta", "black", "white",
                ]),
        )
        .arg(
            Arg::with_name("separator-color")
                .short("C")
                .long("separator-color")
                .takes_value(true)
                .multiple(false)
                .max_values(1)
                .possible_values(&[
                    "red", "green", "blue", "yellow", "cyan", "magenta", "black", "white",
                ]),
        )
        .arg(
            Arg::with_name("random-color")
                .short("r")
                .long("random-color")
                .multiple(false),
        )
        .arg(
            Arg::with_name("hide")
                .short("H")
                .long("hide")
                .takes_value(true)
                .min_values(1)
                .max_values(10)
                .multiple(false)
                .possible_values(&[
                    "host", "os", "kern", "pkgs", "sh", "term", "cpu", "mem", "up", "bat",
                ]),
        )
        .arg(
            Arg::with_name("theme")
                .short("t")
                .long("theme")
                .takes_value(true)
                .max_values(1)
                .multiple(false)
                .possible_values(&["def", "alt"]),
        )
        .arg(
            Arg::with_name("short-sh")
                .short("s")
                .long("short-sh")
                .takes_value(false)
                .multiple(false),
        )
        .arg(
            Arg::with_name("help")
                .short("h")
                .long("help")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("version")
                .short("v")
                .long("version")
                .takes_value(false),
        )
        .get_matches();

    // Instantiates Macchina's elements.
    // Contains the key strings to be displayed
    // as well as the separator character and
    // num_elements that allows hiding elements
    let mut elems = Elements::new();

    // Instantiates Macchina's behavior
    // when no arguments are provided.
    let mut opts = Options::new();

    if matches.is_present("help") {
        display::help();
        std::process::exit(0);
    }
    if matches.is_present("palette") {
        opts.palette_status = true;
    }
    if matches.is_present("color") {
        let color: colored::Color = choose_color(matches.value_of("color").unwrap());
        elems.set_color(color);
    }
    if matches.is_present("separator-color") {
        let color: colored::Color = choose_color(matches.value_of("separator-color").unwrap());
        elems.set_separator_color(color);
    }
    if matches.is_present("short-sh") {
        opts.shell_shorthand = true;
    }
    if matches.is_present("no-color") {
        opts.color = false;
    }
    if matches.is_present("hide") {
        let hide_parameters: Vec<&str> = matches.values_of("hide").unwrap().collect();
        display::hide(elems, opts, hide_parameters);
        std::process::exit(0);
    }
    if matches.is_present("version") {
        println!("Macchina v{}", VERSION);
        std::process::exit(0);
    }
    if matches.is_present("random-color") {
        elems.set_color(display::randomize_color());
    }
    if matches.is_present("theme") && matches.value_of("theme").unwrap() == "alt" {
        elems.set_theme_alt();
    }
    display::print_info(elems, opts);
}
