extern crate clap;
mod bars;
mod display;
mod extra;
mod format;
mod memory;
mod read;
use clap::{App, Arg};
use display::Options;
use display::{choose_color, Elements};

pub const VERSION: &str = "0.1.9";
pub const DEFAULT_COLOR: colored::Color = colored::Color::Magenta;
pub const DEFAULT_SEPARATOR_COLOR: colored::Color = colored::Color::White;
pub const DEFAULT_PADDING: usize = 4;
pub const PATH_TO_BATTERY_PERCENTAGE: &str = "/sys/class/power_supply/BAT0/capacity";
pub const PATH_TO_BATTERY_STATUS: &str = "/sys/class/power_supply/BAT0/status";

fn main() {
    let matches = App::new("Macchina")
        .version(VERSION)
        .author("grtcdr <ba.tahaaziz@gmail.com>")
        .about("System information fetcher")
        .arg(
            Arg::with_name("palette")
                .short("p")
                .long("palette")
                .multiple(false),
        )
        .arg(
            Arg::with_name("padding")
                .short("-P")
                .validator(extra::is_int)
                .long("padding")
                .takes_value(true)
                .multiple(false),
        )
        .arg(
            Arg::with_name("no-color")
                .short("n")
                .long("no-color")
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
        .arg(Arg::with_name("bar").short("b").long("bar").multiple(false))
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
                .max_values(11)
                .multiple(false)
                .possible_values(&[
                    "host", "mach", "os", "kern", "pkgs", "sh", "term", "cpu", "up", "mem", "bat",
                ]),
        )
        .arg(
            Arg::with_name("theme")
                .short("t")
                .long("theme")
                .takes_value(true)
                .max_values(1)
                .multiple(false)
                .possible_values(&["def", "alt", "giraffe"]),
        )
        .arg(
            Arg::with_name("short-sh")
                .short("s")
                .long("short-sh")
                .multiple(false),
        )
        .arg(Arg::with_name("help").short("h").long("help"))
        .arg(Arg::with_name("version").short("v").long("version"))
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
    if matches.is_present("padding") {
        elems.set_left_padding_to(matches.value_of("padding").unwrap().parse().unwrap());
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
    if matches.is_present("bar") {
        elems.enable_bar();
    }
    if matches.is_present("hide") {
        let elements_to_hide: Vec<&str> = matches.values_of("hide").unwrap().collect();
        display::hide(elems, opts, elements_to_hide);
        std::process::exit(0);
    }
    if matches.is_present("version") {
        println!("Macchina v{}", VERSION);
        std::process::exit(0);
    }
    if matches.is_present("random-color") {
        elems.set_color(display::randomize_color());
    }
    if matches.is_present("theme") {
        if matches.value_of("theme").unwrap() == "alt" {
            elems.set_theme_alt();
        } else if matches.value_of("theme").unwrap() == "giraffe" {
            elems.set_theme_giraffe();
        }
    }

    display::print_info(elems, opts);
}