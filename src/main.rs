mod bars;
#[path = "read/battery.rs"]
mod battery;
mod display;
mod extra;
mod format;
#[path = "read/general.rs"]
mod general;
#[path = "read/kernel.rs"]
mod kernel;
#[path = "read/memory.rs"]
mod memory;
#[path = "read/package.rs"]
mod package;
#[path = "read/product.rs"]
mod product;
use clap::{crate_authors, crate_version, App, Arg};
use colored::Color;
use display::{choose_color, Elements, Options};

#[cfg(target_os = "netbsd")]
pub const HIDE_DISTRIBUTION: bool = true;
/// Macchina's version
pub const VERSION: &str = crate_version!();
/// Macchina's default color
pub const DEFAULT_COLOR: Color = Color::Blue;
/// Macchina's default separator color
pub const DEFAULT_SEPARATOR_COLOR: Color = Color::White;
/// Macchina's default padding value
pub const DEFAULT_PADDING: usize = 4;
/// The path that Macchina reads battery percentage from
pub const PATH_TO_BATTERY_PERCENTAGE: &str = "/sys/class/power_supply/BAT0/capacity";
/// The path that Macchina reads battery status from
pub const PATH_TO_BATTERY_STATUS: &str = "/sys/class/power_supply/BAT0/status";

fn main() {
    let matches = App::new("Macchina")
        .version(VERSION)
        .author(crate_authors!())
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
            Arg::with_name("spacing")
                .short("-s")
                .validator(extra::is_int)
                .long("spacing")
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
                .max_values(12)
                .multiple(false)
                .possible_values(&[
                    "host", "mach", "distro", "de", "wm", "kernel", "pkgs", "shell", "term", "cpu",
                    "up", "mem", "bat",
                ]),
        )
        .arg(
            Arg::with_name("show-only")
                .short("X")
                .long("show-only")
                .takes_value(true)
                .min_values(1)
                .max_values(12)
                .multiple(false)
                .possible_values(&[
                    "host", "mach", "distro", "de", "wm", "kernel", "pkgs", "shell", "term", "cpu",
                    "up", "mem", "bat",
                ]),
        )
        .arg(
            Arg::with_name("theme")
                .short("t")
                .long("theme")
                .takes_value(true)
                .max_values(1)
                .multiple(false)
                .possible_values(&["def", "alt", "long"]),
        )
        .arg(
            Arg::with_name("short-shell")
                .short("S")
                .long("short-shell")
                .multiple(false),
        )
        .arg(
            Arg::with_name("short-uptime")
                .short("U")
                .long("short-shell")
                .multiple(false),
        )
        .arg(Arg::with_name("help").short("h").long("help"))
        .arg(Arg::with_name("version").short("v").long("version"))
        .get_matches();

    // Instantiate Macchina's elements.
    let mut elems = Elements::new();
    elems.is_running_wm_only();
    // longest_key() is used to determine how to
    // automatically space the keys, separator and values
    elems.set_longest_key();

    // Instantiate Macchina's default behavior, i.e:
    //   color: enabled
    //   palette: disabled
    //   shell shorthand: disabled
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
    if matches.is_present("spacing") {
        elems.set_spacing(matches.value_of("spacing").unwrap().parse().unwrap());
    }
    if matches.is_present("color") {
        let color: Color = choose_color(matches.value_of("color").unwrap());
        elems.set_color(color);
    }
    if matches.is_present("separator-color") {
        let color: Color = choose_color(matches.value_of("separator-color").unwrap());
        elems.set_separator_color(color);
    }
    if matches.is_present("short-shell") {
        opts.shell_shorthand = true;
    }
    if matches.is_present("short-uptime") {
        opts.uptime_shorthand = true;
    }
    if matches.is_present("no-color") {
        elems.set_color(Color::White);
        elems.set_separator_color(Color::White);
    }
    if matches.is_present("bar") {
        elems.enable_bar();
    }
    if matches.is_present("hide") {
        let elements_to_hide: Vec<&str> = matches.values_of("hide").unwrap().collect();
        display::hide(elems, opts, elements_to_hide);
        std::process::exit(0);
    }
    if matches.is_present("show-only") {
        elems.hide_all();
        let elements_to_unhide: Vec<&str> = matches.values_of("show-only").unwrap().collect();
        display::unhide(elems, opts, elements_to_unhide);
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
        } else if matches.value_of("theme").unwrap() == "long" {
            elems.set_theme_long();
        }
    }

    display::print_info(elems, &opts);
}
