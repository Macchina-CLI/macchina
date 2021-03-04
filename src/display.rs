use crate::{
    bars, format, general, package, DEFAULT_COLOR, DEFAULT_PADDING, DEFAULT_SEPARATOR_COLOR,
};
use colored::{Color, Colorize};
use rand::Rng;
use std::fmt;
#[allow(dead_code)]
pub struct FailedComponent {
    pub failed: bool,
    pub extraction_method: String,
    pub reason: String,
}

impl FailedComponent {
    fn new(f: bool, e: String) -> FailedComponent {
        FailedComponent {
            failed: f,
            extraction_method: e,
            reason: String::new(),
        }
    }
}

/// Elements struct interacts with Fail struct to hide any of its elements whose value is "Unknown".
//  Fail also automatically fails certain elements in certain conditions:
//  -  Automatically fail the distribution key if NetBSD is detected.
//  -  Automatically fail the desktop environment key if only a WM is detected.
pub struct Fail {
    pub window_man: FailedComponent,
    pub desktop_env: FailedComponent,
    pub distro: FailedComponent,
    pub uptime: FailedComponent,
    pub battery: FailedComponent,
    pub host: FailedComponent,
    pub shell: FailedComponent,
    pub terminal: FailedComponent,
    pub packages: FailedComponent,
}

