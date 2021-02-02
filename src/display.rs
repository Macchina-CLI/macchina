extern crate num_cpus;
use crate::read;
use crate::{format, VERSION};
use crate::{memory, DEFAULT_COLOR, DEFAULT_SEPARATOR_COLOR, DEFAULT_PADDING};
use colored::{Color, Colorize};
use rand::Rng;

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
    fn update_key(&mut self, val: String) {
        self.key = val;
    }
}

pub struct Elements {
    separator: String,
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
    separator_color: colored::Color,
    num_elements: [bool; 10],
}

impl Elements {
    pub fn new() -> Elements {
        Elements {
            separator: String::from(":"),
            left_padding: DEFAULT_PADDING,
            hostname: Pair::new(String::from("host"), read::hostname()),
            os: Pair::new(String::from("os"), read::operating_system()),
            kernel: Pair::new(String::from("kern"), read::kernel_version()),
            packages: Pair::new(String::from("pkgs"), read::package_count().to_string()),
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
            separator_color: DEFAULT_SEPARATOR_COLOR,
        }
    }
    pub fn set_theme_alt(&mut self) {
        self.separator = String::from("  => ");
        self.hostname.update_key(String::from("Ho"));
        self.os.update_key(String::from("Os"));
        self.kernel.update_key(String::from("Ke"));
        self.packages.update_key(String::from("Pa"));
        self.shell.update_key(String::from("Sh"));
        self.terminal.update_key(String::from("Te"));
        self.cpu.update_key(String::from("Cp"));
        self.memory.update_key(String::from("Me"));
        self.uptime.update_key(String::from("Up"));
        self.battery.update_key(String::from("Ba"));
    }
    pub fn set_theme_giraffe(&mut self) {
        self.separator = String::from("  ~ ");
        self.hostname.update_key(String::from("Hostname"));
        self.os.update_key(String::from("Distribution"));
        self.kernel.update_key(String::from("Kernel"));
        self.packages.update_key(String::from("Packages"));
        self.shell.update_key(String::from("Shell"));
        self.terminal.update_key(String::from("Terminal"));
        self.cpu.update_key(String::from("Processor"));
        self.memory.update_key(String::from("Memory"));
        self.uptime.update_key(String::from("Uptime"));
        self.battery.update_key(String::from("Battery"));
    }
    pub fn set_color(&mut self, c: Color) {
        self.color = c;
    }
    pub fn set_separator_color(&mut self, c: Color) {
        self.separator_color = c;
    }
    pub fn set_left_padding_to(&mut self, val: usize) {
        self.left_padding = val;
    }
}

