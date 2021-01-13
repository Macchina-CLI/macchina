use colored::Colorize;
use crate::read;

pub fn show_info(color: bool, palette_status: bool, hide: &[u32]) {
    //  left_padding: change value to however many spaces you want
    let left_padding = 6;
    let padding = " ".repeat(left_padding);

    // This set of variables are the labels displayed
    // to the left of each system information Macchina reports
    // Change any x_key value to whatever your want
    // Example:
    //      Changing uptime_key value from "up" to "uptime"
    //      will tell machina to print uptime instead of up
    //      when displaying system information
    let separator = ':';
    let hostname_key = String::from("host");
    let os_key = String::from("os");
    let osrelease_key = String::from("kern");
    let terminal_key = String::from("term");
    let uptime_key = String::from("up");
    let cpu_model_name_key = String::from("cpu");
    let battery_key = String::from("bat");
    let shell_key = String::from("sh");
    // You may override this
    let shell_shorthand: bool = true;

    match color {
        true => {
            if hide[0] != 0 { println!("{}{}{} {}", padding, hostname_key.purple().bold(), separator, read::read_hostname()); }
            if hide[1] != 0 { println!("{}{}{}   {}", padding, os_key.blue().bold(), separator, read::read_operating_system()); }
            if hide[2] != 0 { println!("{}{}{} {}", padding, osrelease_key.green().bold(), separator, read::read_osrelease()); }
            if hide[3] != 0 { println!("{}{}{} {}", padding, terminal_key.cyan().bold(), separator, read::read_terminal()); }
            if hide[4] != 0 { println!("{}{}{}   {}", padding, shell_key.yellow().bold(), separator, read::read_shell(shell_shorthand)); }
            if hide[5] != 0 { println!("{}{}{}  {}{}", padding, cpu_model_name_key.red().bold(), separator, read::read_cpu_model_name(), read::read_cpu_threads()); }
            if hide[6] != 0 { println!("{}{}{}   {}", padding, uptime_key.purple().bold(), separator, read::format_uptime(read::read_uptime())); }
            if hide[7] != 0 { println!("{}{}{}  {}", padding, battery_key.blue().bold(), separator, read::read_battery()); }
        },
        false => {
            if hide[0] != 0 { println!("{}{}{} {}", padding, hostname_key, separator, read::read_hostname()); }
            if hide[1] != 0 { println!("{}{}{}   {}", padding, os_key, separator, read::read_operating_system()); }
            if hide[2] != 0 { println!("{}{}{} {}", padding, osrelease_key, separator, read::read_osrelease()); }
            if hide[3] != 0 { println!("{}{}{} {}", padding, terminal_key, separator, read::read_terminal()); }
            if hide[4] != 0 { println!("{}{}{}   {}", padding, shell_key, separator, read::read_shell(shell_shorthand)); }
            if hide[5] != 0 { println!("{}{}{}  {}{}", padding, cpu_model_name_key, separator, read::read_cpu_model_name(), read::read_cpu_threads()); }
            if hide[6] != 0 { println!("{}{}{}   {}", padding, uptime_key, separator, read::format_uptime(read::read_uptime())); }
            if hide[7] != 0 { println!("{}{}{}  {}", padding, battery_key, separator, read::read_battery()); }
        }
    };
    if palette_status == true && hide[8] != 0 {
        palette(left_padding);
    }
}

pub fn error(color: bool, vector: Vec<String>) {
    let args: [String; 4] = [ vector[0].to_string(), "--help".to_string(), "--palette".to_string(), "--no-color".to_string()];
    let mut incorrect_args: Vec<String> = Vec::new();
        for i in 0 .. vector.len() {
            if !args.contains(&vector[i]) {
                incorrect_args.push(vector[i].clone());
            }
        }
    
        if color {
            eprintln!("  {}: bad option {:?} {}","Error".red().bold(),incorrect_args, "\n  Usage: macchina [option]\n  Options: --help\n           --palette\n           --no-color\n\n  Options are case-sensitive.");
        }
        else {
            eprintln!("  {}: bad option {:?} {}","Error",incorrect_args, "\n  Usage: macchina [option]\n  Options: --help\n           --palette\n           --no-color\n\n  Options are case-sensitive.");
        }
}
    
pub fn hide(color: bool, palette_status: bool, vector: Vec<String>) -> [u32; 9] {
        let mut elements: [u32; 9] = [1;9];
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

        show_info( color, palette_status, &elements);
        return elements;
}
    
pub fn help(color: bool) {
        if color {
            println!("  {}:","Macchina".blue().bold());
            println!("  Usage: macchina [option]\n  Options: --help\n           --palette\n           --no-color\n\n  Options are case-sensitive");
        }
        else
        {
            println!("  {}:","Macchina");
            println!("  Usage: macchina [option]\n  Options: --help\n           --palette\n           --no-color\n\n  Options are case-sensitive");
        }
}

pub fn palette(left_padding: usize) {
    let padding = " ".repeat(left_padding);
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