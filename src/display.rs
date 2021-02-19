use crate::{bars, format, read, DEFAULT_COLOR, DEFAULT_PADDING, DEFAULT_SEPARATOR_COLOR};
use colored::{Color, ColoredString, Colorize};
use rand::Rng;
use std::fmt;

/// __Options__ holds Macchina's behaviour that the user
/// can alter using the program's arguments
pub struct Options {
    pub palette_status: bool,
    pub shell_shorthand: bool,
}

impl Options {
    pub fn new() -> Options {
        Options {
            palette_status: false,
            shell_shorthand: false,
        }
    }
}

/// A __Pair__ is simply two strings: key and value (and the pair's visibility)
pub struct Pair {
    key: String,
    value: String,
    hidden: bool,
}

impl Pair {
    fn new(k: String, v: String) -> Pair {
        Pair {
            key: k,
            value: v,
            hidden: false,
        }
    }
    fn modify(&mut self, val: String) {
        self.value = val;
    }
    fn update_key(&mut self, val: String) {
        self.key = val;
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.key)
    }
}

pub struct Format {
    separator: String,
    bar: bool,
    bar_glyph: String,
    padding: String,
    pub longest_key: String,
    color: colored::Color,
    bracket_open: char,
    bracket_close: char,
    spacing: usize,
    separator_color: colored::Color,
}

impl Format {
    fn new() -> Format {
        Format {
            separator: String::from("-"),
            bar: false,
            bar_glyph: String::from("●"),
            bracket_open: '(',
            bracket_close: ')',
            padding: " ".repeat(DEFAULT_PADDING),
            color: DEFAULT_COLOR,
            separator_color: DEFAULT_SEPARATOR_COLOR,
            spacing: 1,
            longest_key: String::new(),
        }
    }
}

/// __Elements__ encapsulates elements that are to be displayed,
/// each element is a __Pair__
pub struct Elements {
    host: Pair,
    distro: Pair,
    desktop_env: Pair,
    machine: Pair,
    kernel: Pair,
    packages: Pair,
    shell: Pair,
    terminal: Pair,
    cpu: Pair,
    memory: Pair,
    uptime: Pair,
    battery: Pair,
    pub format: Format,
}

/// Initialize each pair of elements, assign them their key name and their value using functions
/// found in the _read crate_
impl Elements {
    pub fn new() -> Elements {
        Elements {
            host: Pair::new(String::from("Host"), format::host()),
            distro: Pair::new(String::from("Dist"), read::operating_system()),
            desktop_env: Pair::new(String::from("Desk"), read::desktop_session()),
            kernel: Pair::new(String::from("Kern"), read::kernel_version()),
            packages: Pair::new(String::from("Pkgs"), read::package_count()),
            shell: Pair::new(String::from("Shll"), String::new()),
            machine: Pair::new(String::from("Mach"), format::machine()),
            terminal: Pair::new(String::from("Term"), read::terminal()),
            cpu: Pair::new(String::from("Proc"), format::cpu()),
            memory: Pair::new(String::from("Memo"), format::memory()),
            uptime: Pair::new(String::from("Upti"), read::uptime()),
            battery: Pair::new(String::from("Batt"), format::battery()),
            format: Format::new(),
        }
    }
    pub fn set_theme_alt(&mut self) {
        self.format.separator = String::from("=>");
        self.format.bar_glyph = String::from("■");
        self.format.bracket_open = '[';
        self.format.bracket_close = ']';
        self.host.update_key(String::from("Ho"));
        self.machine.update_key(String::from("Ma"));
        self.distro.update_key(String::from("Os"));
        self.desktop_env.update_key(String::from("De"));
        self.kernel.update_key(String::from("Ke"));
        self.packages.update_key(String::from("Pa"));
        self.shell.update_key(String::from("Sh"));
        self.terminal.update_key(String::from("Te"));
        self.cpu.update_key(String::from("Cp"));
        self.memory.update_key(String::from("Me"));
        self.uptime.update_key(String::from("Up"));
        self.battery.update_key(String::from("Ba"));
        self.set_longest_key();
    }
    pub fn set_theme_long(&mut self) {
        self.format.separator = String::from("~");
        self.host.update_key(String::from("Hostname"));
        self.machine.update_key(String::from("Machine"));
        self.distro.update_key(String::from("Distribution"));
        self.desktop_env
            .update_key(String::from("Desktop Environment"));
        self.kernel.update_key(String::from("Kernel"));
        self.packages.update_key(String::from("Packages"));
        self.shell.update_key(String::from("Shell"));
        self.terminal.update_key(String::from("Terminal"));
        self.cpu.update_key(String::from("Processor"));
        self.memory.update_key(String::from("Memory"));
        self.uptime.update_key(String::from("Uptime"));
        self.battery.update_key(String::from("Battery"));
        self.set_longest_key();
    }
    pub fn set_color(&mut self, c: Color) {
        self.format.color = c;
    }
    pub fn set_separator_color(&mut self, c: Color) {
        self.format.separator_color = c;
    }
    pub fn set_left_padding_to(&mut self, amount: usize) {
        self.format.padding = " ".repeat(amount)
    }
    pub fn set_longest_key(&mut self) {
        self.format.longest_key = self.longest_key();
    }
    pub fn set_spacing(&mut self, v: usize) {
        self.format.spacing = v;
    }
    pub fn enable_bar(&mut self) {
        self.format.bar = true;
    }
    pub fn longest_key(&self) -> String {
        // Instead of manually declaring which key is the longest
        // in order to satisfy auto-spacing's algorithm, let longest_key()
        // determine the longest key
        let keys: Vec<String> = vec![
            self.host.key.clone(),
            self.machine.key.clone(),
            self.distro.key.clone(),
            self.desktop_env.key.clone(),
            self.kernel.key.clone(),
            self.packages.key.clone(),
            self.shell.key.clone(),
            self.terminal.key.clone(),
            self.cpu.key.clone(),
            self.uptime.key.clone(),
            self.memory.key.clone(),
            self.battery.key.clone(),
        ];

        let mut longest_key = keys[0].clone();
        for val in keys {
            if val.len() > longest_key.len() {
                longest_key = val;
            }
        }
        longest_key
    }
    pub fn calc_spacing(&self, current_key: &String, longest_key: &String) -> usize {
        (longest_key.len() + self.format.spacing) - current_key.len()
    }
}

