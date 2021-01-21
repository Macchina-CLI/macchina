use colored::Colorize;
use crate::read;
use crate::format;

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
}

impl Elements {
    fn new() -> Elements {
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
            battery_key: "bat".to_string()
        }
    }
}

pub fn print_info(opts: Options) {
    let elems = Elements::new();
    let padding: String = " ".repeat(elems.left_padding);
    if opts.signal {
    match opts.color {
        true => {
            println!("{}{}{} {}", padding, elems.hostname_key.purple().bold(), elems.separator, read::read_hostname().unwrap());
            println!("{}{}{} {}", padding, elems.os_key.blue().bold(), elems.separator, read::read_operating_system());
            println!("{}{}{} {}", padding, elems.kernel_version_key.cyan().bold(), elems.separator, read::read_kernel_version().expect("Couldn't retrieve kernel version!"));
            println!("{}{}{} {}", padding, elems.terminal_key.green().bold(), elems.separator, read::read_terminal());
            println!("{}{}{} {}", padding, elems.shell_key.yellow().bold(), elems.separator, read::read_shell(opts.shell_shorthand));
            println!("{}{}{} {}{}", padding, elems.cpu_key.red().bold(), elems.separator, read::read_cpu_model_name(opts.cpu_shorthand), read::read_cpu_threads());
            println!("{}{}{} {}", padding, elems.uptime_key.purple().bold(), elems.separator, format::format_uptime(read::read_uptime().expect("Couldn't retrieve system uptime!")));
            println!("{}{}{} {}", padding, elems.battery_key.blue().bold(), elems.separator, format::format_battery());
        },
        false => {
            println!("{}{}{} {}", padding, elems.hostname_key, elems.separator, read::read_hostname().unwrap());
            println!("{}{}{} {}", padding, elems.os_key, elems.separator, read::read_operating_system());
            println!("{}{}{} {}", padding, elems.kernel_version_key, elems.separator, read::read_kernel_version().expect("Couldn't retrieve kernel version!"));
            println!("{}{}{} {}", padding, elems.terminal_key, elems.separator, read::read_terminal());
            println!("{}{}{} {}", padding, elems.shell_key, elems.separator, read::read_shell(opts.shell_shorthand));
            println!("{}{}{} {}{}", padding, elems.cpu_key, elems.separator, read::read_cpu_model_name(opts.cpu_shorthand), read::read_cpu_threads());
            println!("{}{}{} {}", padding, elems.uptime_key, elems.separator, format::format_uptime(read::read_uptime().expect("Couldn't retrieve system uptime!")));
            println!("{}{}{} {}", padding, elems.battery_key, elems.separator, format::format_battery());
        }
    }
    if opts.palette_status {
        palette(elems);
        println!();
    }
}
}
    
pub fn hide(options: Options, vector: Vec<String>) -> [u32; 9] {
        let mut elements: [u32; 9] = [1;9];
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

        for i in 0 .. 9 {
            if vector.contains(&labels[i]) {
                elements[i] = 0;
            }
        }

        print_info(options);
        elements
}
    
pub fn help(opts: Options) {
    match opts.color {
        true => {
            println!("  {}:","Macchina".blue().bold());
            println!("  Usage: macchina [options]\n  Options:\n  --help\n  --palette\n  --no-color\n  --hide (host, os, kern, etc.)\n  --short-sh : shorten shell value\n  --short-cpu : shorten cpu value\n\n  Options are case-sensitive.");
        },
        false => {
            println!("  Macchina");
            println!("  Usage: macchina [options]\n  Options:\n  --help\n  --palette\n  --no-color\n  --hide (host, os, kern, etc.)\n  --short-sh : shorten shell value\n  --short-cpu : shorten cpu value\n\n  Options are case-sensitive.");
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

pub fn error(inc_args: &Vec<String>) {
    eprintln!("  {}: bad option {:?}","Error".red().bold(), inc_args);
    println!("  Usage: macchina [options]\n  Options:\n  --help\n  --palette\n  --no-color\n  --hide (host, os, kern, ...)\n  --short-sh : shorten shell value\n  --short-cpu : shorten cpu value\n\n  Options are case-sensitive.");
}