impl Fail {
    pub fn new() -> Fail {
        Fail {
            window_man: FailedComponent::new(false, String::from("(ERROR:DISABLED) Window Manager -> Extracted using \"wmctrl -m | grep Name:\"")),
            desktop_env: FailedComponent::new(
                false,
                String::from("(ERROR:DISABLED) Desktop Environment -> Obtained from \"DESKTOP_SESSION\" OR \"XDG_CURRENT_DESKTOP\" environment variables
                            Ignore if not running a desktop environment."),
            ),
            uptime: FailedComponent::new(
                false,
                String::from("(ERROR:DISABLED) Uptime -> Extracted from /proc/uptime"),
            ),
            battery: FailedComponent::new(
                false,
                String::from("(ERROR:DISABLED) Battery -> Percentage extracted from /sys/class/power_supply/BAT0/capacity
                            Status extracted from /sys/class/power_supply/BAT0/status
                            Ignore if on a desktop computer.
                "),
            ),
            host: FailedComponent::new(
                false,
                String::from("(ERROR:DISABLED) Host -> Obtained from nix::unistd::gethostname()")
            ),
            shell: FailedComponent::new(
                false,
                String::from("(ERROR:DISABLED) Shell -> Extracted using \"ps -p $$ -o comm=\" OR \"ps -p $$ -o args=\"")
            ),
            terminal: FailedComponent::new(
                false,
                String::from("(ERROR:DISABLED) Terminal -> Extracted using \"ps -p $$ -p\"")
            ),
            packages: FailedComponent::new(
                false,
                String::from("(ERROR:DISABLED) Packages -> 
                            (Arch-based distros) Extracted using \"pacman -Qq | wc -l\"
                            (Debian-based distros) Extracted using \"dpkg -l | wc -l\"
                            (NetBSD) Extracted using \"pkg_info | wc -l\"
                "),
            ),
            #[cfg(target_os = "linux")]
            distro: FailedComponent::new(
                false,
                String::from("(ERROR:DISABLED) Distribution -> Extracted using \"cat /etc/os-release | head -n 1\""),
            ),
            #[cfg(target_os = "netbsd")]
            distro: FailedComponent::new(
                true,
                String::from("(OK:DISABLED) Distribution -> NetBSD system detected, so the distribution is automatically hidden."),
            ),
        }
    }
    pub fn count_print_failed(&self, failed_comp: &FailedComponent, mut num_fails: usize) -> usize {
        if failed_comp.failed {
            num_fails = num_fails + 1;
            println!("{}", failed_comp.extraction_method);
        }
        num_fails
    }
    pub fn print_failed(&self) {
        let mut num_fails: usize = 0;
        num_fails = self.count_print_failed(&self.desktop_env, num_fails);
        num_fails = self.count_print_failed(&self.battery, num_fails);
        num_fails = self.count_print_failed(&self.window_man, num_fails);
        num_fails = self.count_print_failed(&self.uptime, num_fails);
        num_fails = self.count_print_failed(&self.host, num_fails);
        num_fails = self.count_print_failed(&self.shell, num_fails);
        num_fails = self.count_print_failed(&self.terminal, num_fails);
        num_fails = self.count_print_failed(&self.packages, num_fails);
        if num_fails == 0 {
            println!(
                "Everything is displaying correctly!\nIf this is not true, please create an issue at https://github.com/grtcdr/macchina"
            )
        }
    }
}

/// __Options__ holds Macchina's behaviour that the user
/// can alter using the program's arguments
pub struct Options {
    pub palette_status: bool,
    pub shell_shorthand: bool,
    pub uptime_shorthand: bool,
}

impl Options {
    pub fn new() -> Options {
        Options {
            palette_status: false,
            shell_shorthand: false,
            uptime_shorthand: false,
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
    fn new(k: String) -> Pair {
        Pair {
            key: k,
            value: String::new(),
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
    pub host: Pair,
    pub distro: Pair,
    pub desktop_env: Pair,
    pub window_man: Pair,
    pub machine: Pair,
    pub kernel: Pair,
    pub packages: Pair,
    pub shell: Pair,
    pub terminal: Pair,
    pub cpu: Pair,
    pub memory: Pair,
    pub uptime: Pair,
    pub battery: Pair,
    pub format: Format,
}

/// Initialize each pair of elements, assign them their key name and their value using functions
/// found in the _read crate_
impl Elements {
    pub fn new() -> Elements {
        Elements {
            host: Pair::new(String::from("Host")),
            machine: Pair::new(String::from("Machine")),
            distro: Pair::new(String::from("Distro")),
            desktop_env: Pair::new(String::from("DE")),
            window_man: Pair::new(String::from("WM")),
            #[cfg(target_os = "linux")]
            kernel: Pair::new(String::from("Kernel")),
            #[cfg(target_os = "netbsd")]
            kernel: Pair::new(String::from("OS")),
            packages: Pair::new(String::from("Packages")),
            shell: Pair::new(String::from("Shell")),
            terminal: Pair::new(String::from("Terminal")),
            cpu: Pair::new(String::from("CPU")),
            memory: Pair::new(String::from("Memory")),
            uptime: Pair::new(String::from("Uptime")),
            battery: Pair::new(String::from("Battery")),
            format: Format::new(),
        }
    }
    pub fn is_running_wm_only(&mut self, fail: &mut Fail, apply: bool) -> bool {
        if general::desktop_environment(fail).to_uppercase()
            == general::window_manager(fail).to_uppercase()
            && apply
        {
            fail.desktop_env.failed = true;
            return true;
        } else {
            fail.desktop_env.failed = false
        }
        false
    }
    pub fn set_theme_alt(&mut self, fail: &mut Fail) {
        self.format.separator = String::from("=>");
        self.format.bar_glyph = String::from("■");
        self.format.bracket_open = '[';
        self.format.bracket_close = ']';
        self.host.update_key(String::from("Ho"));
        self.machine.update_key(String::from("Ma"));
        self.distro.update_key(String::from("Di"));
        self.desktop_env.update_key(String::from("De"));
        self.window_man.update_key(String::from("Wm"));
        #[cfg(target_os = "linux")]
        self.kernel.update_key(String::from("Ke"));
        #[cfg(target_os = "netbsd")]
        self.kernel.update_key(String::from("Os"));
        self.packages.update_key(String::from("Pa"));
        self.shell.update_key(String::from("Sh"));
        self.terminal.update_key(String::from("Te"));
        self.cpu.update_key(String::from("Cp"));
        self.memory.update_key(String::from("Me"));
        self.uptime.update_key(String::from("Up"));
        self.battery.update_key(String::from("Ba"));
        self.set_longest_key(fail);
    }
    pub fn set_theme_long(&mut self, fail: &mut Fail) {
        self.format.separator = String::from("~");
        self.host.update_key(String::from("Hostname"));
        self.machine.update_key(String::from("Machine"));
        self.distro.update_key(String::from("Distribution"));
        self.desktop_env
            .update_key(String::from("Desktop Environment"));
        self.window_man.update_key(String::from("Window Manager"));
        #[cfg(target_os = "linux")]
        self.kernel.update_key(String::from("Kernel"));
        #[cfg(target_os = "netbsd")]
        self.kernel.update_key(String::from("Operating System"));
        self.packages.update_key(String::from("Packages"));
        self.shell.update_key(String::from("Shell"));
        self.terminal.update_key(String::from("Terminal"));
        self.cpu.update_key(String::from("Processor"));
        self.memory.update_key(String::from("Memory"));
        self.uptime.update_key(String::from("Uptime"));
        self.battery.update_key(String::from("Battery"));
        self.set_longest_key(fail);
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
    pub fn set_longest_key(&mut self, fail: &mut Fail) {
        self.format.longest_key = self.longest_key(fail);
    }
    pub fn set_spacing(&mut self, v: usize) {
        self.format.spacing = v;
    }
    pub fn enable_bar(&mut self) {
        self.format.bar = true;
    }
    pub fn longest_key(&mut self, fail: &mut Fail) -> String {
        // Instead of manually declaring which key is the longest
        // in order to satisfy auto-spacing's algorithm, let longest_key()
        // determine the longest key
        let mut keys: Vec<String> = Vec::new();
        if !self.host.hidden {
            keys.push(self.host.key.clone());
        }
        if !self.machine.hidden {
            keys.push(self.machine.key.clone());
        }
        if !self.kernel.hidden {
            keys.push(self.kernel.key.clone());
        }
        if !self.distro.hidden {
            keys.push(self.distro.key.clone());
        }
        if !self.packages.hidden {
            keys.push(self.packages.key.clone());
        }
        if !self.shell.hidden {
            keys.push(self.shell.key.clone());
        }
        if !self.terminal.hidden {
            keys.push(self.terminal.key.clone());
        }
        if !self.cpu.hidden {
            keys.push(self.cpu.key.clone());
        }
        if !self.uptime.hidden {
            keys.push(self.uptime.key.clone());
        }
        if !self.memory.hidden {
            keys.push(self.memory.key.clone());
        }
        if !self.battery.hidden {
            keys.push(self.battery.key.clone());
        }
        if self.is_running_wm_only(fail, false) {
            keys.push(self.window_man.key.clone());
        } else {
            if !self.desktop_env.hidden {
                keys.push(self.desktop_env.key.clone());
            }
            if !self.window_man.hidden {
                keys.push(self.desktop_env.key.clone());
            }
        }
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
    pub fn hide_all(&mut self) {
        self.host.hidden = true;
        self.machine.hidden = true;
        self.distro.hidden = true;
        self.desktop_env.hidden = true;
        self.window_man.hidden = true;
        self.kernel.hidden = true;
        self.packages.hidden = true;
        self.shell.hidden = true;
        self.terminal.hidden = true;
        self.cpu.hidden = true;
        self.uptime.hidden = true;
        self.memory.hidden = true;
        self.battery.hidden = true;
    }
    pub fn apply_shorthand_values(&mut self, opts: &Options, fail: &mut Fail) {
        if opts.shell_shorthand && !self.shell.hidden && !fail.shell.failed {
            self.shell.modify(general::shell(true, fail))
        } else {
            self.shell.modify(general::shell(false, fail))
        }

        if opts.uptime_shorthand && !self.uptime.hidden && !fail.shell.failed {
            self.uptime.modify(general::uptime(true, fail))
        } else {
            self.uptime.modify(general::uptime(false, fail))
        }
    }
    pub fn init_elements_for_debug(&mut self, fail: &mut Fail, opts: &Options) {
        self.uptime.modify(general::uptime(true, fail));
        self.desktop_env.modify(general::desktop_environment(fail));
        self.window_man.modify(general::window_manager(fail));
        self.uptime
            .modify(general::uptime(opts.uptime_shorthand, fail));
        self.shell
            .modify(general::shell(opts.shell_shorthand, fail));
        self.terminal.modify(general::terminal(fail));
        self.host.modify(format::host(fail));
        self.battery.modify(format::battery(fail));
        self.packages.modify(package::package_count(fail));
    }
}

trait Printing {
    fn print_host(&mut self, fail: &mut Fail);
    fn print_machine(&mut self);
    fn print_kernel_ver(&mut self);
    fn print_distribution(&mut self, fail: &Fail);
    fn print_desktop_env(&mut self, fail: &mut Fail);
    fn print_window_man(&mut self, fail: &mut Fail);
    fn print_package_count(&mut self, fail: &mut Fail);
    fn print_shell(&mut self, fail: &mut Fail);
    fn print_terminal(&mut self, fail: &mut Fail);
    fn print_processor(&mut self);
    fn print_uptime(&mut self, fail: &Fail);
    fn print_memory(&mut self);
    fn print_battery(&mut self, fail: &mut Fail);
    fn print_bar(&self, blocks: usize);
    fn print_palette(&self, opts: &Options);
}

impl Printing for Elements {
    fn print_host(&mut self, fail: &mut Fail) {
        if !self.host.hidden {
            self.host.modify(format::host(fail));
            if !fail.host.failed {
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
    }
    fn print_machine(&mut self) {
        if !self.machine.hidden {
            self.machine.modify(format::machine());
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
    fn print_distribution(&mut self, fail: &Fail) {
        if !self.distro.hidden {
            #[cfg(target_os = "linux")]
            self.distro.modify(general::distribution());
            if !fail.distro.failed {
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
    }
    fn print_desktop_env(&mut self, fail: &mut Fail) {
        if !self.desktop_env.hidden {
            self.desktop_env.modify(general::desktop_environment(fail));
            self.is_running_wm_only(fail, true);
            if !fail.desktop_env.failed {
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
    }
    fn print_window_man(&mut self, fail: &mut Fail) {
        if !self.window_man.hidden {
            self.window_man.modify(general::window_manager(fail));
            if !fail.window_man.failed {
                println!(
                    "{}{}{}{}{}{}",
                    self.format.padding,
                    self.window_man.key.color(self.format.color).bold(),
                    " ".repeat(self.calc_spacing(&self.window_man.key, &self.format.longest_key)),
                    self.format
                        .separator
                        .color(self.format.separator_color)
                        .bold(),
                    " ".repeat(self.format.spacing),
                    self.window_man.value
                );
            }
        }
    }
    fn print_kernel_ver(&mut self) {
        if !self.kernel.hidden {
            self.kernel.modify(format::kernel());
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
    fn print_package_count(&mut self, fail: &mut Fail) {
        if !self.packages.hidden {
            self.packages.modify(package::package_count(fail));
            if !fail.packages.failed {
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
    }
    fn print_shell(&mut self, fail: &mut Fail) {
        if !self.shell.hidden && !fail.shell.failed {
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
    fn print_terminal(&mut self, fail: &mut Fail) {
        if !self.terminal.hidden {
            self.terminal.modify(general::terminal(fail));
            if !fail.terminal.failed {
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
    }
    fn print_processor(&mut self) {
        if !self.cpu.hidden {
            self.cpu.modify(format::cpu());
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
    fn print_uptime(&mut self, fail: &Fail) {
        if !fail.uptime.failed && !self.uptime.hidden {
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
    fn print_memory(&mut self) {
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
                self.memory.modify(format::memory());
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
    fn print_battery(&mut self, fail: &mut Fail) {
        if !self.battery.hidden {
            self.battery.modify(format::battery(fail));
            if !fail.battery.failed {
                match self.format.bar {
                    true => {
                        print!(
                            "{}{}{}{}{}",
                            self.format.padding,
                            self.battery.key.color(self.format.color).bold(),
                            " ".repeat(
                                self.calc_spacing(&self.battery.key, &self.format.longest_key)
                            ),
                            self.format
                                .separator
                                .color(self.format.separator_color)
                                .bold(),
                            " ".repeat(self.format.spacing),
                        );
                        Printing::print_bar(self, bars::battery(fail));
                    }
                    false => {
                        println!(
                            "{}{}{}{}{}{}",
                            self.format.padding,
                            self.battery.key.color(self.format.color).bold(),
                            " ".repeat(
                                self.calc_spacing(&self.battery.key, &self.format.longest_key)
                            ),
                            self.format
                                .separator
                                .color(self.format.separator_color)
                                .bold(),
                            " ".repeat(self.format.spacing),
                            self.battery.value
                        );
                    }
                };
            }
        }
    }
    /// Print a bar next to memory and battery keys:
    /// it takes a function from the _bars crate_ as the first parameter
    /// and the color of the keys as a second
    fn print_bar(&self, blocks: usize) {
        match &self.format.color {
            Color::White => match blocks {
                10 => println!(
                    "{} {}{} {}",
                    self.format.bracket_open,
                    colored_blocks(self, blocks).color(self.format.color),
                    hidden_blocks(self, blocks).hidden(),
                    self.format.bracket_close
                ),
                _ => println!(
                    "{} {} {} {}",
                    self.format.bracket_open,
                    colored_blocks(self, blocks).color(self.format.color),
                    hidden_blocks(self, blocks).hidden(),
                    self.format.bracket_close
                ),
            },
            _ => match blocks {
                10 => println!(
                    "{} {}{} {}",
                    self.format.bracket_open,
                    colored_blocks(self, blocks).color(self.format.color),
                    colorless_blocks(self, blocks),
                    self.format.bracket_close
                ),
                _ => println!(
                    "{} {} {} {}",
                    self.format.bracket_open,
                    colored_blocks(self, blocks).color(self.format.color),
                    colorless_blocks(self, blocks),
                    self.format.bracket_close
                ),
            },
        }
    }
    /// Print a palette using the terminal's colorscheme
    fn print_palette(&self, opts: &Options) {
        if opts.palette_status {
            println!();
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
            println!();
        }
    }
}

/// Handles displaying each element (key and value pair) found in
/// __Elements__ struct, as well as the palette.
pub fn print_info(mut elems: Elements, opts: &Options, fail: &mut Fail) {
    elems.apply_shorthand_values(opts, fail);
    elems.print_host(fail);
    elems.print_machine();
    elems.print_kernel_ver();
    elems.print_distribution(fail);
    elems.print_desktop_env(fail);
    elems.print_window_man(fail);
    elems.print_package_count(fail);
    elems.print_shell(fail);
    elems.print_terminal(fail);
    elems.print_uptime(fail);
    elems.print_processor();
    elems.print_memory();
    elems.print_battery(fail);
    elems.print_palette(opts);
}

/// Debug allows users to see if everything during the extraction phase went okay.
pub fn debug(fail: &mut Fail) {
    fail.print_failed();
}

/// Hide an element or more e.g. package count, uptime etc. _(--hide <element>)_
pub fn hide(mut elems: Elements, options: Options, fail: &mut Fail, hide_parameters: Vec<&str>) {
    if hide_parameters.contains(&"host") {
        elems.host.hidden = true;
    }
    if hide_parameters.contains(&"mach") {
        elems.machine.hidden = true;
    }
    if hide_parameters.contains(&"distro") {
        elems.distro.hidden = true;
    }
    if hide_parameters.contains(&"de") {
        elems.desktop_env.hidden = true;
    }
    if hide_parameters.contains(&"wm") {
        elems.window_man.hidden = true;
    }
    if hide_parameters.contains(&"kernel") {
        elems.kernel.hidden = true;
    }
    if hide_parameters.contains(&"pkgs") {
        elems.packages.hidden = true;
    }
    if hide_parameters.contains(&"shell") {
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
    elems.set_longest_key(fail);
    print_info(elems, &options, fail);
}

/// Unhide an element or more e.g. package count, uptime etc. _(--hide-all-but <element>)_
pub fn unhide(mut elems: Elements, options: Options, fail: &mut Fail, hide_parameters: Vec<&str>) {
    if hide_parameters.contains(&"host") {
        elems.host.hidden = false;
    }
    if hide_parameters.contains(&"mach") {
        elems.machine.hidden = false;
    }
    if hide_parameters.contains(&"distro") {
        elems.distro.hidden = false;
    }
    if hide_parameters.contains(&"kernel") {
        elems.kernel.hidden = false;
    }
    if hide_parameters.contains(&"pkgs") {
        elems.packages.hidden = false;
    }
    if hide_parameters.contains(&"shell") {
        elems.shell.hidden = false;
    }
    if hide_parameters.contains(&"term") {
        elems.terminal.hidden = false;
    }
    if hide_parameters.contains(&"cpu") {
        elems.cpu.hidden = false;
    }
    if hide_parameters.contains(&"up") {
        elems.uptime.hidden = false;
    }
    if hide_parameters.contains(&"mem") {
        elems.memory.hidden = false;
    }
    if hide_parameters.contains(&"bat") {
        elems.battery.hidden = false;
    }
    if elems.is_running_wm_only(fail, false) {
        if hide_parameters.contains(&"de") {
            elems.desktop_env.hidden = true;
        }
        if hide_parameters.contains(&"wm") {
            elems.window_man.hidden = false;
        }
    } else {
        if hide_parameters.contains(&"de") {
            elems.desktop_env.hidden = false;
        }
        if hide_parameters.contains(&"wm") {
            elems.window_man.hidden = false;
        }
    }
    elems.set_longest_key(fail);
    print_info(elems, &options, fail);
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
    -d, --debug                     -   Print debug information
    -p, --palette                   -   Display palette
    -n, --no-color                  -   Disable colors
    -r, --random-color              -   Let Macchina pick a random color for you
    -c, --color <color>             -   Specify the key color
    -C, --separator-color <color>   -   Specify the separator color
    -t, --theme <theme>             -   Specify the theme to use
    -P, --padding <amount>          -   Specify the amount of left padding to use
    -s, --spacing <amount>          -   Specify the amount of spacing to use
    -b, --bar                       -   Display bars instead of values for battery and memory
    -S, --short-shell               -   Shorten shell output
    -U, --short-uptime              -   Shorten uptime output
    -H, --hide <element>            -   Hide the specified elements";
    let help_string: &str = "
    Coloring:
        Macchina's default key color is blue, to change the key color
        use \"--color / -c <color>\".
        Macchina's default separator color is white, to change the separator color
        use \"--separator-color / -C <color>\".
        To let Macchina pick a random color for you, use \"--random-color / -r\".
        Supported colors (case-sensitive):
            red, green, blue, magenta, cyan, yellow, black and white.
        
    Theming:
        Macchina comes with multiple themes out of the box,
        to change the default theme, use \"--theme / -t <theme>\".
        Supported themes (case-sensitive):
            def, alt and long.

    Hiding elements:
        To hide an element (or more), use \"--hide / -H <element>\"
        To display only the specified element (or more), use \"--show-only / -X <element>\" 
        Elements (case-sensitive):
            host, mach, kernel, distro, de, wm, pkgs, shell, term, cpu, up, mem, bat.
    
    If one of the keys e.g. kernel, uptime etc. fails to display, then Macchina couldn't
    fetch that piece of information, and therefore hides it from you.
    To see failing elements run: \"macchina --debug\".
    ";
    println!("{}\n{}\n", usage_string, help_string);
}

/// Return the correct amount of colored blocks: colored blocks are used blocks
pub fn colored_blocks(elems: &Elements, block_count: usize) -> String {
    let colored_blocks = elems.format.bar_glyph.repeat(block_count);
    colored_blocks
        .chars()
        .collect::<Vec<char>>()
        .chunks(1)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// Return the correct amount of colorless blocks: colorless blocks are unused blocks
pub fn colorless_blocks(elems: &Elements, block_count: usize) -> String {
    let colorless_blocks = elems.format.bar_glyph.repeat(10 - block_count);
    colorless_blocks
        .chars()
        .collect::<Vec<char>>()
        .chunks(1)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

// Used to correctly format the bars when using `--no-color` or `--color white`:
// Show the remaining unused blocks but they are hidden
pub fn hidden_blocks(elems: &Elements, block_count: usize) -> String {
    let colorless_blocks = elems.format.bar_glyph.repeat(10 - block_count);
    colorless_blocks
        .chars()
        .collect::<Vec<char>>()
        .chunks(1)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}
