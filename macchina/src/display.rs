use crate::theme::{HydrogenTheme, ReadoutKey};
use crate::{bars, format, theme::Theme, READOUTS};
use colored::{Color, Colorize};
use macchina_read::traits::{GeneralReadout, KernelReadout, PackageReadout};
use rand::Rng;
use std::fmt;

#[allow(dead_code)]
/// `FailedComponent` is an element that can fail to fetch e.g. host, kernel, battery, etc. \
/// An element that fails to fetch as well as its extraction method are printed to the \
/// terminal when `--debug` is present.
pub struct FailedComponent {
    failed: bool,
    pub extraction_method: String,
}

impl FailedComponent {
    fn new(f: bool, e: String) -> FailedComponent {
        FailedComponent {
            failed: f,
            extraction_method: e,
        }
    }

    pub fn fail_component(&mut self) {
        self.failed = true;
    }
}

/// `Fail` holds a number of `FailedComponent` fields:
/// - Host
/// - Kernel
/// - Uptime
/// - Battery
/// - Distribution, etc.
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
                -> Ignore if not running a desktop environment."),
            ),
            uptime: FailedComponent::new(
                false,
                String::from("(ERROR:DISABLED) Uptime -> Extracted from /proc/uptime"),
            ),
            battery: FailedComponent::new(
                false,
                String::from("(ERROR:DISABLED) Battery -> 
                (Linux) 
                Percentage extracted from /sys/class/power_supply/BATx/capacity
                Status extracted from /sys/class/power_supply/BATx/status
                (NetBSD) (ripgrep is required)
                Percentage extracted using envstat -d acpibat0 and rg (ripgrep)
                Status extracted using envstat -d acpibat0 and rg (ripgrep)
                (macOS)
                Percentage and status extracted from IOKit.
                -> Ignore if on a desktop computer.
                "),
            ),
            host: FailedComponent::new(
                false,
                String::from("(ERROR:DISABLED) Host -> 
                Hostname: Obtained from nix::unistd::gethostname()
                Username: Obtained from whoami"),
            ),
            shell: FailedComponent::new(
                false,
                String::from("(ERROR:DISABLED) Shell -> Extracted using \"ps -p $$ -o comm=\" OR \"ps -p $$ -o args=\""),
            ),
            terminal: FailedComponent::new(
                false,
                String::from("(ERROR:DISABLED) Terminal -> Extracted using \"ps -p $$ -p\""),
            ),
            packages: FailedComponent::new(
                false,
                String::from("(ERROR:DISABLED) Packages -> 
                (Arch-based distros) Extracted using \"pacman -Qq | wc -l\"
                (Debian-based distros) Extracted using \"dpkg -l | wc -l\"
                (Gentoo) Extracted using \"qlist -I | wc -l\"
                (NetBSD) Extracted using \"pkg_info | wc -l\"
                "),
            ),
            #[cfg(target_os = "linux")]
            distro: FailedComponent::new(
                false,
                String::from("(ERROR:DISABLED) Distribution -> Extracted from \"/etc/os-release\""),
            ),
            #[cfg(target_os = "netbsd")]
            distro: FailedComponent::new(
                true,
                String::from("(OK:DISABLED) Distribution -> NetBSD system detected, so the distribution is automatically hidden."),
            ),
            #[cfg(target_os = "macos")]
            distro: FailedComponent::new(
                true,
                String::from("(OK:DISABLED) Distribution -> macOS system detected, so the \
                distribution is automatically hidden."),
            ),
            #[cfg(target_os = "windows")]
            distro: FailedComponent::new(
                true,
                String::from("(OK:DISABLED) Distribution -> Windows system detected, so the \
                distribution is automatically hidden."),
            )
        }
    }
    pub fn count_print_failed(&self, failed_comp: &FailedComponent, mut num_fails: usize) -> usize {
        if failed_comp.failed {
            num_fails += 1;
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
/// can alter using the program's arguments like displaying
/// the palette and enabling shell and uptime shorthand.
pub struct Options {
    pub bar_status: bool,
    pub palette_status: bool,
    pub shell_shorthand: bool,
    pub uptime_shorthand: bool,
}

impl Options {
    pub fn new() -> Options {
        Options {
            bar_status: false,
            palette_status: false,
            shell_shorthand: false,
            uptime_shorthand: false,
        }
    }
}

/// A `Pair` is simply two fields: `value` and `visibility`
pub struct Pair {
    value: String,
    hidden: bool,
}

impl Pair {
    fn new() -> Pair {
        Pair {
            value: String::new(),
            hidden: false,
        }
    }

    fn modify(&mut self, val: Option<String>) {
        if let Some(value) = val {
            self.value = value;
        }
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// This struct encapsulates any element that is a `Pair`. \
/// it also contains miscellaneous fields such as the
/// key color, bar glyph, etc. which are part \
/// of the `Format` struct.
pub struct Elements {
    host: Pair,
    machine: Pair,
    kernel: Pair,
    distribution: Pair,
    operating_system: Pair,
    desktop_environment: Pair,
    window_manager: Pair,
    packages: Pair,
    shell: Pair,
    terminal: Pair,
    uptime: Pair,
    processor: Pair,
    memory: Pair,
    battery: Pair,
    pub theme: Box<dyn Theme>,
}

impl Elements {
    /// Initialize each pair of elements but only assign the pair's key, \
    /// as the value is assigned to an element when it is about to be printed.
    pub fn new() -> Elements {
        Elements {
            theme: HydrogenTheme::new(),
            host: Pair::new(),
            machine: Pair::new(),
            kernel: Pair::new(),
            distribution: Pair::new(),
            operating_system: Pair::new(),
            desktop_environment: Pair::new(),
            window_manager: Pair::new(),
            packages: Pair::new(),
            shell: Pair::new(),
            terminal: Pair::new(),
            uptime: Pair::new(),
            processor: Pair::new(),
            memory: Pair::new(),
            battery: Pair::new(),
        }
    }
    /// Determines which of the elements is the longest key to determine
    /// how to autospace them.
    pub fn longest_key(&mut self, fail: &mut Fail) -> String {
        let mut keys: Vec<String> = Vec::new();
        let abbrev = self.theme.default_abbreviation();
        if !self.host.hidden {
            keys.push(self.theme.key(ReadoutKey::Host, abbrev).to_string());
        }
        if !self.machine.hidden {
            keys.push(self.theme.key(ReadoutKey::Machine, abbrev).to_string());
        }
        if !self.kernel.hidden {
            keys.push(self.theme.key(ReadoutKey::Kernel, abbrev).to_string());
        }
        if !self.distribution.hidden {
            keys.push(self.theme.key(ReadoutKey::Distribution, abbrev).to_string());
        }
        if !self.packages.hidden {
            keys.push(self.theme.key(ReadoutKey::Packages, abbrev).to_string());
        }
        if !self.shell.hidden {
            keys.push(self.theme.key(ReadoutKey::Shell, abbrev).to_string());
        }
        if !self.terminal.hidden {
            keys.push(self.theme.key(ReadoutKey::Terminal, abbrev).to_string());
        }
        if !self.processor.hidden {
            keys.push(self.theme.key(ReadoutKey::Processor, abbrev).to_string());
        }
        if !self.uptime.hidden {
            keys.push(self.theme.key(ReadoutKey::Uptime, abbrev).to_string());
        }
        if !self.memory.hidden {
            keys.push(self.theme.key(ReadoutKey::Memory, abbrev).to_string());
        }
        if !self.battery.hidden {
            keys.push(self.theme.key(ReadoutKey::Battery, abbrev).to_string());
        }
        if let Some(true) = self.is_running_wm_only(fail, false) {
            keys.push(
                self.theme
                    .key(ReadoutKey::WindowManager, abbrev)
                    .to_string(),
            );
        } else {
            if !self.desktop_environment.hidden {
                keys.push(
                    self.theme
                        .key(ReadoutKey::DesktopEnvironment, abbrev)
                        .to_string(),
                );
            }
            if !self.window_manager.hidden {
                keys.push(
                    self.theme
                        .key(ReadoutKey::DesktopEnvironment, abbrev)
                        .to_string(),
                );
            }
        }
        let mut longest_key = keys[0].to_string();
        for val in keys {
            if val.len() > longest_key.len() {
                longest_key = val;
            }
        }
        longest_key
    }

    /// Returns the amount of spacing needed to properly center the `separator` across each line.
    pub fn calc_spacing(&self, current_key: &str) -> usize {
        (self.theme.misc().longest_key.len() + self.theme.misc().spacing) - current_key.len()
    }

    /// Hide every element.
    pub fn hide_all(&mut self) {
        self.host.hidden = true;
        self.machine.hidden = true;
        self.desktop_environment.hidden = true;
        self.window_manager.hidden = true;
        self.kernel.hidden = true;
        self.distribution.hidden = true;
        self.packages.hidden = true;
        self.shell.hidden = true;
        self.terminal.hidden = true;
        self.processor.hidden = true;
        self.uptime.hidden = true;
        self.memory.hidden = true;
        self.battery.hidden = true;
    }

    /// This function will assign an element its shorthand value if the
    /// user chooses to use an argument that enables this behavior.
    pub fn apply_shorthand_values(&mut self, opts: &Options, fail: &mut Fail) {
        let shell_shorthand = opts.shell_shorthand && !self.shell.hidden && !fail.shell.failed;
        let uptime_shorthand = opts.uptime_shorthand && !self.uptime.hidden && !fail.shell.failed;

        match READOUTS.general.shell(shell_shorthand) {
            Ok(shell) => self.shell.modify(Some(shell)),
            Err(_) => fail.shell.fail_component(),
        }

        match format::uptime(uptime_shorthand) {
            Ok(uptime) => self.uptime.modify(Some(uptime)),
            Err(_) => fail.uptime.fail_component(),
        }
    }

    /// Initialize each element its value for debugging purposes
    pub fn init_elements_for_debug(&mut self, fail: &mut Fail, opts: &Options) {
        match format::uptime(true) {
            Ok(uptime) => self.uptime.modify(Some(uptime)),
            Err(_) => fail.uptime.fail_component(),
        }

        match READOUTS.general.desktop_environment() {
            Ok(env) => self.desktop_environment.modify(Some(env)),
            Err(_) => fail.desktop_env.fail_component(),
        }

        match READOUTS.general.window_manager() {
            Ok(wm) => self.window_manager.modify(Some(wm)),
            Err(_) => fail.window_man.fail_component(),
        }

        self.is_running_wm_only(fail, true);

        match format::uptime(opts.uptime_shorthand) {
            Ok(uptime) => self.uptime.modify(Some(uptime)),
            Err(_) => fail.uptime.fail_component(),
        }

        match READOUTS.general.shell(opts.shell_shorthand) {
            Ok(shell) => self.shell.modify(Some(shell)),
            Err(_) => fail.shell.fail_component(),
        }

        match READOUTS.general.terminal() {
            Ok(terminal) => self.terminal.modify(Some(terminal)),
            Err(_) => fail.terminal.fail_component(),
        }

        match format::host() {
            Ok(host) => self.host.modify(Some(host)),
            Err(_) => fail.host.fail_component(),
        }

        match format::battery() {
            Ok(battery) => self.battery.modify(Some(battery)),
            Err(_) => fail.battery.fail_component(),
        }

        self.packages.modify(READOUTS.packages.count_pkgs().ok());
    }

    /// Check if the user is using only a Window Manager.
    pub fn is_running_wm_only(&self, fail: &mut Fail, apply: bool) -> Option<bool> {
        let window_manager = match READOUTS.general.window_manager() {
            Ok(wm) => wm,
            Err(_) => {
                fail.window_man.fail_component();
                return None;
            }
        };

        let desktop_env = match READOUTS.general.desktop_environment() {
            Ok(de) => de,
            Err(_) => {
                fail.desktop_env.fail_component();
                return None;
            }
        };

        if window_manager.to_uppercase() == desktop_env.to_uppercase() && apply {
            fail.desktop_env.fail_component();
            return Some(true);
        }

        Some(false)
    }

    pub fn set_theme(&mut self, theme: Box<dyn Theme>, fail: &mut Fail) {
        self.theme = theme;
        self.theme.misc_mut().longest_key = self.longest_key(fail);
    }
}

/// This trait contains functions whose purpose is to display elements found in the `Elements` struct.
/// Most elements go through two checks before finally being printed to the terminal:
/// - Confirming the element was not hidden using `--hide <element>`
/// - Confirming the element has not failed to fetch
///
/// # Example
/// ```
/// fn print_example(&mut self, fail: &mut Fail) {
///        // Exit the function if the element is hidden
///        if self.example.hidden {
///            return;
///        }
///        
///        // Fetch the element's value
///        // If an error occurs during this process then fail the element and exit
///        match format::example() {
///            Ok(host) => self.example.modify(Some(example)),
///            Err(_) => {
///                fail.example.fail_component();
///                return;
///            }
///        }
///
///        // Now it's time to print the key, separator and value
///        println!(
///        //...    
///        );
///    }
/// ```
trait Display {
    /// Print host information.
    fn print_host(&mut self, fail: &mut Fail);
    /// Print product information.
    fn print_machine(&mut self);
    /// Print kernel information.
    fn print_kernel(&mut self);
    /// Print operating system information.
    fn print_operating_system(&mut self);
    /// Print the distribution name.
    fn print_distribution(&mut self, fail: &mut Fail);
    /// Print the desktop environment name.
    fn print_desktop_environment(&mut self, fail: &mut Fail);
    /// Print the window manager name.
    fn print_window_manager(&mut self, fail: &mut Fail);
    /// Print the number of installed packages.
    fn print_package_count(&mut self, fail: &mut Fail);
    /// Print the shell name/path.
    fn print_shell(&mut self, fail: &Fail);
    /// Print the terminal name.
    fn print_terminal(&mut self, fail: &mut Fail);
    /// Print processor information.
    fn print_processor(&mut self);
    /// Print the computer's uptime.
    fn print_uptime(&mut self, fail: &Fail);
    /// Print memory usage.
    fn print_memory(&mut self, opts: &Options);
    /// Print battery information.
    fn print_battery(&mut self, opts: &Options, fail: &mut Fail);
    /// Print a bar for elements that support it.
    fn print_bar(&self, blocks: usize);
    /// Print an 8 color palette.
    fn print_palette(&self, opts: &Options);
}

impl Display for Elements {
    fn print_host(&mut self, fail: &mut Fail) {
        if self.host.hidden {
            return;
        }

        match format::host() {
            Ok(host) => self.host.modify(Some(host)),
            Err(_) => {
                fail.host.fail_component();
                return;
            }
        }

        println!(
            "{}{}{}{}{}{}",
            self.theme.padding(),
            self.theme.key_to_colored_string(ReadoutKey::Host),
            " ".repeat(
                self.calc_spacing(
                    &self
                        .theme
                        .key(ReadoutKey::Host, self.theme.default_abbreviation())
                )
            ),
            self.theme.misc().separator,
            self.theme.spacing(),
            self.host.value
        );
    }

    fn print_machine(&mut self) {
        if self.machine.hidden {
            return;
        }

        match READOUTS.general.machine() {
            Ok(machine) => self.machine.modify(Some(machine)),
            Err(_) => return,
        }

        println!(
            "{}{}{}{}{}{}",
            self.theme.padding(),
            self.theme.key_to_colored_string(ReadoutKey::Machine),
            " ".repeat(
                self.calc_spacing(
                    &self
                        .theme
                        .key(ReadoutKey::Machine, self.theme.default_abbreviation())
                )
            ),
            self.theme.misc().separator,
            self.theme.spacing(),
            self.machine.value
        );
    }

    fn print_kernel(&mut self) {
        if self.kernel.hidden {
            return;
        }

        match READOUTS.kernel.pretty_kernel() {
            Ok(kernel) => self.kernel.modify(Some(kernel)),
            Err(_) => return,
        }

        println!(
            "{}{}{}{}{}{}",
            self.theme.padding(),
            self.theme.key_to_colored_string(ReadoutKey::Kernel),
            " ".repeat(
                self.calc_spacing(
                    &self
                        .theme
                        .key(ReadoutKey::Kernel, self.theme.default_abbreviation())
                )
            ),
            self.theme.misc().separator,
            self.theme.spacing(),
            self.kernel.value
        );
    }

    fn print_operating_system(&mut self) {
        if self.operating_system.hidden {
            return;
        }

        match READOUTS.general.os_name() {
            Ok(os) => self.operating_system.modify(Some(os)),
            Err(_) => {
                return;
            }
        }

        println!(
            "{}{}{}{}{}{}",
            self.theme.padding(),
            self.theme
                .key_to_colored_string(ReadoutKey::OperatingSystem),
            " ".repeat(self.calc_spacing(&self.theme.key(
                ReadoutKey::OperatingSystem,
                self.theme.default_abbreviation()
            ),)),
            self.theme.misc().separator,
            self.theme.spacing(),
            self.operating_system.value
        );
    }

    fn print_distribution(&mut self, fail: &mut Fail) {
        if self.distribution.hidden {
            return;
        }

        match READOUTS.general.distribution() {
            Ok(dist) => self.distribution.modify(Some(dist)),
            Err(_) => {
                fail.distro.fail_component();
                return;
            }
        }

        println!(
            "{}{}{}{}{}{}",
            self.theme.padding(),
            self.theme.key_to_colored_string(ReadoutKey::Distribution),
            " ".repeat(
                self.calc_spacing(
                    &self
                        .theme
                        .key(ReadoutKey::Distribution, self.theme.default_abbreviation())
                )
            ),
            self.theme.misc().separator,
            self.theme.spacing(),
            self.distribution.value
        );
    }

    fn print_desktop_environment(&mut self, fail: &mut Fail) {
        if self.desktop_environment.hidden {
            return;
        }

        match READOUTS.general.desktop_environment() {
            Ok(env) => self.desktop_environment.modify(Some(env)),
            Err(_) => {
                fail.desktop_env.fail_component();
                return;
            }
        }

        if self.is_running_wm_only(fail, true) == None {
            return;
        }

        println!(
            "{}{}{}{}{}{}",
            self.theme.padding(),
            self.theme
                .key_to_colored_string(ReadoutKey::DesktopEnvironment),
            " ".repeat(self.calc_spacing(&self.theme.key(
                ReadoutKey::DesktopEnvironment,
                self.theme.default_abbreviation()
            ))),
            self.theme.misc().separator,
            self.theme.spacing(),
            self.desktop_environment.value
        );
    }

    fn print_window_manager(&mut self, fail: &mut Fail) {
        if self.window_manager.hidden {
            return;
        }

        match READOUTS.general.window_manager() {
            Ok(wm) => self.window_manager.modify(Some(wm)),
            Err(_) => {
                fail.window_man.fail_component();
                return;
            }
        }

        println!(
            "{}{}{}{}{}{}",
            self.theme.padding(),
            self.theme.key_to_colored_string(ReadoutKey::WindowManager),
            " ".repeat(
                self.calc_spacing(
                    &self
                        .theme
                        .key(ReadoutKey::WindowManager, self.theme.default_abbreviation())
                )
            ),
            self.theme.misc().separator,
            self.theme.spacing(),
            self.window_manager.value
        );
    }

    fn print_package_count(&mut self, fail: &mut Fail) {
        if self.packages.hidden {
            return;
        }

        match crate::READOUTS.packages.count_pkgs() {
            Ok(pc) => self.packages.modify(Some(pc)),
            Err(_) => {
                fail.packages.fail_component();
                return;
            }
        }

        println!(
            "{}{}{}{}{}{}",
            self.theme.padding(),
            self.theme.key_to_colored_string(ReadoutKey::Packages),
            " ".repeat(
                self.calc_spacing(
                    &self
                        .theme
                        .key(ReadoutKey::Packages, self.theme.default_abbreviation())
                )
            ),
            self.theme.misc().separator,
            self.theme.spacing(),
            self.packages.value
        );
    }

    fn print_shell(&mut self, fail: &Fail) {
        if self.shell.hidden || fail.shell.failed {
            return;
        }

        println!(
            "{}{}{}{}{}{}",
            self.theme.padding(),
            self.theme.key_to_colored_string(ReadoutKey::Shell),
            " ".repeat(
                self.calc_spacing(
                    &self
                        .theme
                        .key(ReadoutKey::Shell, self.theme.default_abbreviation())
                )
            ),
            self.theme.misc().separator,
            self.theme.spacing(),
            self.shell.value
        );
    }

    fn print_terminal(&mut self, fail: &mut Fail) {
        if self.terminal.hidden {
            return;
        }

        match READOUTS.general.terminal() {
            Ok(terminal) => self.terminal.modify(Some(terminal)),
            Err(_) => {
                fail.terminal.fail_component();
                return;
            }
        }

        println!(
            "{}{}{}{}{}{}",
            self.theme.padding(),
            self.theme.key_to_colored_string(ReadoutKey::Terminal),
            " ".repeat(
                self.calc_spacing(
                    &self
                        .theme
                        .key(ReadoutKey::Terminal, self.theme.default_abbreviation())
                )
            ),
            self.theme.misc().separator,
            self.theme.spacing(),
            self.terminal.value
        );
    }

    fn print_processor(&mut self) {
        if self.processor.hidden {
            return;
        }

        match format::cpu() {
            Ok(cpu) => self.processor.modify(Some(cpu)),
            Err(_) => self.processor.modify(Some(String::from("Unknown"))),
        }

        println!(
            "{}{}{}{}{}{}",
            self.theme.padding(),
            self.theme.key_to_colored_string(ReadoutKey::Processor),
            " ".repeat(
                self.calc_spacing(
                    &self
                        .theme
                        .key(ReadoutKey::Processor, self.theme.default_abbreviation())
                )
            ),
            self.theme.misc().separator,
            self.theme.spacing(),
            self.processor.value
        );
    }

    fn print_uptime(&mut self, fail: &Fail) {
        if self.uptime.hidden || fail.uptime.failed {
            return;
        }

        println!(
            "{}{}{}{}{}{}",
            self.theme.padding(),
            self.theme.key_to_colored_string(ReadoutKey::Uptime),
            " ".repeat(
                self.calc_spacing(
                    &self
                        .theme
                        .key(ReadoutKey::Uptime, self.theme.default_abbreviation())
                )
            ),
            self.theme.misc().separator,
            self.theme.spacing(),
            self.uptime.value
        );
    }

    fn print_memory(&mut self, opts: &Options) {
        if self.memory.hidden {
            return;
        }

        if opts.bar_status {
            match bars::memory() {
                Ok(mem) => self.memory.modify(Some(mem.to_string())),
                Err(_) => self.memory.modify(Some(String::from("0"))),
            }
        } else {
            match format::memory() {
                Ok(mem) => self.memory.modify(Some(mem)),
                Err(_) => self.memory.modify(Some(String::from("Unknown"))),
            }
        }

        if opts.bar_status {
            print!(
                "{}{}{}{}{}",
                self.theme.padding(),
                self.theme.key_to_colored_string(ReadoutKey::Memory),
                " ".repeat(
                    self.calc_spacing(
                        &self
                            .theme
                            .key(ReadoutKey::Memory, self.theme.default_abbreviation())
                    )
                ),
                self.theme.misc().separator,
                self.theme.spacing(),
            );
            Self::print_bar(self, self.memory.value.parse().unwrap());
        } else {
            println!(
                "{}{}{}{}{}{}",
                self.theme.padding(),
                self.theme.key_to_colored_string(ReadoutKey::Memory),
                " ".repeat(
                    self.calc_spacing(
                        &self
                            .theme
                            .key(ReadoutKey::Memory, self.theme.default_abbreviation())
                    )
                ),
                self.theme.misc().separator,
                self.theme.spacing(),
                self.memory.value
            );
        }
    }

    fn print_battery(&mut self, opts: &Options, fail: &mut Fail) {
        if self.battery.hidden {
            return;
        }

        match format::battery() {
            Ok(bat) => self.battery.modify(Some(bat)),
            Err(_) => {
                fail.battery.fail_component();
                return;
            }
        }

        if opts.bar_status {
            print!(
                "{}{}{}{}{}",
                self.theme.padding(),
                self.theme.key_to_colored_string(ReadoutKey::Battery),
                " ".repeat(
                    self.calc_spacing(
                        &self
                            .theme
                            .key(ReadoutKey::Battery, self.theme.default_abbreviation())
                    )
                ),
                self.theme.misc().separator,
                self.theme.spacing(),
            );

            Self::print_bar(self, bars::battery().unwrap_or(0));
        } else {
            println!(
                "{}{}{}{}{}{}",
                self.theme.padding(),
                self.theme.key_to_colored_string(ReadoutKey::Battery),
                " ".repeat(
                    self.calc_spacing(
                        &self
                            .theme
                            .key(ReadoutKey::Battery, self.theme.default_abbreviation())
                    )
                ),
                self.theme.misc().separator,
                self.theme.spacing(),
                self.battery.value
            );
        }
    }

    fn print_bar(&self, blocks: usize) {
        match self.theme.misc().color {
            Color::White => match blocks {
                10 => println!(
                    "{} {}{} {}",
                    self.theme.bar().symbol_open,
                    colored_glyphs(self, blocks).color(self.theme.misc().color),
                    colorless_glyphs(self, blocks).hidden(),
                    self.theme.bar().symbol_close,
                ),
                _ => println!(
                    "{} {} {} {}",
                    self.theme.bar().symbol_open,
                    colored_glyphs(self, blocks).color(self.theme.misc().color),
                    colorless_glyphs(self, blocks).hidden(),
                    self.theme.bar().symbol_close,
                ),
            },
            _ => match blocks {
                10 => println!(
                    "{} {}{} {}",
                    self.theme.bar().symbol_open,
                    colored_glyphs(self, blocks).color(self.theme.misc().color),
                    colorless_glyphs(self, blocks),
                    self.theme.bar().symbol_close,
                ),
                _ => println!(
                    "{} {} {} {}",
                    self.theme.bar().symbol_open,
                    colored_glyphs(self, blocks).color(self.theme.misc().color),
                    colorless_glyphs(self, blocks),
                    self.theme.bar().symbol_close,
                ),
            },
        }
    }

    fn print_palette(&self, opts: &Options) {
        if opts.palette_status {
            println!();
            println!(
                "{}{}{}{}{}{}{}{}{}",
                self.theme.padding(),
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

/// Calls all print functions found in the `Printing` trait
pub fn print_info(mut elems: Elements, opts: &Options, fail: &mut Fail) {
    elems.apply_shorthand_values(opts, fail);
    elems.print_host(fail);
    elems.print_machine();
    elems.print_kernel();
    elems.print_operating_system();
    elems.print_distribution(fail);
    elems.print_desktop_environment(fail);
    elems.print_window_manager(fail);
    elems.print_package_count(fail);
    elems.print_shell(fail);
    elems.print_terminal(fail);
    elems.print_uptime(fail);
    elems.print_processor();
    elems.print_memory(opts);
    elems.print_battery(opts, fail);
    elems.print_palette(opts);
}

/// List elements that failed to fetch when `--debug` is present.
pub fn debug(fail: &mut Fail) {
    fail.print_failed();
}

/// Hide one or more elements e.g. package count, uptime etc. when `--hide <element>` is present.
pub fn hide(mut elems: Elements, options: Options, fail: &mut Fail, hide_parameters: Vec<&str>) {
    // We hide the keys the user asked to hide
    elems.host.hidden = hide_parameters.contains(&"host");
    elems.machine.hidden = hide_parameters.contains(&"mach");
    elems.distribution.hidden = hide_parameters.contains(&"distro");
    elems.operating_system.hidden = !hide_parameters.contains(&"os");
    elems.desktop_environment.hidden = hide_parameters.contains(&"de");
    elems.window_manager.hidden = hide_parameters.contains(&"wm");
    elems.kernel.hidden = hide_parameters.contains(&"kernel");
    elems.packages.hidden = hide_parameters.contains(&"pkgs");
    elems.shell.hidden = hide_parameters.contains(&"shell");
    elems.terminal.hidden = hide_parameters.contains(&"term");
    elems.processor.hidden = hide_parameters.contains(&"cpu");
    elems.uptime.hidden = hide_parameters.contains(&"up");
    elems.memory.hidden = hide_parameters.contains(&"mem");
    elems.battery.hidden = hide_parameters.contains(&"bat");

    // We don't know which keys the user has allowed to show, so we reset the longest key
    elems.theme.misc_mut().longest_key = elems.longest_key(fail);
    // Print everything
    print_info(elems, &options, fail);
}

/// Print only the specified elements e.g. package count, uptime etc. when `--show-only <element>` is present.
pub fn unhide(mut elems: Elements, options: Options, fail: &mut Fail, hide_parameters: Vec<&str>) {
    // We unhide the keys the user asked to show
    elems.host.hidden = !hide_parameters.contains(&"host");
    elems.machine.hidden = !hide_parameters.contains(&"mach");
    elems.distribution.hidden = !hide_parameters.contains(&"distro");
    elems.operating_system.hidden = !hide_parameters.contains(&"os");
    elems.kernel.hidden = !hide_parameters.contains(&"kernel");
    elems.packages.hidden = !hide_parameters.contains(&"pkgs");
    elems.shell.hidden = !hide_parameters.contains(&"shell");
    elems.terminal.hidden = !hide_parameters.contains(&"term");
    elems.processor.hidden = !hide_parameters.contains(&"cpu");
    elems.uptime.hidden = !hide_parameters.contains(&"up");
    elems.memory.hidden = !hide_parameters.contains(&"mem");
    elems.battery.hidden = !hide_parameters.contains(&"bat");

    if let Some(true) = elems.is_running_wm_only(fail, false) {
        elems.desktop_environment.hidden = hide_parameters.contains(&"de");
        elems.window_manager.hidden = !hide_parameters.contains(&"wm");
    } else {
        elems.desktop_environment.hidden = !hide_parameters.contains(&"de");
        elems.window_manager.hidden = !hide_parameters.contains(&"wm");
    }

    // We don't know which keys the user has allowed to show, so we reset the longest key
    elems.theme.misc_mut().longest_key = elems.longest_key(fail);
    print_info(elems, &options, fail);
}

/// Convert arguments passed to `--color` to their respective color.
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

/// Pick a random color for the keys when `--random-color` is present.
pub fn randomize_color() -> Color {
    let mut rng = rand::thread_rng();
    let rand: usize = rng.gen_range(0..8);
    match rand {
        0 => Color::Red,
        1 => Color::Green,
        2 => Color::Blue,
        3 => Color::Magenta,
        4 => Color::Yellow,
        5 => Color::Cyan,
        6 => Color::Black,
        _ => Color::White,
    }
}

/// Print usage and help text.
pub fn help() {
    let usage_string: &str = "
    USAGE: macchina [OPTIONS]
    OPTIONS:
    -h, --help                      -   Print help text
    -v, --version                   -   Print Macchina's version
    -d, --debug                     -   Print debug information
    -p, --palette                   -   Display palette
    -n, --no-color                  -   Disable colors
    -r, --random-color              -   Picks a random key color for you
    -R, --random-sep-color          -   Picks a random separator color for you
    -c, --color <color>             -   Specify the key color
    -C, --separator-color <color>   -   Specify the separator color
    -t, --theme <theme>             -   Specify the theme to use
    -P, --padding <amount>          -   Specify the amount of left padding to use
    -s, --spacing <amount>          -   Specify the amount of spacing to use
    -b, --bar                       -   Display bars instead of values for battery and memory
    -S, --short-shell               -   Shorten shell output
    -U, --short-uptime              -   Shorten uptime output
    -H, --hide <element>            -   Hide the specified elements
    -X, --show-only <element>       -   Display only the specified elements";
    let help_string: &str = "
    Coloring:
        Macchina's default key color is blue, to change the key color
        use \"--color / -c <color>\"
        Macchina's default separator color is white, to change the separator color
        use \"--separator-color / -C <color>\"
        To let Macchina pick a random color for you, use \"--random-color / -r\"
        Supported colors (case-sensitive):
            red, green, blue, magenta, cyan, yellow, black and white.
        
    Theming:
        Macchina comes with multiple themes out of the box,
        to change the default theme, use \"--theme / -t <theme>\"
        Supported themes (case-sensitive):
            H, He, Li.

    Hiding elements:
        To hide an element (or more), use \"--hide / -H <element>\"
        To display only the specified element (or more), use \"--show-only / -X <element>\" 
        Elements (case-sensitive):
            host, mach, kernel, os, distro, de, wm, pkgs, shell, term, cpu, up, mem and bat.
    
    If an element e.g. kernel, uptime etc. fails to display, then Macchina couldn't
    fetch that piece of information, and therefore hides it from you.
    To see failing elements run: \"macchina --debug\"";
    println!("{}\n{}\n", usage_string, help_string);
}

/// Return the correct amount of colored blocks: colored blocks are used blocks.
///
pub fn colored_glyphs(elems: &Elements, block_count: usize) -> String {
    let colored_glyphs = elems.theme.bar().glyph.repeat(block_count);
    colored_glyphs
        .chars()
        .collect::<Vec<char>>()
        .chunks(1)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// Return the correct amount of colorless blocks: colorless blocks are unused blocks.
pub fn colorless_glyphs(elems: &Elements, block_count: usize) -> String {
    let colorless_glyphs = elems.theme.bar().glyph.repeat(10 - block_count);
    colorless_glyphs
        .chars()
        .collect::<Vec<char>>()
        .chunks(1)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}
