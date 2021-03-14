#![allow(dead_code)]
use colored::{Color, ColoredString, Colorize};
/// Themes control the program's visuals, the visuals range from the color choice to the way keys are named
pub struct Theme {
    pub keys: Keys,
    pub bar: Bar,
    pub misc: Misc,
}

impl Theme {
    // Themes are named after chemical elements
    // 1 Hydrogen (H)
    // Usage: macchina --theme H
    pub fn hydrogen() -> Theme {
        Theme {
            keys: Keys::classic(),
            bar: Bar::rounded(),
            misc: Misc::dash(),
        }
    }
    // 2 Helium (He)
    // Usage: macchina --theme He
    pub fn helium() -> Theme {
        Theme {
            keys: Keys::alternative(),
            bar: Bar::squared(),
            misc: Misc::arrow(),
        }
    }
    // 3 Lithium (Li)
    // Usage: macchina --theme Li
    pub fn lithium() -> Theme {
        Theme {
            keys: Keys::long(),
            bar: Bar::angled(),
            misc: Misc::squiggly(),
        }
    }
}

pub struct Misc {
    separator: &'static str,
    pub separator_color: Color,
    pub color: Color,
    pub padding: usize,
    pub spacing: usize,
    pub longest_key: String,
}

impl Misc {
    fn dash() -> Misc {
        Misc {
            color: Color::Red,
            separator_color: Color::White,
            separator: "—",
            spacing: 2,
            padding: 4,
            longest_key: String::new(),
        }
    }
    fn arrow() -> Misc {
        Misc {
            color: Color::Green,
            separator_color: Color::White,
            separator: "=>",
            spacing: 2,
            padding: 4,
            longest_key: String::new(),
        }
    }
    fn squiggly() -> Misc {
        Misc {
            color: Color::Yellow,
            separator_color: Color::White,
            separator: "~",
            spacing: 2,
            padding: 4,
            longest_key: String::new(),
        }
    }
}

pub struct Bar {
    pub glyph: &'static str,
    pub symbol_open: char,
    pub symbol_close: char,
}

impl Bar {
    fn squared() -> Bar {
        Bar {
            glyph: "■",
            symbol_open: '[',
            symbol_close: ']',
        }
    }
    fn rounded() -> Bar {
        Bar {
            glyph: "●",
            symbol_open: '(',
            symbol_close: ')',
        }
    }
    fn angled() -> Bar {
        Bar {
            glyph: "×",
            symbol_open: '<',
            symbol_close: '>',
        }
    }
}

pub struct Keys {
    pub host: &'static str,
    pub machine: &'static str,
    pub kernel: &'static str,
    pub distribution: &'static str,
    pub operating_system: &'static str,
    pub desktop_environment: &'static str,
    pub window_manager: &'static str,
    pub packages: &'static str,
    pub shell: &'static str,
    pub terminal: &'static str,
    pub uptime: &'static str,
    pub processor: &'static str,
    pub memory: &'static str,
    pub battery: &'static str,
}

impl Keys {
    fn classic() -> Keys {
        Keys {
            host: "Host",
            machine: "Machine",
            kernel: "Kernel",
            distribution: "Distro",
            operating_system: "OS",
            desktop_environment: "DE",
            window_manager: "WM",
            packages: "Packages",
            shell: "Shell",
            terminal: "Terminal",
            uptime: "Uptime",
            processor: "CPU",
            memory: "Memory",
            battery: "Battery",
        }
    }
    fn alternative() -> Keys {
        Keys {
            host: "Hos",
            machine: "Mac",
            kernel: "Ker",
            distribution: "Dis",
            operating_system: "Ope",
            desktop_environment: "Des",
            window_manager: "Win",
            packages: "Pac",
            shell: "She",
            terminal: "Ter",
            uptime: "Upt",
            processor: "Cpu",
            memory: "Mem",
            battery: "Bat",
        }
    }
    fn long() -> Keys {
        Keys {
            host: "Host",
            machine: "Machine",
            kernel: "Kernel",
            distribution: "Distribution",
            operating_system: "Operating System",
            desktop_environment: "Desktop Environment",
            window_manager: "Window Manager",
            packages: "Packages",
            shell: "Shell",
            terminal: "Terminal",
            uptime: "Uptime",
            processor: "Processor",
            memory: "Memory",
            battery: "Battery",
        }
    }
}