trait Printing {
    fn print_host(&self);
    fn print_machine(&self);
    fn print_os(&self);
    fn print_desktop_env(&self);
    fn print_kernel_ver(&self);
    fn print_package_count(&self);
    fn print_shell(&self);
    fn print_terminal(&self);
    fn print_processor(&self);
    fn print_uptime(&self);
    fn print_memory(&self);
    fn print_battery(&self);
    fn print_bar(&self, bar: usize);
    fn print_palette(&self);
}

impl Printing for Elements {
    fn print_host(&self) {
        if !self.host.hidden {
            println!(
                "{}{}{}{}{}{}",
                self.format.padding,
                self.host.key.color(self.format.color).bold(),
                " ".repeat(self.calc_spacing(&self.host.key, &self.format.longest_key)),
                self.format
                    .separator
                    .color(self.format.separator_color)
                    .bold(),
                " ".repeat(self.format.spacing),
                self.host.value
            );
        }
    }
    fn print_machine(&self) {
        if !self.machine.hidden {
            println!(
                "{}{}{}{}{}{}",
                self.format.padding,
                self.machine.key.color(self.format.color).bold(),
                " ".repeat(self.calc_spacing(&self.machine.key, &self.format.longest_key)),
                self.format
                    .separator
                    .color(self.format.separator_color)
                    .bold(),
                " ".repeat(self.format.spacing),
                self.machine.value
            );
        }
    }
    fn print_os(&self) {
        if !self.distro.hidden {
            println!(
                "{}{}{}{}{}{}",
                self.format.padding,
                self.distro.key.color(self.format.color).bold(),
                " ".repeat(self.calc_spacing(&self.distro.key, &self.format.longest_key)),
                self.format
                    .separator
                    .color(self.format.separator_color)
                    .bold(),
                " ".repeat(self.format.spacing),
                self.distro.value
            );
        }
    }
    fn print_desktop_env(&self) {
        if !self.desktop_env.hidden {
            println!(
                "{}{}{}{}{}{}",
                self.format.padding,
                self.desktop_env.key.color(self.format.color).bold(),
                " ".repeat(self.calc_spacing(&self.desktop_env.key, &self.format.longest_key)),
                self.format
                    .separator
                    .color(self.format.separator_color)
                    .bold(),
                " ".repeat(self.format.spacing),
                self.desktop_env.value
            );
        }
    }
    fn print_kernel_ver(&self) {
        if !self.kernel.hidden {
            println!(
                "{}{}{}{}{}{}",
                self.format.padding,
                self.kernel.key.color(self.format.color).bold(),
                " ".repeat(self.calc_spacing(&self.kernel.key, &self.format.longest_key)),
                self.format
                    .separator
                    .color(self.format.separator_color)
                    .bold(),
                " ".repeat(self.format.spacing),
                self.kernel.value
            );
        }
    }
    fn print_package_count(&self) {
        if !self.packages.hidden {
            println!(
                "{}{}{}{}{}{}",
                self.format.padding,
                self.packages.key.color(self.format.color).bold(),
                " ".repeat(self.calc_spacing(&self.packages.key, &self.format.longest_key)),
                self.format
                    .separator
                    .color(self.format.separator_color)
                    .bold(),
                " ".repeat(self.format.spacing),
                self.packages.value
            );
        }
    }
    fn print_shell(&self) {
        if !self.shell.hidden {
            println!(
                "{}{}{}{}{}{}",
                self.format.padding,
                self.shell.key.color(self.format.color).bold(),
                " ".repeat(self.calc_spacing(&self.shell.key, &self.format.longest_key)),
                self.format
                    .separator
                    .color(self.format.separator_color)
                    .bold(),
                " ".repeat(self.format.spacing),
                self.shell.value
            );
        }
    }
    fn print_terminal(&self) {
        if !self.terminal.hidden {
            println!(
                "{}{}{}{}{}{}",
                self.format.padding,
                self.terminal.key.color(self.format.color).bold(),
                " ".repeat(self.calc_spacing(&self.terminal.key, &self.format.longest_key)),
                self.format
                    .separator
                    .color(self.format.separator_color)
                    .bold(),
                " ".repeat(self.format.spacing),
                self.terminal.value
            );
        }
    }
    fn print_processor(&self) {
        if !self.cpu.hidden {
            println!(
                "{}{}{}{}{}{}",
                self.format.padding,
                self.cpu.key.color(self.format.color).bold(),
                " ".repeat(self.calc_spacing(&self.cpu.key, &self.format.longest_key)),
                self.format
                    .separator
                    .color(self.format.separator_color)
                    .bold(),
                " ".repeat(self.format.spacing),
                self.cpu.value
            );
        }
    }
    fn print_uptime(&self) {
        if !self.uptime.hidden {
            println!(
                "{}{}{}{}{}{}",
                self.format.padding,
                self.uptime.key.color(self.format.color).bold(),
                " ".repeat(self.calc_spacing(&self.uptime.key, &self.format.longest_key)),
                self.format
                    .separator
                    .color(self.format.separator_color)
                    .bold(),
                " ".repeat(self.format.spacing),
                self.uptime.value
            );
        }
    }
    fn print_memory(&self) {
        if !self.memory.hidden {
            if self.format.bar {
                print!(
                    "{}{}{}{}{}",
                    self.format.padding,
                    self.memory.key.color(self.format.color).bold(),
                    " ".repeat(self.calc_spacing(&self.memory.key, &self.format.longest_key)),
                    self.format
                        .separator
                        .color(self.format.separator_color)
                        .bold(),
                    " ".repeat(self.format.spacing),
                );
                Printing::print_bar(self, bars::memory());
            } else {
                println!(
                    "{}{}{}{}{}{}",
                    self.format.padding,
                    self.memory.key.color(self.format.color).bold(),
                    " ".repeat(self.calc_spacing(&self.memory.key, &self.format.longest_key)),
                    self.format
                        .separator
                        .color(self.format.separator_color)
                        .bold(),
                    " ".repeat(self.format.spacing),
                    self.memory.value
                );
            }
        }
    }
    fn print_battery(&self) {
        if !self.battery.hidden {
            if self.format.bar {
                print!(
                    "{}{}{}{}{}",
                    self.format.padding,
                    self.battery.key.color(self.format.color).bold(),
                    " ".repeat(self.calc_spacing(&self.battery.key, &self.format.longest_key)),
                    self.format
                        .separator
                        .color(self.format.separator_color)
                        .bold(),
                    " ".repeat(self.format.spacing),
                );
                Printing::print_bar(self, bars::battery());
            } else {
                println!(
                    "{}{}{}{}{}{}",
                    self.format.padding,
                    self.battery.key.color(self.format.color).bold(),
                    " ".repeat(self.calc_spacing(&self.battery.key, &self.format.longest_key)),
                    self.format
                        .separator
                        .color(self.format.separator_color)
                        .bold(),
                    " ".repeat(self.format.spacing),
                    self.battery.value
                );
            }
        }
    }
    /// Print a bar next to memory and battery keys:
    /// it takes a function from the _bars crate_ as the first parameter
    /// and the color of the keys as a second
    fn print_bar(&self, bar: usize) {
        match &self.format.color {
            Color::White => println!(
                "{} {} {} {}",
                self.format.bracket_open,
                colored_blocks(self, bar),
                hidden_blocks(self, bar),
                self.format.bracket_close
            ),
            _ => println!(
                "{} {} {} {}",
                self.format.bracket_open,
                colored_blocks(self, bar),
                colorless_blocks(self, bar),
                self.format.bracket_close
            ),
        }
    }
    /// Print a palette using the terminal's colorscheme
    fn print_palette(&self) {
        println!(
            "{}{}{}{}{}{}{}{}{}",
            self.format.padding,
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
}

/// Handles displaying each element (key and value pair) found in
/// __Elements__ struct, as well as the palette.
pub fn print_info(mut elems: Elements, opts: Options) {
    if opts.shell_shorthand {
        elems.shell.modify(read::shell(true))
    } else {
        elems.shell.modify(read::shell(false))
    }

    elems.print_host();
    elems.print_machine();
    elems.print_os();
    elems.print_desktop_env();
    elems.print_kernel_ver();
    elems.print_package_count();
    elems.print_shell();
    elems.print_terminal();
    elems.print_processor();
    elems.print_uptime();
    elems.print_memory();
    elems.print_battery();

    if opts.palette_status {
        println!();
        elems.print_palette();
        println!();
    }
}

/// Hide an element or more e.g. package count, uptime etc. _(--hide <element>)_
pub fn hide(mut elems: Elements, options: Options, hide_parameters: Vec<&str>) {
    if hide_parameters.contains(&"host") {
        elems.host.hidden = true;
    }
    if hide_parameters.contains(&"mach") {
        elems.machine.hidden = true;
    }
    if hide_parameters.contains(&"distro") {
        elems.distro.hidden = true;
    }
    if hide_parameters.contains(&"desk") {
        elems.desktop_env.hidden = true;
    }
    if hide_parameters.contains(&"kern") {
        elems.kernel.hidden = true;
    }
    if hide_parameters.contains(&"pkgs") {
        elems.packages.hidden = true;
    }
    if hide_parameters.contains(&"sh") {
        elems.shell.hidden = true;
    }
    if hide_parameters.contains(&"term") {
        elems.terminal.hidden = true;
    }
    if hide_parameters.contains(&"cpu") {
        elems.cpu.hidden = true;
    }
    if hide_parameters.contains(&"up") {
        elems.uptime.hidden = true;
    }
    if hide_parameters.contains(&"mem") {
        elems.memory.hidden = true;
    }
    if hide_parameters.contains(&"bat") {
        elems.battery.hidden = true;
    }

    print_info(elems, options);
}

/// Colorize the keys using the user-specified color _(--color <color>)_
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
        _ => Color::Magenta,
    }
}

