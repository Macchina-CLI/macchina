use colored::*;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;

fn main() {
let mut args:
    Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            show_info(true);
        },
        2 => {
            if args.contains(&"--no-color".to_string()) {
                show_info(false);
            }
            else if args.contains(&"-h".to_string()) | args.contains(&"--help".to_string()) {
                help();
            }
            else {
                error(&mut args);
            }
        },
        _ => {
            error(&mut args);
        }
    }
}

fn show_info(color: bool) {
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
            println!("  {}{} {}", hostname_key.purple().bold(), separator, read_hostname());
            println!("  {}{}   {}", os_key.blue().bold(), separator, read_operating_system());
            println!("  {}{} {}", osrelease_key.green().bold(), separator, read_osrelease());
            println!("  {}{} {}", terminal_key.yellow().bold(), separator, read_terminal());
            println!("  {}{}   {}", uptime_key.red().bold(), separator, format_uptime());
            println!("  {}{}  {}", cpu_model_name_key.purple().bold(), separator, read_cpu_model_name());
            println!("  {}{}  {}", battery_key.blue().bold(), separator, read_battery());
        },
        false => {
            println!("  {}{} {}", hostname_key, separator, read_hostname());
            println!("  {}{}   {}", os_key, separator, read_operating_system());
            println!("  {}{} {}", osrelease_key, separator, read_osrelease());
            println!("  {}{} {}", terminal_key, separator, read_terminal());
            println!("  {}{}   {}", uptime_key, separator, format_uptime());
            println!("  {}{}  {}", cpu_model_name_key, separator, read_cpu_model_name());
            println!("  {}{}  {}", battery_key, separator, read_battery());
        }
    };
}

fn error(vector: &mut Vec<String>) {
    vector.remove(0);
    println!("  {}: bad option {:?}","error".red().bold(),vector);
    println!("  usage: rustfetch [option]");
    println!("  options: --no-color");
    println!("           --help / -h");
}

fn help() {
    println!("  {}:","rustfetch".blue().bold());
    println!("  usage: rustfetch [option]");
    println!("  options: --no-color");
    println!("           --help / -h");
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
    
    return String::from(percentage + "% - " + &status);
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
    let line_num = 1usize;
    let mut os = String::from(get_line_at(Path::new("/etc/os-release"), line_num - 1).unwrap());
    os = os.replace("NAME=\"","");
    os = os.replace("\"","");
    return os;
}

fn read_cpu_model_name() -> String
{
    let line_num = 5usize;
    let mut cpu = String::from(get_line_at(Path::new("/proc/cpuinfo"), line_num - 1).unwrap());
    cpu = cpu.replace("model name","").replace(":","").trim().to_string();
    return cpu;
}

fn get_line_at(path: &Path, line_num: usize) -> Result<String, Error> {
    let file = File::open(path).expect("File not found or cannot be opened");
    let content = BufReader::new(&file);
    let mut lines = content.lines();
    lines.nth(line_num).expect("No line found at that position")
}