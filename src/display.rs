use colored::Colorize;
use crate::read;
use crate::format;

pub struct Options {
    pub color: bool,
    pub palette_status: bool,
    pub icons: bool,
    pub signal: bool,
    pub cpu_shorthand: bool,
    pub shell_shorthand: bool
}

impl Options {
    pub fn new(col: bool, pal: bool, ico: bool, sig: bool, cpu_short: bool, shell_short: bool) -> Options {
        Options { color: col, palette_status: pal, icons: ico, signal: sig, cpu_shorthand: cpu_short, shell_shorthand: shell_short }
    }
}

pub fn print(options: Options, hide: &[u32]) {
    if options.signal {
    //  ----------------- CUSTOMIZABLE ------------------ \\
    //  
    //  left_padding:
    //  Change value to however many spaces you 
    //  want Macchina to display to the left of text
    let left_padding = 6;
    //  shell_shorthand:
    //  If set to "true", Macchina will display full path to shell binary, e.g: /usr/bin/zsh
    //  If set to "false", Macchina will display shell name, e.g: zsh

    // This set of variables are the labels displayed
    // to the left of each system information Macchina reports
    // Change any x_key value to whatever your want
    // Change the colors to whatever you want
    // Example:
    //      Changing uptime_key value from "up" to "uptime"
    //      will tell Macchina to print uptime instead of up
    //      when displaying system information
        let separator = if options.icons { ' ' } else { ':' };
        let hostname_key = String::from("host").purple().bold();
        let os_key = String::from("os").blue().bold();
        let osrelease_key = String::from("kern").cyan().bold();
        let terminal_key = String::from("term").green().bold();
        let shell_key = String::from("sh").yellow().bold();
        let cpu_model_name_key = String::from("cpu").red().bold();
        let uptime_key = String::from("up").purple().bold();
        let battery_key = String::from("bat").blue().bold();
        
        
    //  ------------------------------------------------- \\
    
    //  This line does the padding calculation 
    //  based on the value of left_padding
    let padding = " ".repeat(left_padding);
    

    match options.color {
        true => {
            match options.icons {
                true => {
                    let (hostname_icon, os_icon, terminal_icon, uptime_icon, osrelease_icon, cpu_icon, shell_icon, battery_icon): 
                    (char, char, char, char, char, char, char, char);
                    hostname_icon = '';
                    terminal_icon = '';
                    uptime_icon = '神';
                    osrelease_icon = '';
                    cpu_icon = '';
                    shell_icon = '';
                    os_icon = pick_icon_for_os();
                    battery_icon = pick_icon_for_battery();
                    if hide[0] != 0 { println!("{}{}  {}", padding, hostname_icon, read::read_hostname().purple().bold()); }
                    if hide[1] != 0 { println!("{}{}  {}", padding, os_icon, read::read_operating_system().blue().bold()); }
                    if hide[2] != 0 { println!("{}{}  {}", padding, osrelease_icon, read::read_kernel_version().green().bold()); }
                    if hide[3] != 0 { println!("{}{}  {}", padding, terminal_icon, read::read_terminal().cyan().bold()); }
                    if hide[4] != 0 { println!("{}{}  {}", padding, shell_icon, read::read_shell(options.shell_shorthand).yellow().bold()); }
                    if hide[5] != 0 { println!("{}{}  {}{}", padding, cpu_icon, read::read_cpu_model_name(options.cpu_shorthand).red().bold(), read::read_cpu_threads().red().bold()); }
                    if hide[6] != 0 { println!("{}{} {}", padding, uptime_icon, format::format_uptime(read::read_uptime()).purple().bold()); }
                    if hide[7] != 0 { println!("{}{}  {}", padding, battery_icon, format::format_battery().blue().bold()); }
                },
                _ => {
                    if hide[0] != 0 { println!("{}{}{} {}", padding, hostname_key, separator, read::read_hostname()); }
                    if hide[1] != 0 { println!("{}{}{}   {}", padding, os_key, separator, read::read_operating_system()); }
                    if hide[2] != 0 { println!("{}{}{} {}", padding, osrelease_key, separator, read::read_kernel_version()); }
                    if hide[3] != 0 { println!("{}{}{} {}", padding, terminal_key, separator, read::read_terminal()); }
                    if hide[4] != 0 { println!("{}{}{}   {}", padding, shell_key, separator, read::read_shell(options.shell_shorthand)); }
                    if hide[5] != 0 { println!("{}{}{}  {}{}", padding, cpu_model_name_key, separator, read::read_cpu_model_name(options.cpu_shorthand), read::read_cpu_threads()); }
                    if hide[6] != 0 { println!("{}{}{}   {}", padding, uptime_key, separator, format::format_uptime(read::read_uptime())); }
                    if hide[7] != 0 { println!("{}{}{}  {}", padding, battery_key, separator, format::format_battery()); }
                }
            }   
        },
        false => {
            match options.icons {
                true => {
                    let (hostname_icon,
                        os_icon, 
                        terminal_icon, 
                        uptime_icon,
                        osrelease_icon,
                        cpu_icon,
                        shell_icon,
                        battery_icon): (char, char, char, char, char, char, char, char);
                    hostname_icon = '';
                    terminal_icon = '';
                    uptime_icon = '神';
                    shell_icon = '';
                    cpu_icon = '';
                    osrelease_icon = '';
                    os_icon = pick_icon_for_os();
                    battery_icon = pick_icon_for_battery();
                    
                    if hide[0] != 0 { println!("{}{}  {}", padding, hostname_icon, read::read_hostname()); }
                    if hide[1] != 0 { println!("{}{}  {}", padding, os_icon, read::read_operating_system()); }
                    if hide[2] != 0 { println!("{}{}  {}", padding, osrelease_icon, read::read_kernel_version()); }
                    if hide[3] != 0 { println!("{}{}  {}", padding, terminal_icon, read::read_terminal()); }
                    if hide[4] != 0 { println!("{}{}  {}", padding, shell_icon, read::read_shell(options.shell_shorthand)); }
                    if hide[5] != 0 { println!("{}{}  {}{}", padding, cpu_icon, read::read_cpu_model_name(options.cpu_shorthand), read::read_cpu_threads()); }
                    if hide[6] != 0 { println!("{}{}  {}", padding, uptime_icon, format::format_uptime(read::read_uptime())); }
                    if hide[7] != 0 { println!("{}{}  {}", padding, battery_icon, format::format_battery()); }
                },
                _ => {
                    if hide[0] != 0 { println!("{}{}{} {}", padding, hostname_key, separator, read::read_hostname()); }
                    if hide[1] != 0 { println!("{}{}{}   {}", padding, os_key, separator, read::read_operating_system()); }
                    if hide[2] != 0 { println!("{}{}{} {}", padding, osrelease_key, separator, read::read_kernel_version()); }
                    if hide[3] != 0 { println!("{}{}{} {}", padding, terminal_key, separator, read::read_terminal()); }
                    if hide[4] != 0 { println!("{}{}{}   {}", padding, shell_key, separator, read::read_shell(options.shell_shorthand)); }
                    if hide[5] != 0 { println!("{}{}{}  {}{}", padding, cpu_model_name_key, separator, read::read_cpu_model_name(options.cpu_shorthand), read::read_cpu_threads()); }
                    if hide[6] != 0 { println!("{}{}{}   {}", padding, uptime_key, separator, format::format_uptime(read::read_uptime())); }
                    if hide[7] != 0 { println!("{}{}{}  {}", padding, battery_key, separator, format::format_battery()); }
                }
            }
        }
    };

    if options.palette_status && hide[8] != 0 {
        palette(left_padding);
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

        print(options, &elements);
        return elements;
}
    
pub fn help(color: bool) {
        if color {
            println!("  {}:","Macchina".blue().bold());
            println!("  Usage: macchina [options]\n
            Options:
            --help
            --palette
            --no-color
            --hide (host, os, kern, ...)
            --short-sh : shorten shell value
            --short-cpu : shorten cpu value\n
            Options are case-sensitive.");
        }
        else
        {
            println!("  Macchina");
            println!("  Usage: macchina [option]
            Options:
            --help
            --palette
            --no-color
            --hide (host, os, kern, ...)
            --short-sh : shorten shell value
            --short-cpu : shorten cpu value\n
            Options are case-sensitive.");
        }
}

pub fn palette(left_padding: usize) {
    let padding: String = " ".repeat(left_padding);
    // The way this works is by setting the background color 
    // of 3 consecutive spaces to achieve a 'block' of color
    // This is done for every color supported by the terminal and 
    // the colors can change depending on the colorscheme of the terminal 
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

pub fn pick_icon_for_os() -> char {
    //  Macchina will pick an icon for your OS 
    //  based on NAME value stored in /etc/os-release
    if read::read_operating_system() == "Ubuntu".to_string() { 
        return '';
    }
    else if read::read_operating_system() == "Arch Linux".to_string() { 
        return ''; 
    }
    else if read::read_operating_system() == "Fedora".to_string() {
        return '';
    }
    else if read::read_operating_system() == "elementary OS".to_string() {
        return '';
    }
    else if read::read_operating_system() == "Gentoo".to_string() {
        return '';
    }
    else if read::read_operating_system() == "Linux Mint".to_string() {
        return '';
    }
    return '';
}

pub fn pick_icon_for_battery() -> char {
    let bat_perc: i32 = read::read_battery_percentage().parse::<i32>().unwrap();
    //  Macchina will pick an icon to display next to your
    //  battery stats based on the battery percentage
    if bat_perc <= 10 { 
        return '';
    }
    else if bat_perc >= 10 && bat_perc <= 20 { 
        return ''; 
    }
    else if bat_perc >= 20 && bat_perc <= 30 {
        return '';
    }
    else if bat_perc >= 30 && bat_perc <= 40 {
        return '';
    }
    else if bat_perc >= 40 && bat_perc <= 50 {
        return '';
    }
    else if bat_perc >= 50 && bat_perc <= 60 {
        return '';
    }
    else if bat_perc >= 60 && bat_perc <= 70 {
        return '';
    }
    else if bat_perc >= 70 && bat_perc <= 80 {
        return '';
    }
    else if bat_perc >= 80 && bat_perc <= 90 {
        return '';
    }
    return '';
}

#[allow(dead_code)]
pub fn error(vector: Vec<String>) {
    let args: [String; 4] = [ "--help".to_string(), "--palette".to_string(), "--no-color".to_string(), "--hide".to_string()];
    let mut incorrect_args: Vec<String> = Vec::new();
        for i in 0 .. vector.len() {
            if !args.contains(&vector[i]) {
                incorrect_args.push(vector[i].clone());
            }
        }
    eprintln!("  {}: bad option {:?} {}","Error".red().bold(),incorrect_args, "\n  Usage: macchina [option]\n  Options: --help\n           --palette\n           --no-color\n           --hide\n\n  Options are case-sensitive.");
}