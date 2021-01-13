use std::path::Path;
use std::fs::File;
use std::{env, fs};
use std::io::{BufRead, BufReader, Error};

pub fn read_uptime() -> f32 {
    let uptime = fs::read_to_string("/proc/uptime")
    .expect("Could not read uptime from /proc/uptime");
    //  Read first float from uptime
    let up = uptime.split_whitespace().next().unwrap_or("");
    //  up is now returned as f32 so we can properly format it using format_uptime()
    return up.parse().unwrap();
}

pub fn read_battery() -> String {
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

pub fn read_terminal() -> String
{
    if option_env!("TERM").unwrap().to_string() != "" { return option_env!("TERM").unwrap().to_string(); }
    return String::from("is $TERM set?")
}

pub fn read_shell(shorthand: bool) -> String {
    if env!("SHELL").to_string() != "" {
        if shorthand { return env!("SHELL").to_string().replace("/usr/bin/",""); }
        else { return env!("SHELL").to_string(); }
        
    }
    return String::from("is $SHELL set?")
}

pub fn read_osrelease() -> String {
    let osrelease = fs::read_to_string("/proc/sys/kernel/osrelease").expect("Could not read osrelease from /proc/sys/kernel/osrelease");
    let mut osrelease_str = String::from(osrelease);
    osrelease_str.pop();
    return osrelease_str;
}

pub fn read_hostname() -> String {
    let hostname = fs::read_to_string("/etc/hostname").expect("Could not read hostname from /etc/hostname");
    let mut hostname_str = String::from(hostname);
    hostname_str.pop();
    return hostname_str;
}

pub fn read_operating_system() -> String {
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

pub fn read_cpu_model_name() -> String {
    // To extract the cpu model name
    // we will feed cpu, the fourth line from 
    // /proc/cpuinfo and do some operations 
    // to return only the cpu model name
    let mut cpu = String::from(get_line_at(Path::new("/proc/cpuinfo"), 4, "Could not extract CPU model name!").unwrap());
    cpu = cpu.replace("model name","").replace(":","").trim().to_string();
    return cpu;
}

pub fn read_cpu_threads() -> String {
    let mut threads = String::from(get_line_at(Path::new("/proc/cpuinfo"), 10, "Could not extract CPU thread count!").unwrap());
    threads = threads.replace("siblings","").replace(":","").trim().to_string();
    let threads_text = String::from(" (".to_owned() + &threads + ")");
    return threads_text;
}

pub fn format_uptime(uptime: f32) -> String {
    
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

pub fn get_line_at(path: &Path, line_num: usize, msg: &str) -> Result<String, Error> {
    let file = File::open(path).expect(&msg);
    let content = BufReader::new(&file);
    let mut lines = content.lines();
    lines.nth(line_num).expect("Line out-of-bounds")
}