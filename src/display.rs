use colored::Colorize;
use crate::read;
use crate::format;
use std::process::exit;

pub struct Options {
    pub color: bool,
    pub palette_status: bool,
    pub signal: bool,
    pub cpu_shorthand: bool,
    pub shell_shorthand: bool,
}

impl Options {
    pub fn new(col: bool, pal: bool, sig: bool, cpu_short: bool, shell_short: bool) -> Options {
        Options { color: col, palette_status: pal, signal: sig, cpu_shorthand: cpu_short, shell_shorthand: shell_short }
    }
}

pub struct Elements {
    pub separator: char,
    pub left_padding: usize,
    pub hostname_key: String,
    pub os_key: String,
    pub kernel_version_key: String,
    pub terminal_key: String, 
    pub shell_key: String,
    pub cpu_key: String,
    pub uptime_key: String,
    pub battery_key: String,
    pub num_elements: [u32; 9]
}

impl Elements {
    fn new() -> Elements {
        Elements { 
            separator: ':',
            left_padding: 4,
            hostname_key: "host".to_string(),
            os_key: "os".to_string(),
            kernel_version_key: "kern".to_string(),
            terminal_key: "term".to_string(),
            shell_key: "sh".to_string(),
            cpu_key: "cpu".to_string(),
            uptime_key: "up".to_string(),
            battery_key: "bat".to_string(),
            num_elements: [1; 9],
        }
    }
}

pub fn print_info(opts: Options) {
    let elems = Elements::new();
    let padding: String = " ".repeat(elems.left_padding);
    if opts.signal {
        match opts.color {
            true => {
                if elems.num_elements[0] == 1 { println!("{}{}{} {}", padding, elems.hostname_key.purple().bold(), elems.separator, read::read_hostname().expect("Couldn't retrive hostname!")); }
                if elems.num_elements[1] == 1 { println!("{}{}{} {}", padding, elems.os_key.blue().bold(), elems.separator, read::read_operating_system()); }
                if elems.num_elements[2] == 1 { println!("{}{}{} {}", padding, elems.kernel_version_key.cyan().bold(), elems.separator, read::read_kernel_version().expect("Couldn't retrieve kernel version!")); }
                if elems.num_elements[3] == 1 { println!("{}{}{} {}", padding, elems.terminal_key.green().bold(), elems.separator, read::read_terminal()); }
                if elems.num_elements[4] == 1 { println!("{}{}{} {}", padding, elems.shell_key.yellow().bold(), elems.separator, read::read_shell(opts.shell_shorthand)); }
                if elems.num_elements[5] == 1 { println!("{}{}{} {}{}", padding, elems.cpu_key.red().bold(), elems.separator, read::read_cpu_model_name(opts.cpu_shorthand), read::read_cpu_threads()); }
                if elems.num_elements[6] == 1 { println!("{}{}{} {}", padding, elems.uptime_key.purple().bold(), elems.separator, format::format_uptime(read::read_uptime().expect("Couldn't retrieve system uptime!"))); }
                if elems.num_elements[7] == 1 { println!("{}{}{} {}", padding, elems.battery_key.blue().bold(), elems.separator, format::format_battery()); }
            },
            false => {
                if elems.num_elements[0] == 1 { println!("{}{}{} {}", padding, elems.hostname_key, elems.separator, read::read_hostname().expect("Couldn't retrieve hostname!")); }
                if elems.num_elements[1] == 1 { println!("{}{}{} {}", padding, elems.os_key, elems.separator, read::read_operating_system()); }
                if elems.num_elements[2] == 1 { println!("{}{}{} {}", padding, elems.kernel_version_key, elems.separator, read::read_kernel_version().expect("Couldn't retrieve kernel version!")); }
                if elems.num_elements[3] == 1 { println!("{}{}{} {}", padding, elems.terminal_key, elems.separator, read::read_terminal()); }
                if elems.num_elements[4] == 1 { println!("{}{}{} {}", padding, elems.shell_key, elems.separator, read::read_shell(opts.shell_shorthand)); }
                if elems.num_elements[5] == 1 { println!("{}{}{} {}{}", padding, elems.cpu_key, elems.separator, read::read_cpu_model_name(opts.cpu_shorthand), read::read_cpu_threads()); }
                if elems.num_elements[6] == 1 { println!("{}{}{} {}", padding, elems.uptime_key, elems.separator, format::format_uptime(read::read_uptime().expect("Couldn't retrieve system uptime!"))); }
                if elems.num_elements[7] == 1 { println!("{}{}{} {}", padding, elems.battery_key, elems.separator, format::format_battery()); }
            }
        }
        if opts.palette_status {
            palette(elems);
            println!();
        }
    }
}
    
