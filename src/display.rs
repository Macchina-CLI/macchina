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
pub struct Pair {
    key: String,
    value: String,
}

impl Pair {
    pub fn new(k: String, v: String) -> Pair {
        Pair { key: k, value: v }
    }
    fn modify(&mut self, val: String) {
        self.value = val;
    }
}

pub struct Elements {
    separator: char,
    left_padding: usize,
    hostname: Pair,
    os: Pair,
    kernel: Pair,
    packages: Pair,
    shell: Pair,
    terminal: Pair,
    cpu: Pair,
    memory: Pair,
    uptime: Pair,
    battery: Pair,
    num_elements: [bool; 10],
}

impl Elements {
    pub fn new() -> Elements {
        Elements {
            separator: ':',
            left_padding: 4,
            hostname: Pair::new(String::from("host"), read::hostname()),
            os: Pair::new(String::from("os"), read::operating_system()),
            kernel: Pair::new(String::from("kern"), read::kernel_version()),
            packages: Pair::new(String::from("pkg"), read::package_count().to_string()),
            shell: Pair::new(String::from("sh"), String::new()),
            terminal: Pair::new(String::from("term"), read::terminal()),
            cpu: Pair::new(
                String::from("cpu"),
                format::cpu(read::cpu_model_name(), num_cpus::get()),
            ),
            memory: Pair::new(
                String::from("mem"),
                format::memory(memory::used(), memory::memtotal()),
            ),
            uptime: Pair::new(String::from("up"), format::uptime(read::uptime())),
            battery: Pair::new(
                String::from("bat"),
                format::battery(read::battery_percentage(), read::battery_status()),
            ),
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

pub fn print_info(mut elems: Elements, opts: Options) {
    let padding: String = " ".repeat(elems.left_padding);
    if opts.shell_shorthand {
        elems.shell.modify(read::shell(true))
    } else {
        elems.shell.modify(read::shell(false))
    }
    match opts.color {
        true => {
            dsp!(
                elems.num_elements[0],
                padding,
                elems.hostname.key.purple().bold(),
                elems.separator,
                elems.hostname.value
            );
            dsp!(
                elems.num_elements[1],
                padding,
                elems.os.key.blue().bold(),
                elems.separator,
                elems.os.value
            );
            dsp!(
                elems.num_elements[2],
                padding,
                elems.kernel.key.cyan().bold(),
                elems.separator,
                elems.kernel.value
            );
            dsp!(
                elems.num_elements[3],
                padding,
                elems.packages.key.green().bold(),
                elems.separator,
                elems.os.value
            );
            dsp!(
                elems.num_elements[4],
                padding,
                elems.shell.key.yellow().bold(),
                elems.separator,
                elems.shell.value
            );
            dsp!(
                elems.num_elements[5],
                padding,
                elems.terminal.key.red().bold(),
                elems.separator,
                elems.terminal.value
            );
            dsp!(
                elems.num_elements[6],
                padding,
                elems.cpu.key.purple().bold(),
                elems.separator,
                elems.cpu.value
            );
            dsp!(
                elems.num_elements[7],
                padding,
                elems.memory.key.blue().bold(),
                elems.separator,
                elems.memory.value
            );
            dsp!(
                elems.num_elements[8],
                padding,
                elems.uptime.key.cyan().bold(),
                elems.separator,
                elems.uptime.value
            );
            dsp!(
                elems.num_elements[9],
                padding,
                elems.battery.key.green().bold(),
                elems.separator,
                elems.battery.value
            );
        }
        false => {
            dsp!(
                elems.num_elements[0],
                padding,
                elems.hostname.key,
                elems.separator,
                elems.hostname.value
            );
            dsp!(
                elems.num_elements[1],
                padding,
                elems.os.key,
                elems.separator,
                elems.os.value
            );
            dsp!(
                elems.num_elements[2],
                padding,
                elems.kernel.key,
                elems.separator,
                elems.kernel.value
            );
            dsp!(
                elems.num_elements[3],
                padding,
                elems.packages.key,
                elems.separator,
                elems.packages.value
            );
            dsp!(
                elems.num_elements[4],
                padding,
                elems.shell.key,
                elems.separator,
                elems.shell.value
            );
            dsp!(
                elems.num_elements[5],
                padding,
                elems.terminal.key,
                elems.separator,
                elems.terminal.value
            );
            dsp!(
                elems.num_elements[6],
                padding,
                elems.cpu.key,
                elems.separator,
                elems.cpu.value
            );
            dsp!(
                elems.num_elements[7],
                padding,
                elems.memory.key,
                elems.separator,
                elems.memory.value
            );
            dsp!(
                elems.num_elements[8],
                padding,
                elems.uptime.key,
                elems.separator,
                elems.uptime.value
            );
            dsp!(
                elems.num_elements[9],
                padding,
                elems.battery.key,
                elems.separator,
                elems.battery.value
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
