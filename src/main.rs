use colored::Colorize;
use std::{env, fs};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

fn main() {
let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            show_info(true, false);
        },
        2 => {
            if args.contains(&"--no-color".to_string()) {
                show_info(false, false);
            }
            else if args.contains(&"--help".to_string()) {
                help();
            }
            else if args.contains(&"--palette".to_string()) {
                show_info(true, true);
            }
            else {
                error(args);
            }
        },
        _ => {
            error(args);
        }
    }
}

fn error(vector: Vec<String>) {
let args: [String; 4] = [
                      vector[0].to_string(),
                      "--help".to_string(),
                      "--palette".to_string(),
                      "--no-color".to_string()
                  ];

let mut incorrect_args: Vec<String> = Vec::new();
    for i in 0 .. vector.len() {
        if !args.contains(&vector[i]) {
            incorrect_args.push(vector[i].clone());
        }
    }

    println!("  {}: bad option {:?}","Error".red().bold(),incorrect_args);
    println!("  Usage: macchina [option]\n  Options: --help\n           --palette\n           --no-color\n\n  Options are case-sensitive");
}

fn help() {
    println!("  {}:","Macchina".blue().bold());
    println!("  Usage: macchina [option]\n  Options: --help\n           --palette\n           --no-color\n\n  Options are case-sensitive");
}

fn palette(left_padding: usize) {
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

fn show_info(color: bool, palette_status: bool) {
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
            println!("{}{}{} {}", padding, hostname_key.purple().bold(), separator, read_hostname());
            println!("{}{}{}   {}", padding, os_key.blue().bold(), separator, read_operating_system());
            println!("{}{}{} {}", padding, osrelease_key.green().bold(), separator, read_osrelease());
            println!("{}{}{} {}", padding, terminal_key.cyan().bold(), separator, read_terminal());
            println!("{}{}{}   {}", padding, shell_key.yellow().bold(), separator, read_shell(shell_shorthand));
            println!("{}{}{}  {}{}", padding, cpu_model_name_key.red().bold(), separator, read_cpu_model_name(), read_cpu_threads());
            println!("{}{}{}   {}", padding, uptime_key.purple().bold(), separator, format_uptime(read_uptime()));
            println!("{}{}{}  {}", padding, battery_key.blue().bold(), separator, read_battery());
            
            
        },
        false => {
            println!("{}{}{} {}", padding, hostname_key, separator, read_hostname());
            println!("{}{}{}   {}", padding, os_key, separator, read_operating_system());
            println!("{}{}{} {}", padding, osrelease_key, separator, read_osrelease());
            println!("{}{}{} {}", padding, terminal_key, separator, read_terminal());
            println!("{}{}{}   {}", padding, uptime_key, separator, format_uptime(read_uptime()));
            println!("{}{}{}  {}{}", padding, cpu_model_name_key, separator, read_cpu_model_name(), read_cpu_threads());
            println!("{}{}{}   {}", padding, shell_key.blue().bold(), separator, read_shell(shell_shorthand));
        }
    };
    if palette_status == true {
    palette(left_padding);
    }
}

fn format_uptime(uptime: f32) -> String {
    
    let mut _uptime = String::new();
    // Uptime is formatted to dd:hh:mm if the system has been up for longer than 60 seconds
    if uptime > 60.0 {
    let up_days     = (uptime / 60.0/60.0/24.0).floor();
    if up_days != 0.0 { _uptime = _uptime + &up_days.to_string() + "d, "; }

    let up_hours    = (uptime / 60.0/60.0%24.0).floor();
    if up_hours != 0.0 { _uptime = _uptime + &up_hours.to_string() + "h, "; }

    let up_minutes  = (uptime / 60.0%60.0).floor();
    if up_minutes != 0.0 { _uptime = _uptime + &up_minutes.to_string() + "m"; }
    }
    // Uptime is formatted to ss if the system has been up for fewer than 60 seconds
    else {
        let up_seconds  = (uptime % 60.0).floor();
        if up_seconds != 0.0 { _uptime = up_seconds.to_string() + "s"; } 
    }

    return _uptime;
}