pub fn hide(options: Options, hide_parameters: Vec<String>) {
        let mut elements: [u32; 9] = [1;9];
        let mut supplied_wrong_parameter: bool = false;
        let mut inc_params: Vec<String> = Vec::new();
        //  labels contains all hideable elements
        let labels: [String; 9] = [
            "host".to_string(),
            "os".to_string(),
            "kern".to_string(),
            "term".to_string(),
            "sh".to_string(),
            "cpu".to_string(),
            "up".to_string(),
            "bat".to_string(),
            "palette".to_string()
        ];
        for z in 0 .. hide_parameters.len() {
            if labels.contains(&hide_parameters[z]) {
                elements[z] = 0;
            }
            else {
                inc_params.push(hide_parameters[z].clone());
                supplied_wrong_parameter = true;
            }
        }
        if supplied_wrong_parameter == true {
            hide_error(&inc_params);
            exit(0);
        }
        print_info(options);
}
    
pub fn help(opts: Options) {
    let elems = Elements::new();
    let padding: String = " ".repeat(elems.left_padding);
    match opts.color {
        true => {
            println!("{}{}:", padding, "Macchina".blue().bold());
            println!("{}{}", padding, "Usage: macchina [options]");
            println!("{}{}", padding, "Options:");
            println!("{}{}", padding, "--help");
            println!("{}{}", padding, "--palette");
            println!("{}{}", padding, "--no-color");
            println!("{}{}", padding, "--hide (host, os, kern, etc.)");
            println!("{}{}", padding, "--short-sh");
            println!("{}{}", padding, "--short-cpu");
        },
        false => {
            println!("{}{}", padding, "Macchina");
            println!("{}{}", padding, "Usage: macchina [options]");
            println!("{}{}", padding, "Options:");
            println!("{}{}", padding, "--help");
            println!("{}{}", padding, "--palette");
            println!("{}{}", padding, "--no-color");
            println!("{}{}", padding, "--hide (host, os, kern, etc.)");
            println!("{}{}", padding, "--short-sh");
            println!("{}{}", padding, "--short-cpu");
        }
    }
}

pub fn palette(elems: Elements) {
    let padding: String = " ".repeat(elems.left_padding);
    println!();
    println!("{}{}{}{}{}{}{}{}{}",
             padding,
             "   ".on_bright_black(),
             "   ".on_bright_red(),
             "   ".on_bright_green(),
             "   ".on_bright_yellow(),
             "   ".on_bright_blue(),
             "   ".on_bright_purple(),
             "   ".on_bright_cyan(),
             "   ".on_bright_white());
}

pub fn error(inc_args: &Vec<String>) {
    let elems = Elements::new();
    let padding: String = " ".repeat(elems.left_padding);
    eprintln!("{}{}: {} {:?}", padding, "Error".red().bold(), "bad option", inc_args);
    println!("{}{}", padding, "Usage: macchina [options]");
    println!("{}{}", padding, "Options:");
    println!("{}{}", padding, "--help");
    println!("{}{}", padding, "--palette");
    println!("{}{}", padding, "--no-color");
    println!("{}{}", padding, "--hide (host, os, kern, etc.)");
    println!("{}{}", padding, "--short-sh");
    println!("{}{}", padding, "--short-cpu");
}

pub fn hide_error(inc_params: &Vec<String>) {
    let elems = Elements::new();
    let padding: String = " ".repeat(elems.left_padding);
    eprintln!("{}{}: {} {:?}", padding, "Error".red().bold(), "bad option", inc_params);
    println!("{}{}", padding, "Usage: macchina --hide [elements]");
    println!("{}{}", padding, "Elements:");
    println!("{}{}", padding, "host");
    println!("{}{}", padding, "os");
    println!("{}{}", padding, "kern");
    println!("{}{}", padding, "term");
    println!("{}{}", padding, "sh");
    println!("{}{}", padding, "cpu");
    println!("{}{}", padding, "up");
    println!("{}{}", padding, "bat");
}