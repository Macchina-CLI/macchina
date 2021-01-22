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
    pub num_elements: [bool; 9]
}

impl Elements {
    pub fn new() -> Elements {
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
            num_elements: [true; 9],
        }
    }
}

pub fn print_info(elems: Elements, opts: Options) {
    let padding: String = " ".repeat(elems.left_padding);
    if opts.signal {
        match opts.color {
            true => {
                if elems.num_elements[0] { println!("{}{}{} {}", padding, elems.hostname_key.purple().bold(), elems.separator, read::read_hostname().expect("Couldn't retrive hostname!")); }
                if elems.num_elements[1] { println!("{}{}{} {}", padding, elems.os_key.blue().bold(), elems.separator, read::read_operating_system()); }
                if elems.num_elements[2] { println!("{}{}{} {}", padding, elems.kernel_version_key.cyan().bold(), elems.separator, read::read_kernel_version().expect("Couldn't retrieve kernel version!")); }
                if elems.num_elements[3] { println!("{}{}{} {}", padding, elems.terminal_key.green().bold(), elems.separator, read::read_terminal()); }
                if elems.num_elements[4] { println!("{}{}{} {}", padding, elems.shell_key.yellow().bold(), elems.separator, read::read_shell(opts.shell_shorthand)); }
                if elems.num_elements[5] { println!("{}{}{} {}{}", padding, elems.cpu_key.red().bold(), elems.separator, read::read_cpu_model_name(opts.cpu_shorthand), read::read_cpu_threads()); }
                if elems.num_elements[6] { println!("{}{}{} {}", padding, elems.uptime_key.purple().bold(), elems.separator, format::format_uptime(read::read_uptime().expect("Couldn't retrieve system uptime!"))); }
                if elems.num_elements[7] { println!("{}{}{} {}", padding, elems.battery_key.blue().bold(), elems.separator, format::format_battery(read::read_battery_percentage(), read::read_battery_status())); }
            },
            false => {
                if elems.num_elements[0] { println!("{}{}{} {}", padding, elems.hostname_key, elems.separator, read::read_hostname().expect("Couldn't retrieve hostname!")); }
                if elems.num_elements[1] { println!("{}{}{} {}", padding, elems.os_key, elems.separator, read::read_operating_system()); }
                if elems.num_elements[2] { println!("{}{}{} {}", padding, elems.kernel_version_key, elems.separator, read::read_kernel_version().expect("Couldn't retrieve kernel version!")); }
                if elems.num_elements[3] { println!("{}{}{} {}", padding, elems.terminal_key, elems.separator, read::read_terminal()); }
                if elems.num_elements[4] { println!("{}{}{} {}", padding, elems.shell_key, elems.separator, read::read_shell(opts.shell_shorthand)); }
                if elems.num_elements[5] { println!("{}{}{} {}{}", padding, elems.cpu_key, elems.separator, read::read_cpu_model_name(opts.cpu_shorthand), read::read_cpu_threads()); }
                if elems.num_elements[6] { println!("{}{}{} {}", padding, elems.uptime_key, elems.separator, format::format_uptime(read::read_uptime().expect("Couldn't retrieve system uptime!"))); }
                if elems.num_elements[7] { println!("{}{}{} {}", padding, elems.battery_key, elems.separator, format::format_battery(read::read_battery_percentage(), read::read_battery_status())); }
            }
        }
        if opts.palette_status {
            palette(elems);
            println!();
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
    
pub fn hide(mut elems: Elements, options: Options, hide_parameters: Vec<String>) {
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
    if hide_parameters.len() > 0 {
        for z in 0 .. hide_parameters.len() {
            if !labels.contains(&hide_parameters[z]){
                inc_params.push(hide_parameters[z].clone());
                supplied_wrong_parameter = true;
            }
        }
        if supplied_wrong_parameter == true {
            hide_error(&inc_params);
            exit(0);
        }
        else {
            for i in 0 .. 9 {
                if hide_parameters.contains(&labels[i]) {
                    elems.num_elements[i] = false;
                }
            }
        }
    }
    else {
        hide_error(&inc_params);
        exit(0);
    }
    print_info(elems, options);
}

pub fn hide_error(inc_params: &Vec<String>) {
    let elems = Elements::new();
    let padding: String = " ".repeat(elems.left_padding);
    eprintln!("{}{}: {} {:?}", padding, "Error".red().bold(), "bad option", inc_params);
    println!("{}{} [{}]", padding, "Usage: macchina --hide", "elements".yellow().bold());
    println!("{}{}:", padding, "Elements".yellow().bold());
    println!("{}{}", padding, "host");
    println!("{}{}", padding, "os");
    println!("{}{}", padding, "kern");
    println!("{}{}", padding, "term");
    println!("{}{}", padding, "sh");
    println!("{}{}", padding, "cpu");
    println!("{}{}", padding, "up");
    println!("{}{}", padding, "bat");
}

fn usage(elems: Elements) {
    let padding: String = " ".repeat(elems.left_padding);
    println!("{}{} [{}]", padding, "Usage: macchina", "options".yellow().bold());
    println!("{}{}:", padding, "Options".yellow().bold());
    println!("{}{}", padding, "--help");
    println!("{}{}", padding, "--palette");
    println!("{}{}", padding, "--no-color");
    println!("{}{}", padding, "--hide (host, os, kern, etc.)");
    println!("{}{}", padding, "--short-sh");
    println!("{}{}", padding, "--short-cpu");
}
    
pub fn help(opts: Options) {
    let elems = Elements::new();
    let padding: String = " ".repeat(elems.left_padding);
    match opts.color {
        true => {
            println!("{}{}:", padding, "Macchina".blue().bold());
            usage(elems);
        },
        false => {
            println!("{}{}", padding, "Macchina");
            usage(elems);
        }
    }
}

pub fn error(inc_args: &Vec<String>) {
    let elems = Elements::new();
    let padding: String = " ".repeat(elems.left_padding);
    eprintln!("{}{}: {} {:?}", padding, "Error".red().bold(), "bad option", inc_args);
    usage(elems);
}