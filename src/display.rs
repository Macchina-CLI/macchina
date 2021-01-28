extern crate num_cpus;
use crate::memory;
use crate::read;
use crate::{format, VERSION};
use colored::Colorize;
use std::process::exit;

pub struct Options {
    pub color: bool,
    pub palette_status: bool,
    pub shell_shorthand: bool,
}

impl Options {
    pub fn new() -> Options {
        Options {
            color: true,
            palette_status: false,
            shell_shorthand: false,
        }
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
    pub package_count_key: String,
    pub memory_key: String,
    pub num_elements: [bool; 10],
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
            package_count_key: "pkgs".to_string(),
            memory_key: "mem".to_string(),
            num_elements: [true; 10],
        }
    }
}

macro_rules! usage {
    ($i: ident) => {
        let padding: String = " ".repeat($i.left_padding);
        println!(
            "{}{} <{}>",
            padding,
            "USAGE: macchina",
            "OPTIONS".cyan().bold()
        );
        println!("{}{}:", padding, "OPTIONS".cyan().bold());
        println!("{} {}", padding, "-h, --help");
        println!("{} {}", padding, "-p, --palette");
        println!("{} {}", padding, "-n, --no-color");
        println!("{} {}", padding, "-H, --hide (host, os, kern, etc.)");
        println!("{} {}", padding, "-s, --short-sh");
    };
}

/// dsp: display element
macro_rules! dsp {
    ($elem: expr, $pad: ident, $key: expr, $sep: expr, $val: expr) => {
        if $elem {
            println!("{}{}{} {}", $pad, $key, $sep, $val);
        }
    };
}

pub fn print_info(elems: Elements, opts: Options) {
    let padding: String = " ".repeat(elems.left_padding);
    match opts.color {
        true => {
            dsp!(
                elems.num_elements[0],
                padding,
                elems.hostname_key.purple().bold(),
                elems.separator,
                read::hostname()
            );
            dsp!(
                elems.num_elements[1],
                padding,
                elems.os_key.blue().bold(),
                elems.separator,
                read::operating_system()
            );
            dsp!(
                elems.num_elements[2],
                padding,
                elems.kernel_version_key.cyan().bold(),
                elems.separator,
                read::kernel_version()
            );
            dsp!(
                elems.num_elements[3],
                padding,
                elems.package_count_key.green().bold(),
                elems.separator,
                read::package_count()
            );
            dsp!(
                elems.num_elements[4],
                padding,
                elems.shell_key.yellow().bold(),
                elems.separator,
                read::shell(opts.shell_shorthand)
            );
            dsp!(
                elems.num_elements[5],
                padding,
                elems.terminal_key.red().bold(),
                elems.separator,
                read::terminal()
            );
            dsp!(
                elems.num_elements[6],
                padding,
                elems.cpu_key.purple().bold(),
                elems.separator,
                format::cpu(read::cpu_model_name(), num_cpus::get())
            );
            dsp!(
                elems.num_elements[7],
                padding,
                elems.memory_key.blue().bold(),
                elems.separator,
                format::memory(memory::used(), memory::memtotal())
            );
            dsp!(
                elems.num_elements[8],
                padding,
                elems.uptime_key.cyan().bold(),
                elems.separator,
                format::uptime(read::uptime())
            );
            dsp!(
                elems.num_elements[9],
                padding,
                elems.battery_key.green().bold(),
                elems.separator,
                format::battery(read::battery_percentage(), read::battery_status())
            );
        }
        false => {
            dsp!(
                elems.num_elements[0],
                padding,
                elems.hostname_key,
                elems.separator,
                read::hostname()
            );
            dsp!(
                elems.num_elements[1],
                padding,
                elems.os_key,
                elems.separator,
                read::operating_system()
            );
            dsp!(
                elems.num_elements[2],
                padding,
                elems.kernel_version_key,
                elems.separator,
                read::kernel_version()
            );
            dsp!(
                elems.num_elements[3],
                padding,
                elems.package_count_key,
                elems.separator,
                read::package_count()
            );
            dsp!(
                elems.num_elements[4],
                padding,
                elems.shell_key,
                elems.separator,
                read::shell(opts.shell_shorthand)
            );
            dsp!(
                elems.num_elements[5],
                padding,
                elems.terminal_key,
                elems.separator,
                read::terminal()
            );
            dsp!(
                elems.num_elements[6],
                padding,
                elems.cpu_key,
                elems.separator,
                format::cpu(read::cpu_model_name(), num_cpus::get())
            );
            dsp!(
                elems.num_elements[7],
                padding,
                elems.memory_key,
                elems.separator,
                format::memory(memory::used(), memory::memtotal())
            );
            dsp!(
                elems.num_elements[8],
                padding,
                elems.uptime_key,
                elems.separator,
                format::uptime(read::uptime())
            );
            dsp!(
                elems.num_elements[9],
                padding,
                elems.battery_key,
                elems.separator,
                format::battery(read::battery_percentage(), read::battery_status())
            );
        }
    }
    if opts.palette_status {
        palette(elems);
        println!();
    }
}