pub trait Printing {
    /// Prints the host key
    fn host(&self) -> ColoredString;
    /// Prints the machine key
    fn machine(&self) -> ColoredString;
    /// Prints the kernel key
    fn kernel(&self) -> ColoredString;
    /// Prints the distro key
    fn distribution(&self) -> ColoredString;
    /// Prints the operating system key
    fn operating_system(&self) -> ColoredString;
    /// Prints the desktop environment key
    fn desktop_environment(&self) -> ColoredString;
    /// Prints the window manager key
    fn window_manager(&self) -> ColoredString;
    /// Prints the packages key
    fn packages(&self) -> ColoredString;
    /// Prints the shell key
    fn shell(&self) -> ColoredString;
    /// Prints the terminal key
    fn terminal(&self) -> ColoredString;
    /// Prints the uptime key
    fn uptime(&self) -> ColoredString;
    /// Prints the processor key
    fn processor(&self) -> ColoredString;
    /// Prints the memory key
    fn memory(&self) -> ColoredString;
    /// Prints the battery key
    fn battery(&self) -> ColoredString;
    /// Prints the separator
    fn separator(&self) -> ColoredString;
    /// Prints the padding
    fn padding(&self) -> String;
    /// Prints the spacing
    fn spacing(&self) -> String;
}

impl Printing for Theme {
    fn host(&self) -> ColoredString {
        self.keys.host.to_string().color(self.misc.color).bold()
    }
    fn machine(&self) -> ColoredString {
        self.keys.machine.to_string().color(self.misc.color).bold()
    }
    fn kernel(&self) -> ColoredString {
        self.keys.kernel.to_string().color(self.misc.color).bold()
    }
    fn distribution(&self) -> ColoredString {
        self.keys
            .distribution
            .to_string()
            .color(self.misc.color)
            .bold()
    }
    fn operating_system(&self) -> ColoredString {
        self.keys
            .operating_system
            .to_string()
            .color(self.misc.color)
            .bold()
    }
    fn desktop_environment(&self) -> ColoredString {
        self.keys
            .desktop_environment
            .to_string()
            .color(self.misc.color)
            .bold()
    }
    fn window_manager(&self) -> ColoredString {
        self.keys
            .window_manager
            .to_string()
            .color(self.misc.color)
            .bold()
    }
    fn packages(&self) -> ColoredString {
        self.keys.packages.to_string().color(self.misc.color).bold()
    }
    fn shell(&self) -> ColoredString {
        self.keys.shell.to_string().color(self.misc.color).bold()
    }
    fn terminal(&self) -> ColoredString {
        self.keys.terminal.to_string().color(self.misc.color).bold()
    }
    fn processor(&self) -> ColoredString {
        self.keys
            .processor
            .to_string()
            .color(self.misc.color)
            .bold()
    }
    fn uptime(&self) -> ColoredString {
        self.keys.uptime.to_string().color(self.misc.color).bold()
    }
    fn memory(&self) -> ColoredString {
        self.keys.memory.to_string().color(self.misc.color).bold()
    }
    fn battery(&self) -> ColoredString {
        self.keys.battery.to_string().color(self.misc.color).bold()
    }
    fn separator(&self) -> ColoredString {
        self.misc.separator.color(self.misc.separator_color).bold()
    }
    fn spacing(&self) -> String {
        " ".repeat(self.misc.spacing)
    }
    fn padding(&self) -> String {
        " ".repeat(self.misc.padding)
    }
}
