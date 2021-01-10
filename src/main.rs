use colored::*;
use std:: {env, fs};
use std::fs::File;
use std::io:: {BufRead, BufReader, Error};
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
                      "macchina".to_string(),
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

    println!("  {}: bad option {:?}","error".red().bold(),incorrect_args);
    println!("  usage: rustfetch [option]");
    println!("  options: --help");
    println!("           --palette");
    println!("           --no-color");
}

fn help() {
    println!("  {}:","rustfetch".blue().bold());
    println!("  usage: rustfetch [option]");
    println!("  options: --help");
    println!("           --palette");
    println!("           --no-color");
}

fn palette(left_padding: usize) {
    let padding = " ".repeat(left_padding);
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
    let separator = ':';
    let hostname_key = String::from("host");
    let os_key = String::from("os");
    let osrelease_key = String::from("kern");
    let terminal_key = String::from("term");
    let uptime_key = String::from("up");
    let cpu_model_name_key = String::from("cpu");
    let battery_key = String::from("bat");

    match color {
        true => {
            println!("{}{}{} {}", padding, hostname_key.purple().bold(), separator, read_hostname());
            println!("{}{}{}   {}", padding, os_key.blue().bold(), separator, read_operating_system());
            println!("{}{}{} {}", padding, osrelease_key.green().bold(), separator, read_osrelease());
            println!("{}{}{} {}", padding, terminal_key.cyan().bold(), separator, read_terminal());
            println!("{}{}{}   {}", padding, uptime_key.yellow().bold(), separator, format_uptime());
            println!("{}{}{}  {}{}", padding, cpu_model_name_key.red().bold(), separator, read_cpu_model_name(), read_cpu_threads());
            println!("{}{}{}  {}", padding, battery_key.purple().bold(), separator, read_battery());
        },
        false => {
            println!("{}{}{} {}", padding, hostname_key, separator, read_hostname());
            println!("{}{}{}   {}", padding, os_key, separator, read_operating_system());
            println!("{}{}{} {}", padding, osrelease_key, separator, read_osrelease());
            println!("{}{}{} {}", padding, terminal_key, separator, read_terminal());
            println!("{}{}{}   {}", padding, uptime_key, separator, format_uptime());
            println!("{}{}{}  {}{}", padding, cpu_model_name_key, separator, read_cpu_model_name(), read_cpu_threads());
            println!("{}{}{}  {}", padding, battery_key, separator, read_battery());
        }
    };
    if palette_status == true {
    palette(left_padding);
    }
}

fn format_uptime() -> String
{
let uptime_f32:
    f32 = read_uptime();
    let mut _uptime = String::new();
    let up_days     = (uptime_f32 / 60.0/60.0/24.0).floor();
    if up_days != 0.0 {
    _uptime = _uptime + &up_days.to_string() + "d, ";
    }

    let up_hours    = (uptime_f32 / 60.0/60.0%24.0).floor();
    if up_hours != 0.0 {
    _uptime = _uptime + &up_hours.to_string() + "h, ";
    }

    let up_minutes  = (uptime_f32 / 60.0%60.0).floor();
    if up_minutes != 0.0 {
    _uptime = _uptime + &up_minutes.to_string() + "m";
    }

    return _uptime;
}

fn read_uptime() -> f32
{
    let uptime = fs::read_to_string("/proc/uptime")
    .expect("Could not read uptime from /proc/uptime");

    //  Read first float from uptime
    let up = uptime
    .split_whitespace()
    .next()
    .unwrap_or("");

let uptime_f32:
    f32 = up.parse().unwrap();

    return uptime_f32;
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

    if percentage != "100" {
    return String::from(percentage + "% - " + &status);
    }

    return String::from(&status);
}

fn read_terminal() -> String
{
    return env!("TERM").to_string();
}

fn read_osrelease() -> String
{
    let osrelease = fs::read_to_string("/proc/sys/kernel/osrelease")
    .expect("Could not read osrelease from /proc/sys/kernel/osrelease");

    let mut osrelease_str = String::from(osrelease);
    if osrelease_str.ends_with('\n') {
        osrelease_str.pop();
    }

    return osrelease_str;
}

fn read_hostname() -> String
{
    let hostname = fs::read_to_string("/etc/hostname")
    .expect("Could not read hostname from /etc/hostname");

    let mut hostname_str = String::from(hostname);
    if hostname_str.ends_with('\n') {
        hostname_str.pop();
    }

    return hostname_str;
}

fn read_operating_system() -> String
{
    let line_num = 0;
    let mut os = String::from(get_line_at(Path::new("/etc/os-release"), line_num).unwrap());
    os = os.replace("NAME=\"","");
    os = os.replace("\"","");
    return os;
}

fn read_cpu_model_name() -> String
{
    let line_num = 4;
    let mut cpu = String::from(get_line_at(Path::new("/proc/cpuinfo"), line_num).unwrap());
    cpu = cpu.replace("model name","").replace(":","").trim().to_string();
    return cpu;
}

fn read_cpu_threads() -> String
{
    let line_num = 10;
    let mut threads = String::from(get_line_at(Path::new("/proc/cpuinfo"), line_num).unwrap());
    threads = threads.replace("siblings","").replace(":","").trim().to_string();
    let threads_text = String::from(" (".to_owned() + &threads + ")");
    return threads_text;
}

fn get_line_at(path: &Path, line_num: usize) -> Result<String, Error> {
    let file = File::open(path).expect("File not found or cannot be opened");
    let content = BufReader::new(&file);
    let mut lines = content.lines();
    lines.nth(line_num).expect("No line found at that position")
}