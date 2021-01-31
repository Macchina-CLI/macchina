extern crate num_cpus;
use crate::read;
use crate::{format, VERSION};
use crate::{memory, DEFAULT_COLOR, DEFAULT_PADDING};
use colored::{Color, Colorize};
use rand::Rng;
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
    fn new(k: String, v: String) -> Pair {
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
    color: colored::Color,
    num_elements: [bool; 10],
}

impl Elements {
    pub fn new() -> Elements {
        Elements {
            separator: ':',
            left_padding: DEFAULT_PADDING,
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
            color: DEFAULT_COLOR,
        }
    }
    pub fn set_color(&mut self, c: Color) {
        self.color = c;
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
        println!(
            "{} {}",
            padding, "-h, --help            -  display the help menu"
        );
        println!(
            "{} {}",
            padding, "-p, --palette         -  display the palette"
        );
        println!("{} {}", padding, "-n, --no-color        -  disable colors");
        println!(
            "{} {}",
            padding, "-r, --random-color    -  let macchina decide the color for you randomly"
        );
        println!(
            "{} {}",
            padding, "-c, --color           -  specify the color"
        );
        println!("{} {}", padding, "-H, --hide            -  hide elements");
        println!(
            "{} {}",
            padding, "-s, --short-sh        -  short shell output"
        );
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
                elems.hostname.key.color(elems.color).bold(),
                elems.separator,
                elems.hostname.value
            );
            dsp!(
                elems.num_elements[1],
                padding,
                elems.os.key.color(elems.color).bold(),
                elems.separator,
                elems.os.value
            );
            dsp!(
                elems.num_elements[2],
                padding,
                elems.kernel.key.color(elems.color).bold(),
                elems.separator,
                elems.kernel.value
            );
            dsp!(
                elems.num_elements[3],
                padding,
                elems.packages.key.color(elems.color).bold(),
                elems.separator,
                elems.packages.value
            );
            dsp!(
                elems.num_elements[4],
                padding,
                elems.shell.key.color(elems.color).bold(),
                elems.separator,
                elems.shell.value
            );
            dsp!(
                elems.num_elements[5],
                padding,
                elems.terminal.key.color(elems.color).bold(),
                elems.separator,
                elems.terminal.value
            );
            dsp!(
                elems.num_elements[6],
                padding,
                elems.cpu.key.color(elems.color).bold(),
                elems.separator,
                elems.cpu.value
            );
            dsp!(
                elems.num_elements[7],
                padding,
                elems.memory.key.color(elems.color).bold(),
                elems.separator,
                elems.memory.value
            );
            dsp!(
                elems.num_elements[8],
                padding,
                elems.uptime.key.color(elems.color).bold(),
                elems.separator,
                elems.uptime.value
            );
            dsp!(
                elems.num_elements[9],
                padding,
                elems.battery.key.color(elems.color).bold(),
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

pub fn choose_color(color: &str) -> Color {
    match color {
        "black" => Color::Black,
        "red" => Color::Red,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "blue" => Color::Blue,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "white" => Color::White,
        _ => color_error(),
    }
}

fn color_error() -> Color {
    return Color::Magenta;
}

pub fn randomize_color() -> Color {
    let mut rng = rand::thread_rng();
    let rand: usize = rng.gen_range(0..8);
    match rand {
        0 => return Color::Red,
        1 => return Color::Green,
        2 => return Color::Blue,
        3 => return Color::Magenta,
        4 => return Color::Yellow,
        5 => return Color::Cyan,
        6 => return Color::Black,
        _ => return Color::White,
    }
}

pub fn help() {
    let elems = Elements::new();
    let padding: String = " ".repeat(elems.left_padding);
    println!("{}{}, v{}", padding, "Macchina".blue().bold(), VERSION);
    usage!(elems);
    println!();
    println!("{}{}:", padding, "Battery information".green().bold());
    println!("{}{}", padding, "Battery information might print an error if the file macchina");
    println!("{}{}", padding, "is trying to read from does not exist.");
    println!();
    println!("{}{}", padding, "Macchina reads battery information from two paths.");
    println!("{}{}", padding, "Each value is contained in a constant.");
    println!("{}{}", padding, "These two constants are defined in main.rs.");
    println!("{}{}{}", padding, padding, "PATH_TO_BATTERY_PERCENTAGE = /sys/class/power_supply/BAT0/capacity");
    println!("{}{}{}", padding, padding, "PATH_TO_BATTERY_STATUS = /sys/class/power_supply/BAT0/status");
    println!("{}{}", padding, "----------------------------------");
    println!("{}{}:", padding, "Package information".green().bold());
    println!(
        "{}{}{}{}{}",
        padding,
        "Package count will equal ",
        "0".white().bold(),
        " if the system is ",
        "not arch-based".bold()
    );
    println!("{}{}", padding, "as Macchina queries pacman to get a list of the installed packages.");
    println!("{}{}", padding, "-----------------------------------");
    println!("{}{}:", padding, "Coloring".green().bold());
    println!("{}{}", padding, "Macchina's default color is magenta, but this can be overriden.");
    println!("{}{}", padding, "--color / -c supports a number of colors, provided by the colored crate.");
    println!(
        "{}Supported colors: {}, {}, {}, {}, {}, {}, {}, {}",
        padding,
        "red".red(),
        "green".green(),
        "blue".blue(),
        "magenta".magenta(),
        "yellow".yellow(),
        "cyan".cyan(),
        "black".black(),
        "white".white()
    );
    println!("{}{}", padding, "You may also run macchina followed by -r / --random-color");
    println!("{}{}", padding, "to get a different color everytime.");
    println!("{}{}", padding, "-----------------------------------");
    println!("{}{}:", padding, "Hiding elements".green().bold());
    println!("{}{}", padding, "Macchina allows you to hide elements using -H / --hide e.g. ");
    println!("{}{}", padding, "to hide the kernel version and the package count simply run:");
    println!("{}{}", padding, "     $ macchina --hide kern pkgs");
    println!("{}{}", padding, "In case the element you are trying to hide doesn't exist,");
    println!("{}{}", padding, "Macchina will display an error alongside a list of hideable elements");
}
