extern crate num_cpus;
use crate::{bars, format, memory, read, DEFAULT_COLOR, DEFAULT_PADDING, DEFAULT_SEPARATOR_COLOR};
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
    machine: Pair,
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
    num_elements: [bool; 11],
    bar: bool,
}

impl Elements {
    pub fn new() -> Elements {
        Elements {
            separator: String::from(":"),
            left_padding: DEFAULT_PADDING,
            bar: false,
            hostname: Pair::new(String::from("host"), read::hostname()),
            os: Pair::new(String::from("os"), read::operating_system()),
            kernel: Pair::new(String::from("kern"), read::kernel_version()),
            packages: Pair::new(String::from("pkgs"), read::package_count().to_string()),
            shell: Pair::new(String::from("sh"), String::new()),
            machine: Pair::new(String::from("mach"), read::product_name()),
            terminal: Pair::new(String::from("term"), read::terminal()),
            cpu: Pair::new(
                String::from("cpu"),
                format::cpu(read::cpu_model_name(), num_cpus::get()),
            ),
            memory: Pair::new(
                String::from("mem"),
                format::memory(memory::used(), memory::memtotal()),
            ),
            uptime: Pair::new(String::from("up"), read::uptime()),
            battery: Pair::new(
                String::from("bat"),
                format::battery(read::battery_percentage(), read::battery_status()),
            ),
            num_elements: [true; 11],
            color: DEFAULT_COLOR,
            separator_color: DEFAULT_SEPARATOR_COLOR,
        }
    }
    pub fn set_theme_alt(&mut self) {
        self.separator = String::from("  => ");
        self.hostname.update_key(String::from("Ho"));
        self.machine.update_key(String::from("Ma"));
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
        self.machine.update_key(String::from("Machine"));
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
    pub fn enable_bar(&mut self) {
        self.bar = true;
    }
}

/// dsp: display element
macro_rules! dsp {
    ($elem: expr, $pad: ident, $key: expr, $sep: expr, $val: expr) => {
        if $elem {
            println!("{}{}{} {}", $pad, $key, $sep, $val);
        }
    };
}

macro_rules! dsp_bar {
    ($elem: expr, $pad: ident, $key: expr, $sep: expr) => {
        if $elem {
            print!("{}{}{} ", $pad, $key, $sep);
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
                elems.machine.key.color(elems.color).bold(),
                elems.separator.color(elems.separator_color).bold(),
                elems.machine.value
            );
            dsp!(
                elems.num_elements[2],
                padding,
                elems.os.key.color(elems.color).bold(),
                elems.separator.color(elems.separator_color).bold(),
                elems.os.value
            );
            dsp!(
                elems.num_elements[3],
                padding,
                elems.kernel.key.color(elems.color).bold(),
                elems.separator.color(elems.separator_color).bold(),
                elems.kernel.value
            );
            dsp!(
                elems.num_elements[4],
                padding,
                elems.packages.key.color(elems.color).bold(),
                elems.separator.color(elems.separator_color).bold(),
                elems.packages.value
            );
            dsp!(
                elems.num_elements[5],
                padding,
                elems.shell.key.color(elems.color).bold(),
                elems.separator.color(elems.separator_color).bold(),
                elems.shell.value
            );
            dsp!(
                elems.num_elements[6],
                padding,
                elems.terminal.key.color(elems.color).bold(),
                elems.separator.color(elems.separator_color).bold(),
                elems.terminal.value
            );
            dsp!(
                elems.num_elements[7],
                padding,
                elems.cpu.key.color(elems.color).bold(),
                elems.separator.color(elems.separator_color).bold(),
                elems.cpu.value
            );
            dsp!(
                elems.num_elements[8],
                padding,
                elems.uptime.key.color(elems.color).bold(),
                elems.separator.color(elems.separator_color).bold(),
                elems.uptime.value
            );
            match elems.bar {
                false => {
                    dsp!(
                        elems.num_elements[9],
                        padding,
                        elems.memory.key.color(elems.color).bold(),
                        elems.separator.color(elems.separator_color).bold(),
                        elems.memory.value
                    );
                }
                true => {
                    dsp_bar!(
                        elems.num_elements[9],
                        padding,
                        elems.memory.key.color(elems.color).bold(),
                        elems.separator.color(elems.separator_color).bold()
                    );
                    if elems.num_elements[9] {
                        show_bar(bars::memory(), elems.color);
                    }
                }
            }
            match elems.bar {
                false => {
                    dsp!(
                        elems.num_elements[10],
                        padding,
                        elems.battery.key.color(elems.color).bold(),
                        elems.separator.color(elems.separator_color).bold(),
                        elems.battery.value
                    );
                }
                true => {
                    dsp_bar!(
                        elems.num_elements[10],
                        padding,
                        elems.battery.key.color(elems.color).bold(),
                        elems.separator.color(elems.separator_color).bold()
                    );
                    if elems.num_elements[10] {
                        show_bar(bars::battery(), elems.color);
                    }
                }
            }
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
                elems.machine.key,
                elems.separator,
                elems.machine.value
            );
            dsp!(
                elems.num_elements[2],
                padding,
                elems.os.key,
                elems.separator,
                elems.os.value
            );
            dsp!(
                elems.num_elements[3],
                padding,
                elems.kernel.key,
                elems.separator,
                elems.kernel.value
            );
            dsp!(
                elems.num_elements[4],
                padding,
                elems.packages.key,
                elems.separator,
                elems.packages.value
            );
            dsp!(
                elems.num_elements[5],
                padding,
                elems.shell.key,
                elems.separator,
                elems.shell.value
            );
            dsp!(
                elems.num_elements[6],
                padding,
                elems.terminal.key,
                elems.separator,
                elems.terminal.value
            );
            dsp!(
                elems.num_elements[7],
                padding,
                elems.cpu.key,
                elems.separator,
                elems.cpu.value
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
                elems.memory.key,
                elems.separator,
                elems.memory.value
            );
            dsp!(
                elems.num_elements[10],
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
    let labels: [&str; 11] = [
        "host", "mach", "os", "kern", "pkgs", "sh", "term", "cpu", "up", "mem", "bat",
    ];

    for i in 0..11 {
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
    let usage_string: &str = "
    USAGE: macchina [OPTIONS]
    OPTIONS:
    -h, --help                      -   display the help menu
    -p, --palette                   -   display the palette
    -n, --no-color                  -   disable colors
    -r, --random-color              -   let macchina pick a random color for you
    -c, --color <color>             -   specify the key color
    -C, --separator-color <color>   -   specify the separator color
    -t, --theme <theme>             -   specify the theme
    -H, --hide <element>            -   hide the specified elements
    -P, --padding <amount>          -   specify the amount of left padding to use
    -b, --bar                       -   display bars instead of values for battery and memory";
    let help_string: &str = "
    Battery Information:
        Battery information might print an error if the file macchina is 
        trying to read from does not exist.
        Macchina reads battery information from two paths.
        Each path is contained in a constant.
        These two constants are defined in main.rs:
            PATH_TO_BATTERY_PERCENTAGE = /sys/class/power_supply/BAT0/capacity
            PATH_TO_BATTERY_STATUS = /sys/class/power_supply/BAT0/status
        If one of the paths does not exist, macchina will print \"could not extract battery info\"

    Package information:
        Package count will equal 0 if the system is not arch-based, as macchina queries pacman to
        get a list of the installed packages and then return the package count.

    Coloring:
        Macchina's default key color is magenta, but this can be overriden
        using --color / -c <color>
        Supported colors (case-sensitive):
            red, green, blue, magenta, cyan, yellow, black and white.
        To let macchina randomly pick a color for you, use --random-color / -r
        To change the separator color, use --separator-color / -C <color>   

    Theming:
        Macchina comes with multiple themes out of the box,
        to change the default theme, use --theme / -t <theme>
        Supported themes (case-sensitive):
            def, alt and giraffe.

    Hiding elements:
        To hide an element (or more), use --hide / -H <element>
        Hideable elements (case-sensitive):
            host, mach, os, kern, pkgs, sh, term, cpu, up, mem, bat ";
    println!("{}",usage_string);
    println!("{}", help_string);
}

pub fn show_bar(bar: usize, color: Color) {
    match color {
        Color::Black
        | Color::Blue
        | Color::Red
        | Color::Green
        | Color::Yellow
        | Color::Cyan
        | Color::Magenta => match bar {
            1 => println!("[ {} ■ ■ ■ ■ ■ ■ ■ ■ ■ ]", "■".color(color)),
            2 => println!(
                "[ {} {} ■ ■ ■ ■ ■ ■ ■ ■ ]",
                "■".color(color),
                "■".color(color)
            ),
            3 => println!(
                "[ {} {} {} ■ ■ ■ ■ ■ ■ ■ ]",
                "■".color(color),
                "■".color(color),
                "■".color(color)
            ),
            4 => println!(
                "[ {} {} {} {} ■ ■ ■ ■ ■ ■ ]",
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color)
            ),
            5 => println!(
                "[ {} {} {} {} {} ■ ■ ■ ■ ■ ]",
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color)
            ),
            6 => println!(
                "[ {} {} {} {} {} {} ■ ■ ■ ■ ]",
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color)
            ),
            7 => println!(
                "[ {} {} {} {} {} {} {} ■ ■ ■ ]",
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color)
            ),
            8 => println!(
                "[ {} {} {} {} {} {} {} {} ■ ■ ]",
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color)
            ),
            9 => println!(
                "[ {} {} {} {} {} {} {} {} {} ■ ]",
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color)
            ),
            10 => println!(
                "[ {} {} {} {} {} {} {} {} {} {} ]",
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color),
                "■".color(color)
            ),
            _ => println!("could not display memory bar"),
        },
        _ => match bar {
            1 => println!("[ ■                   ]"),
            2 => println!("[ ■ ■                 ]"),
            3 => println!("[ ■ ■ ■               ]"),
            4 => println!("[ ■ ■ ■ ■             ]"),
            5 => println!("[ ■ ■ ■ ■ ■           ]"),
            6 => println!("[ ■ ■ ■ ■ ■ ■         ]"),
            7 => println!("[ ■ ■ ■ ■ ■ ■ ■       ]"),
            8 => println!("[ ■ ■ ■ ■ ■ ■ ■ ■     ]"),
            9 => println!("[ ■ ■ ■ ■ ■ ■ ■ ■ ■   ]"),
            10 => println!("[ ■ ■ ■ ■ ■ ■ ■ ■ ■ ■ ]"),
            _ => println!("could not display memory bar"),
        },
    }
}
