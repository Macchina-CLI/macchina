extern crate clap;
mod display;
mod extra;
mod format;
mod memory;
mod read;
use clap::{App, Arg};
use display::Options;
use display::{choose_color, Elements};

pub const VERSION: &str = "0.1.1";
pub const DEFAULT_COLOR: colored::Color = colored::Color::Magenta;
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
                .multiple(false)
                .help("Display palette"),
        )
        .arg(
            Arg::with_name("no-color")
                .short("n")
                .long("no-color")
                .takes_value(false)
                .multiple(false)
                .help("Disable colors"),
        )
        .arg(
            Arg::with_name("color")
                .short("c")
                .long("color")
                .takes_value(true)
                .multiple(false)
                .max_values(1)
                .help("Specify the color of the keys"),
        )
        .arg(
            Arg::with_name("random-color")
                .short("r")
                .long("random-color")
                .multiple(false)
                .help("Specify the color of the keys"),
        )
        .arg(
            Arg::with_name("hide")
                .short("H")
                .long("hide")
                .takes_value(true)
                .min_values(1)
                .max_values(10)
                .multiple(false)
                .help("Hide elements such as (host, kern, os, etc.)"),
        )
        .arg(
            Arg::with_name("short-sh")
                .short("s")
                .long("short-sh")
                .takes_value(false)
                .multiple(false)
                .help("Shorten shell value, for example: /usr/bin/zsh -> zsh"),
        )
        .arg(
            Arg::with_name("help")
                .short("h")
                .long("help")
                .takes_value(false)
                .help("Print out helpful information"),
        )
        .arg(
            Arg::with_name("version")
                .short("v")
                .long("version")
                .takes_value(false)
                .help("Print out Macchina's version"),
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
        let color: colored::Color = choose_color(matches.value_of("color").unwrap());
        elems.set_color(color);
    }
    if matches.is_present("random-color") {
        elems.set_color(display::randomize_color());
    }
    display::print_info(elems, opts);
}