macro_rules! usage {
    ($i: ident) => {
        let padding: String = " ".repeat($i.left_padding);
        println!(
            "{}{} <{}>",
            padding,
            "USAGE: macchina",
            "OPTIONS".blue().bold()
        );
        println!("{}{}:", padding, "OPTIONS".blue().bold());
        println!(
            "{} {}",
            padding, "-h, --help  -  display the help menu"
        );
        println!(
            "{} {}",
            padding, "-p, --palette  -  display the palette"
        );
        println!("{} {}", padding, "-n, --no-color  -  disable colors");
        println!(
            "{} {}",
            padding, "-r, --random-color  -  let macchina pick a random color for you"
        );
        println!(
            "{} {}",
            padding, "-c, --color <color>  -  specify the color"
        );
        println!(
            "{} {}",
            padding, "-C, --separator-color <color>  -  specify the separator color"
        );
        println!(
            "{} {}",
            padding, "-t, --theme <theme_name>  -  specify the theme"
        );
        println!("{} {}", padding, "-H, --hide <element>  -  hide elements");
        println!(
            "{} {}",
            padding, "-s, --short-sh  -  short shell output"
        );
        println!(
            "{} {}",
            padding, "-P, --padding  <amount> -  specify the amount of padding to use"
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
                elems.separator.color(elems.separator_color).bold(),
                elems.hostname.value
            );
            dsp!(
                elems.num_elements[1],
                padding,
                elems.os.key.color(elems.color).bold(),
                elems.separator.color(elems.separator_color).bold(),
                elems.os.value
            );
            dsp!(
                elems.num_elements[2],
                padding,
                elems.kernel.key.color(elems.color).bold(),
                elems.separator.color(elems.separator_color).bold(),
                elems.kernel.value
            );
            dsp!(
                elems.num_elements[3],
                padding,
                elems.packages.key.color(elems.color).bold(),
                elems.separator.color(elems.separator_color).bold(),
                elems.packages.value
            );
            dsp!(
                elems.num_elements[4],
                padding,
                elems.shell.key.color(elems.color).bold(),
                elems.separator.color(elems.separator_color).bold(),
                elems.shell.value
            );
            dsp!(
                elems.num_elements[5],
                padding,
                elems.terminal.key.color(elems.color).bold(),
                elems.separator.color(elems.separator_color).bold(),
                elems.terminal.value
            );
            dsp!(
                elems.num_elements[6],
                padding,
                elems.cpu.key.color(elems.color).bold(),
                elems.separator.color(elems.separator_color).bold(),
                elems.cpu.value
            );
            dsp!(
                elems.num_elements[7],
                padding,
                elems.memory.key.color(elems.color).bold(),
                elems.separator.color(elems.separator_color).bold(),
                elems.memory.value
            );
            dsp!(
                elems.num_elements[8],
                padding,
                elems.uptime.key.color(elems.color).bold(),
                elems.separator.color(elems.separator_color).bold(),
                elems.uptime.value
            );
            dsp!(
                elems.num_elements[9],
                padding,
                elems.battery.key.color(elems.color).bold(),
                elems.separator.color(elems.separator_color).bold(),
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
    //  Labels contains all hideable elements.
    //  The order of each element in the array
    //  is important for the hide functionality
    //  to work properly
    let labels: [&str; 10] = [
        "host", "os", "kern", "pkgs", "sh", "term", "cpu", "mem", "up", "bat",
    ];
    
    for i in 0..10 {
            if hide_parameters.contains(&labels[i]) {
                elems.num_elements[i] = false;
        }
    }

    print_info(elems, options);
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
    println!("{}Macchina, version {}", padding, VERSION);
    println!();
    usage!(elems);
    println!();
    println!("{}{}", padding, "Battery information".blue().bold());
    println!("{}{}", padding, "Battery information might print an error if the file macchina");
    println!("{}{}", padding, "is trying to read from does not exist.");
    println!();
    println!("{}{}", padding, "Macchina reads battery information from two paths.");
    println!("{}{}", padding, "Each path is contained in a constant.");
    println!("{}{}", padding, "These two constants are defined in main.rs:");
    println!("{}    {}", padding, "PATH_TO_BATTERY_PERCENTAGE = /sys/class/power_supply/BAT0/capacity");
    println!("{}    {}", padding, "PATH_TO_BATTERY_STATUS = /sys/class/power_supply/BAT0/status");
    println!("{}{}", padding, "If one of the paths does not exist, or is incorrect, macchina will print");
    println!("{}{}", padding, "\"could not extract battery info\" next to the battery key.");
    println!("{}{}", padding, "----------------------------------");
    println!("{}{}", padding, "Package information".blue().bold());
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
    println!("{}{}", padding, "Coloring".blue().bold());
    println!("{}{}", padding, "Macchina's default key color is magenta, but this can be overriden.");
    println!("{}{}", padding, "using --color / -c <color>");
    println!("{}{}", padding, "You can also change the default separator color using");
    println!("{}{}", padding, "--separator-color / -C <color>");
    println!("{}{}", padding, "these two arguments support a range of colors, provided by the colored crate.");
    println!("{}{}", padding, "     Supported colors: red, green, blue, magenta, cyan, yellow, black and white.");
    println!("{}{}", padding, "You may also run macchina followed by -r / --random-color");
    println!("{}{}", padding, "to let Macchina choose a random color you.");
    println!("{}{}", padding, "-----------------------------------");
    println!("{}{}", padding, "Theming".blue().bold());
    println!("{}{}", padding, "Macchina offers themes for you to change between on the fly using");
    println!("{}{}", padding, "the --theme / -t argument.");
    println!("{}{}", padding, "-----------------------------------");
    println!("{}{}", padding, "Hiding elements".blue().bold());
    println!("{}{}", padding, "Macchina allows you to hide elements using -H / --hide e.g. ");
    println!("{}{}", padding, "to hide the kernel version and the package count simply run:");
    println!("{}{}", padding, "     $ macchina --hide kern pkgs");
    println!("{}{}", padding, "In case the element you are trying to hide doesn't exist,");
    println!("{}{}", padding, "Macchina will display an error alongside a list of hideable elements");
}
