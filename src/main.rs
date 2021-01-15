use std::{env, process::exit};
mod display;
use display::Options;
mod read;
mod extra;
mod format;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let elements: [u32; 9] = [1; 9];
    let mut options= Options::new(true, false, false, true, false, false);
    if args.len() == 1 {
        display::print(options, &elements);
    } else {
        args.remove(0);
        args.sort();
        if args.len() == 1 && (args[0] == "--help".to_string() || args[0] == "-h".to_string()) {
            display::help(true);
            exit(0);
        }
        if args.contains(&"--no-color".to_string()) {
            options.color = false;
        }
        if args.contains(&"--icons".to_string()) {
            options.icons = true;
        }
        if args.contains(&"--palette".to_string()) {
            options.palette_status = true;
        }
        if args.contains(&"--short-cpu".to_string()) {
            options.cpu_shorthand = true;
        }
        if args.contains(&"--short-sh".to_string()) {
            options.shell_shorthand = true;
        }
        if args.contains(&"--hide".to_string()) {
            display::hide(options, args);
            exit(0);
        }
        display::print(options, &elements);
    }
}