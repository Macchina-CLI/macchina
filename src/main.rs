use std::{env, process::exit};
mod display;
mod read;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let elements: [u32; 9] = [1; 9];
    let (mut color, mut palette_status, mut icons): (bool, bool, bool) = (true, false, false);

    if args.len() == 1 {
        display::show_info(true, false, false, true, &elements);
    } else {
        args.remove(0);
        if args.len() == 1 && (args[0] == "--help".to_string() || args[0] == "-h".to_string()) {
            display::help(true);
            exit(0);
        }
        if args.contains(&ts("--no-color")) {
            color = false;
        }
        if args.contains(&ts("--icons")) {
            icons = true;
        }
        if args.contains(&ts("--palette")) {
            palette_status = true;
        }
        if args.contains(&ts("--hide")) {
            display::hide(color, palette_status, icons, args);
            exit(0);
        }
        display::show_info(color, palette_status, icons, true, &elements);
    }
}

fn ts(argname: &str) -> String {
    return argname.to_string();
}