fn read_uptime() -> f32 {
    let uptime = fs::read_to_string("/proc/uptime")
    .expect("Could not read uptime from /proc/uptime");
    //  Read first float from uptime
    let up = uptime.split_whitespace().next().unwrap_or("");
    //  up is now returned as f32 so we can properly format it using format_uptime()
    return up.parse().unwrap();
}

fn read_battery() -> String {
    let mut percentage = fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
    .expect("Could not read battery percentage from /sys/class/power_supply/BAT0/capacity");
    if percentage.ends_with('\n') {
        percentage.pop();
    }

    let mut status = fs::read_to_string("/sys/class/power_supply/BAT0/status")
    .expect("Could not read battery percentage from /sys/class/power_supply/BAT0/status");
    if status.ends_with('\n') {
        status.pop();
    }

    // Some computers stop charging before they reach 100%
    // so we will consider the battery to be full when
    // the battery percentage is within bat_full_range
    // This range is inclusive
    let bat_full_range: std::ops::RangeInclusive<i32> = 98 ..=100;
    if !bat_full_range.contains(&percentage.parse().unwrap()) {
    return String::from(percentage + "% - " + &status);
    }

    return String::from(&status);
}

fn read_terminal() -> String
{
    if env!("TERM").to_string() != "" { return env!("TERM").to_string(); }
    return String::from("is $TERM set?")
}

fn read_shell(shorthand: bool) -> String {
    if env!("SHELL").to_string() != "" {
        if shorthand { return env!("SHELL").to_string().replace("/usr/bin/",""); }
        else { return env!("SHELL").to_string(); }
        
    }
    return String::from("is $SHELL set?")
}

fn read_osrelease() -> String {
    let osrelease = fs::read_to_string("/proc/sys/kernel/osrelease").expect("Could not read osrelease from /proc/sys/kernel/osrelease");
    let mut osrelease_str = String::from(osrelease);
    osrelease_str.pop();
    return osrelease_str;
}

fn read_hostname() -> String {
    let hostname = fs::read_to_string("/etc/hostname").expect("Could not read hostname from /etc/hostname");
    let mut hostname_str = String::from(hostname);
    hostname_str.pop();
    return hostname_str;
}

fn read_operating_system() -> String {
    // To extract the operating system name
    // we will feed os, the first line from 
    // /etc/os-release and do some operations 
    // to return only the operating system name
    let mut os = String::from(get_line_at(Path::new("/etc/os-release"), 0, "Could not extract OS name!").unwrap());
    // Remove NAME=" from the line
    os = os.replace("NAME=\"","");
    // Remove the quote located at the end of line
    os.pop();
    return os;
}

fn read_cpu_model_name() -> String {
    // To extract the cpu model name
    // we will feed cpu, the fourth line from 
    // /proc/cpuinfo and do some operations 
    // to return only the cpu model name
    let mut cpu = String::from(get_line_at(Path::new("/proc/cpuinfo"), 4, "Could not extract CPU model name!").unwrap());
    cpu = cpu.replace("model name","").replace(":","").trim().to_string();
    return cpu;
}

fn read_cpu_threads() -> String {
    let mut threads = String::from(get_line_at(Path::new("/proc/cpuinfo"), 10, "Could not extract CPU thread count!").unwrap());
    threads = threads.replace("siblings","").replace(":","").trim().to_string();
    let threads_text = String::from(" (".to_owned() + &threads + ")");
    return threads_text;
}

fn get_line_at(path: &Path, line_num: usize, msg: &str) -> Result<String, Error> {
    let file = File::open(path).expect(&msg);
    let content = BufReader::new(&file);
    let mut lines = content.lines();
    lines.nth(line_num).expect("Line out-of-bounds")
}