/// Using the _rand crate_, pick a random color for the keys _(--random-color)_
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

/// Prints a help message
pub fn help() {
    let usage_string: &str = "
    USAGE: macchina [OPTIONS]
    OPTIONS:
    -h, --help                      -   Print help text
    -v, --version                   -   Print Macchina's version
    -p, --palette                   -   Display palette
    -n, --no-color                  -   Disable colors
    -r, --random-color              -   Let Macchina pick a random color for you
    -c, --color <color>             -   Specify the key color
    -C, --separator-color <color>   -   Specify the separator color
    -t, --theme <theme>             -   Specify the theme to use
    -P, --padding <amount>          -   Specify the amount of left padding to use
    -S, --spacing <amount>          -   Specify the amount of spacing to use
    -b, --bar                       -   Display bars instead of values for battery and memory
    -s, --short-shell               -   Shorten shell output
    -H, --hide <element>            -   Hide the specified elements";
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
        Macchina's default key color is blue, but this can be overriden
        using --color / -c <color>
        Supported colors (case-sensitive):
            red, green, blue, magenta, cyan, yellow, black and white.
        To let Macchina randomly pick a color for you, use --random-color / -r
        To change the separator color, use --separator-color / -C <color>   

    Theming:
        Macchina comes with multiple themes out of the box,
        to change the default theme, use --theme / -t <theme>
        Supported themes (case-sensitive):
            def, alt and long.

    Hiding elements:
        To hide an element (or more), use --hide / -H <element>
        Hideable elements (case-sensitive):
            host, mach, distro, kern, pkgs, sh, term, cpu, up, mem, bat.";
    println!("{}", usage_string);
    println!("{}", help_string);
}

/// Return the correct amount of colored blocks: colored blocks are used blocks
pub fn colored_blocks(elems: &Elements, block_count: usize) -> ColoredString {
    let colored_blocks = elems.format.bar_glyph.repeat(block_count);
    colored_blocks
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .chunks(1)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
        .color(elems.format.color)
}

/// Return the correct amount of colorless blocks: colorless blocks are unused blocks
pub fn colorless_blocks(elems: &Elements, block_count: usize) -> String {
    let colorless_blocks = elems.format.bar_glyph.repeat(10 - block_count);
    colorless_blocks
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .chunks(1)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

// Used to correctly format the bars when using `--no-color`:
// Show the remaining unused blocks but they are hidden
pub fn hidden_blocks(elems: &Elements, block_count: usize) -> ColoredString {
    let colorless_blocks = elems.format.bar_glyph.repeat(10 - block_count);
    colorless_blocks
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .chunks(1)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
        .hidden()
}
