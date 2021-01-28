mod display;
mod extra;
mod format;
mod memory;
mod read;
use display::Elements;
use display::Options;
use std::{env, process::exit};

//  main.rs will soon be reworked to use clap because the current
//  method of parsing is not optimal whatsoever

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut inc_args: Vec<String> = Vec::new();
    let mut supplied_wrong_arg: bool = false;
    let elems = Elements::new();
    let mut opts = Options::new(true, false, true, false, false);
    let allowed_args: [String; 6] = [
        "--help".to_string(),
        "--palette".to_string(),
        "--no-color".to_string(),
        "--short-cpu".to_string(),
        "--short-sh".to_string(),
        "--hide".to_string(),
    ];

    args.remove(0);
    args.sort();
    for z in 0..args.len() {
        if allowed_args.contains(&args[z]) {
            if args.len() == 1 && args[0] == "--help".to_string() {
                display::help();
                exit(0);
            }
            if args.contains(&"--no-color".to_string()) {
                opts.color = false;
            }
            if args.contains(&"--palette".to_string()) {
                opts.palette_status = true;
            }
            if args.contains(&"--short-cpu".to_string()) {
                opts.cpu_shorthand = true;
            }
            if args.contains(&"--short-sh".to_string()) {
                opts.shell_shorthand = true;
            }
            if args.contains(&"--hide".to_string()) {
                let mut params: Vec<String> = Vec::new();
                let args_copy = args;
                for i in 0..args_copy.len() {
                    if !args_copy[i].starts_with('-') {
                        params.push(args_copy[i].clone());
                    }
                }
                display::hide(elems, opts, params);
                exit(0);
            }
        } else {
            inc_args.push(args[z].clone());
            supplied_wrong_arg = true;
        }
    }
    if supplied_wrong_arg {
        display::error(&inc_args);
        exit(0);
    } else {
        display::print_info(elems, opts);
    }
}
