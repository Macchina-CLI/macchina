extern crate clap;
mod display;
mod extra;
mod format;
mod memory;
mod read;
use clap::{App, Arg, SubCommand};
use display::Elements;
use display::Options;
use std::process::exit;

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
            Arg::with_name("hide")
                .short("H")
                .long("hide")
                .takes_value(true)
                .max_values(10)
                .min_values(1)
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
                .multiple(false)
                .help("Print out helpful information"),
        )
        .arg(
            Arg::with_name("version")
                .short("v")
                .long("version")
                .takes_value(false)
                .multiple(false)
                .help("Print out Macchina's version"),
        )
        .get_matches();

    // Instanties Macchina's elements.
    // Contains the key strings to be displayed
    // as well as the separator character and
    // num_elements that allows hiding elements
    let elems = Elements::new();

    // Instantiates Macchina's behavior
    // when no arguments are provided.
    let mut opts = Options::new();

    if matches.is_present("help") {
        display::help();
        exit(0);
    }
    if matches.is_present("palette") {
        opts.palette_status = true;
    }
    if matches.is_present("no-color") {
        opts.color = false;
    }
    if matches.is_present("short-sh") {
        opts.shell_shorthand = true;
    }

    display::print_info(elems, opts);
}

pub const VERSION: &str = "1.0.0";
