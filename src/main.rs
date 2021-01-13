use std::env;
mod display;
mod read;

fn main() {
let args: Vec<String> = env::args().collect();
let elements: [u32; 9] = [1;9];

    /*match args.len() {
        1 => {
            display::show_info(true, false, &elements);
        },
        2 => {
            if args.contains(&"--help".to_string()) {
                display::help(true);
            }
            else if args.contains(&"--no-color".to_string()) {
                display::show_info(false, false, &elements); 
            }
        },
        _ => {
            else if args.contains(&"--palette".to_string()) {
                display::show_info(true, true, &elements);
            }
            else if args.contains(&"--hide".to_string()) {
                display::hide(true, true, args);
            }
            else {
                display::error(true, args);
            }
        }
    }*/

    if args.len() == 1 { 
        display::show_info(true, false, &elements) 
    }
    else if args.len() == 2 { 
        if args.contains(&"--help".to_string()) {
            display::help(true);
        }
        else if args.contains(&"--no-color".to_string()) {
            display::show_info(false, false, &elements); 
        }
        else if args.contains(&"--palette".to_string()) {
            display::show_info(true, true, &elements);
        }
        else
        {
            display::error(true, args); 
        }
    }
    else {
        if args.contains(&"--help".to_string()) && args.contains(&"--no-color".to_string()) {
                display::help(false);
        }
        if args.contains(&"--hide".to_string()) {
            display::hide(true, true, args);
        }
        else {
            display::error(true, args);
        }
    }
}