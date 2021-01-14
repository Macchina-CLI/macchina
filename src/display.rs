use colored::Colorize;
use crate::read;

pub fn show_info(color: bool, palette_status: bool, icons: bool, signal: bool, hide: &[u32]) {
    if signal {
    //  ----------------- CUSTOMIZABLE ------------------ \\
    //  
    //  left_padding:
    //  Change value to however many spaces you 
    //  want Macchina to display to the left of text
    let left_padding = 6;
    //  shell_shorthand:
    //  If set to "true", Macchina will display full path to shell binary, e.g: /usr/bin/zsh
    //  If set to "false", Macchina will display shell name, e.g: zsh
    let shell_shorthand: bool = false;

    // This set of variables are the labels displayed
    // to the left of each system information Macchina reports
    // Change any x_key value to whatever your want
    // Example:
    //      Changing uptime_key value from "up" to "uptime"
    //      will tell Macchina to print uptime instead of up
    //      when displaying system information
        let separator = if icons { ' ' } else { ':' };
        let hostname_key = String::from("host");
        let os_key = String::from("os");
        let osrelease_key = String::from("kern");
        let terminal_key = String::from("term");
        let uptime_key = String::from("up");
        let cpu_model_name_key = String::from("cpu");
        let battery_key = String::from("bat");
        let shell_key = String::from("sh");
        
    //  ------------------------------------------------- \\
    
    //  This line does the padding calculation 
    //  based on the value of left_padding
    let padding = " ".repeat(left_padding);
    

    match color {
        true => {
            match icons {
                true => {
                    let (hostname_icon, os_icon, terminal_icon, uptime_icon, osrelease_icon, cpu_icon, shell_icon, battery_icon): 
                    (char, char, char, char, char, char, char, char);
                    hostname_icon = '';
                    terminal_icon = '';
                    uptime_icon = '神';
                    osrelease_icon = '';
                    os_icon = pick_icon_for_os();
                    cpu_icon = '';
                    battery_icon = '';
                    shell_icon = '';
                    if hide[0] != 0 { println!("{}{}  {}", padding, hostname_icon, read::read_hostname().purple().bold()); }
                    if hide[1] != 0 { println!("{}{}  {}", padding, os_icon, read::read_operating_system().blue().bold()); }
                    if hide[2] != 0 { println!("{}{}  {}", padding, osrelease_icon, read::read_osrelease().green().bold()); }
                    if hide[3] != 0 { println!("{}{}  {}", padding, terminal_icon, read::read_terminal().cyan().bold()); }
                    if hide[4] != 0 { println!("{}{}  {}", padding, shell_icon, read::read_shell(shell_shorthand).yellow().bold()); }
                    if hide[5] != 0 { println!("{}{}  {}{}", padding, cpu_icon, read::read_cpu_model_name().red().bold(), read::read_cpu_threads().red().bold()); }
                    if hide[6] != 0 { println!("{}{} {}", padding, uptime_icon, read::format_uptime(read::read_uptime()).purple().bold()); }
                    if hide[7] != 0 { println!("{}{}  {}", padding, battery_icon, read::read_battery().blue().bold()); }
                },
                _ => {
                    if hide[0] != 0 { println!("{}{}{} {}", padding, hostname_key.purple().bold(), separator, read::read_hostname()); }
                    if hide[1] != 0 { println!("{}{}{}   {}", padding, os_key.blue().bold(), separator, read::read_operating_system()); }
                    if hide[2] != 0 { println!("{}{}{} {}", padding, osrelease_key.green().bold(), separator, read::read_osrelease()); }
                    if hide[3] != 0 { println!("{}{}{} {}", padding, terminal_key.cyan().bold(), separator, read::read_terminal()); }
                    if hide[4] != 0 { println!("{}{}{}   {}", padding, shell_key.yellow().bold(), separator, read::read_shell(shell_shorthand)); }
                    if hide[5] != 0 { println!("{}{}{}  {}{}", padding, cpu_model_name_key.red().bold(), separator, read::read_cpu_model_name(), read::read_cpu_threads()); }
                    if hide[6] != 0 { println!("{}{}{}   {}", padding, uptime_key.purple().bold(), separator, read::format_uptime(read::read_uptime())); }
                    if hide[7] != 0 { println!("{}{}{}  {}", padding, battery_key.blue().bold(), separator, read::read_battery()); }
                }
            }
            
        },
        false => {
            match icons {
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
                    os_icon = '';
                    terminal_icon = '';
                    uptime_icon = '神';
                    osrelease_icon = '';
                    cpu_icon = '';
                    battery_icon = '';
                    shell_icon = '';
                    if hide[0] != 0 { println!("{}{}  {}", padding, hostname_icon, read::read_hostname()); }
                    if hide[1] != 0 { println!("{}{}  {}", padding, os_icon, read::read_operating_system()); }
                    if hide[2] != 0 { println!("{}{}  {}", padding, osrelease_icon, read::read_osrelease()); }
                    if hide[3] != 0 { println!("{}{}  {}", padding, terminal_icon, read::read_terminal()); }
                    if hide[4] != 0 { println!("{}{}  {}", padding, shell_icon, read::read_shell(shell_shorthand)); }
                    if hide[5] != 0 { println!("{}{}  {}{}", padding, cpu_icon, read::read_cpu_model_name(), read::read_cpu_threads()); }
                    if hide[6] != 0 { println!("{}{}  {}", padding, uptime_icon, read::format_uptime(read::read_uptime())); }
                    if hide[7] != 0 { println!("{}{}  {}", padding, battery_icon, read::read_battery()); }
                },
                _ => {
                    if hide[0] != 0 { println!("{}{}{} {}", padding, hostname_key, separator, read::read_hostname()); }
                    if hide[1] != 0 { println!("{}{}{}   {}", padding, os_key, separator, read::read_operating_system()); }
                    if hide[2] != 0 { println!("{}{}{} {}", padding, osrelease_key, separator, read::read_osrelease()); }
                    if hide[3] != 0 { println!("{}{}{} {}", padding, terminal_key, separator, read::read_terminal()); }
                    if hide[4] != 0 { println!("{}{}{}   {}", padding, shell_key, separator, read::read_shell(shell_shorthand)); }
                    if hide[5] != 0 { println!("{}{}{}  {}{}", padding, cpu_model_name_key, separator, read::read_cpu_model_name(), read::read_cpu_threads()); }
                    if hide[6] != 0 { println!("{}{}{}   {}", padding, uptime_key, separator, read::format_uptime(read::read_uptime())); }
                    if hide[7] != 0 { println!("{}{}{}  {}", padding, battery_key, separator, read::read_battery()); }
                }
            }
            
        }
    };

    if palette_status && hide[8] != 0 {
        palette(left_padding);
    }
}
}

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
    
pub fn hide(color: bool, palette_status: bool, icons: bool, vector: Vec<String>) -> [u32; 9] {
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

        show_info( color, palette_status, icons, true, &elements);
        return elements;
}
    
pub fn help(color: bool) {
        if color {
            println!("  {}:","Macchina".blue().bold());
            println!("  Usage: macchina [option]\n  Options: --help\n           --palette\n           --no-color\n           --hide\n\n  Options are case-sensitive.");
        }
        else
        {
            println!("  Macchina");
            println!("  Usage: macchina [option]\n  Options: --help\n           --palette\n           --no-color\n           --hide\n\n  Options are case-sensitive.");
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