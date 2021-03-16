mod bars;
mod display;
mod format;
mod theme;

use clap::{crate_authors, crate_version, App, Arg};
use colored::Color;
use display::{choose_color, Elements, Fail, Options};
use macchina_read::{extra, Readouts};
use theme::Theme;

#[macro_use]
extern crate lazy_static;

use macchina_read::traits::*;

/// Macchina's version
pub const VERSION: &str = crate_version!();
/// Macchina's default color
pub const DEFAULT_COLOR: Color = Color::Blue;
/// Macchina's default separator color
pub const DEFAULT_SEPARATOR_COLOR: Color = Color::White;
/// Macchina's default padding value
pub const DEFAULT_PADDING: usize = 4;

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

fn main() {
    let matches = App::new("Macchina")
        .version(VERSION)
        .author(crate_authors!())
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
            Arg::with_name("random-sep-color")
                .short("R")
                .long("random-sep-color")
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
                .possible_values(&["H", "He", "Li"]),
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
                .long("short-uptime")
                .multiple(false),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .multiple(false),
        )
        .arg(Arg::with_name("help").short("h").long("help"))
        .arg(Arg::with_name("version").short("v").long("version"))
        .get_matches();

    // Instantiate Macchina's elements.
    let mut elems = Elements::new();
    let mut fail = Fail::new();
    elems.set_theme(theme::HydrogenTheme::new(), &mut fail);

    if !matches.is_present("theme") {
        elems.theme.misc_mut().longest_key = elems.longest_key(&mut fail);
    }

    // Instantiate Macchina's default behavior, i.e:
    //   color: enabled
    //   palette: disabled
    //   shell shorthand: disabled
    //   uptime shorthand: disabled
    let mut opts = Options::new();

    if matches.is_present("palette") {
        opts.palette_status = true;
    }
    if matches.is_present("padding") {
        elems.theme.misc_mut().padding = matches.value_of("padding").unwrap().parse().unwrap();
    }
    if matches.is_present("spacing") {
        elems.theme.misc_mut().spacing = matches.value_of("spacing").unwrap().parse().unwrap();
    }
    if matches.is_present("color") {
        let color: Color = choose_color(matches.value_of("color").unwrap());
        elems.theme.misc_mut().color = color;
    }
    if matches.is_present("separator-color") {
        let color: Color = choose_color(matches.value_of("separator-color").unwrap());
        elems.theme.misc_mut().separator_color = color;
    }
    if matches.is_present("short-shell") {
        opts.shell_shorthand = true;
    }
    if matches.is_present("short-uptime") {
        opts.uptime_shorthand = true;
    }
    if matches.is_present("no-color") {
        let mut misc = elems.theme.misc_mut();
        misc.color = Color::White;
        misc.separator_color = Color::White;
    }
    if matches.is_present("bar") {
        opts.bar_status = true;
    }
    if matches.is_present("hide") {
        let elements_to_hide: Vec<&str> = matches.values_of("hide").unwrap().collect();
        display::hide(elems, opts, &mut fail, elements_to_hide);
        std::process::exit(0);
    }
    if matches.is_present("show-only") {
        elems.hide_all();
        let elements_to_unhide: Vec<&str> = matches.values_of("show-only").unwrap().collect();
        display::unhide(elems, opts, &mut fail, elements_to_unhide);
        std::process::exit(0);
    }
    if matches.is_present("debug") {
        elems.init_elements_for_debug(&mut fail, &opts);
        display::debug(&mut fail);
        std::process::exit(0);
    }
    if matches.is_present("help") {
        display::help();
        std::process::exit(0);
    }
    if matches.is_present("version") {
        println!("Macchina v{}", VERSION);
        std::process::exit(0);
    }
    if matches.is_present("random-color") {
        elems.theme.misc_mut().color = display::randomize_color();
    }
    if matches.is_present("random-sep-color") {
        elems.theme.misc_mut().separator_color = display::randomize_color();
    }
    if matches.is_present("theme") {
        if matches.value_of("theme").unwrap() == "He" {
            elems.set_theme(theme::HeliumTheme::new(), &mut fail)
        } else if matches.value_of("theme").unwrap() == "Li" {
            elems.set_theme(theme::LithiumTheme::new(), &mut fail)
        }
    }

    display::print_info(elems, &opts, &mut fail);
}