pub fn palette(elems: Elements) {
    let padding: String = " ".repeat(elems.left_padding);
    println!();
    println!(
        "{}{}{}{}{}{}{}{}{}",
        padding,
        "   ".on_bright_black(),
        "   ".on_bright_red(),
        "   ".on_bright_green(),
        "   ".on_bright_yellow(),
        "   ".on_bright_blue(),
        "   ".on_bright_purple(),
        "   ".on_bright_cyan(),
        "   ".on_bright_white()
    );
}

pub fn hide(mut elems: Elements, options: Options, hide_parameters: Vec<&str>) {
    let mut supplied_wrong_parameter: bool = false;
    let mut inc_params: Vec<&str> = Vec::new();

    //  Labels contains all hideable elements.
    //  The order of each element in the array
    //  is important for the hide functionality
    //  to work properly
    let labels: [&str; 10] = [
        "host", "os", "kern", "pkgs", "sh", "term", "cpu", "mem", "up", "bat",
    ];

    for z in 0..hide_parameters.len() {
        if !labels.contains(&hide_parameters[z]) {
            inc_params.push(&hide_parameters[z].clone());
            supplied_wrong_parameter = true;
        }
    }
    if supplied_wrong_parameter == true {
        hide_error(&inc_params);
        exit(0);
    } else {
        for i in 0..9 {
            if hide_parameters.contains(&labels[i]) {
                elems.num_elements[i] = false;
            }
        }
    }

    print_info(elems, options);
}

pub fn hide_error(inc_params: &Vec<&str>) {
    let elems = Elements::new();
    let padding: String = " ".repeat(elems.left_padding);
    eprintln!(
        "{}{}: {} {:?}",
        padding,
        "Error".red().bold(),
        "bad option",
        inc_params
    );
    println!(
        "{}{} <{}>",
        padding,
        "USAGE: macchina --hide",
        "ELEMENTS".cyan().bold()
    );
    println!("{}{}:", padding, "ELEMENTS".cyan().bold());
    println!("{} -  {}", padding, "host");
    println!("{} -  {}", padding, "os");
    println!("{} -  {}", padding, "kern");
    println!("{} -  {}", padding, "pkgs");
    println!("{} -  {}", padding, "term");
    println!("{} -  {}", padding, "sh");
    println!("{} -  {}", padding, "cpu");
    println!("{} -  {}", padding, "mem");
    println!("{} -  {}", padding, "up");
    println!("{} -  {}", padding, "bat");
}

pub fn help() {
    let elems = Elements::new();
    let padding: String = " ".repeat(elems.left_padding);
    println!("{}{}, v{}", padding, "Macchina".blue().bold(), VERSION);
    usage!(elems);
    println!();
    println!("{}{}", padding, "Battery information might print an error if the file Macchina is trying to read from does not exist.");
    println!("{}{}", padding, "Macchina reads battery information from:");
    println!(
        "{}{}{}",
        padding, padding, "/sys/class/power_supply/BAT0/capacity"
    );
    println!(
        "{}{}{}",
        padding, padding, "/sys/class/power_supply/BAT0/status"
    );
}