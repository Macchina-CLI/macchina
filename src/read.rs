use crate::extra;
use std::{env, fs};

pub fn read_battery_percentage() -> String {
    let mut percentage = fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
        .expect("Could not read battery percentage from /sys/class/power_supply/BAT0/capacity");
    if percentage.ends_with('\n') {
        percentage.pop();
    }
    return String::from(&percentage);
}

pub fn read_battery_status() -> String {
    let mut status = fs::read_to_string("/sys/class/power_supply/BAT0/status")
        .expect("Could not read battery percentage from /sys/class/power_supply/BAT0/status");
    if status.ends_with('\n') {
        status.pop();
    }
    return status;
}

pub fn read_terminal() -> String {
    if option_env!("TERM").expect("Is $TERM set?").to_string() != "" {
        return option_env!("TERM").unwrap().to_string();
    }
    return String::from("is $TERM set?");
}

pub fn read_shell(shorthand: bool) -> String {
    if env!("SHELL").to_string() != "" {
        if shorthand {
            return env!("SHELL").to_string().replace("/usr/bin/", "");
        } else {
            return env!("SHELL").to_string();
        }
    }
    return String::from("is $SHELL set?");
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
    let mut os = String::from(extra::get_line_at("/etc/os-release", 0, "Could not extract OS name!").unwrap());
    // Keep only the Operating System name
    // Some Linux distributions write their
    // Operating System name inside quotes and
    // some do not, so we will account for both conditions
    if os.contains("NAME=") && !os.contains("\"") {
        os = os.replace("NAME=", "");
    } else {
        os = os.replace("NAME=\"", "");
        os.pop();
    }
    return os;
}

pub fn read_cpu_model_name() -> String {
    // To extract the cpu model name
    // we will feed cpu, the fourth line from
    // /proc/cpuinfo and do some operations
    // to return only the cpu model name
    let mut cpu = String::from(
        extra::get_line_at("/proc/cpuinfo", 4, "Could not extract CPU model name!").unwrap());
    cpu = cpu
        .replace("model name", "")
        .replace(":", "")
        .trim()
        .to_string();
    return cpu;
}

pub fn read_cpu_threads() -> String {
    let mut threads = String::from(extra::get_line_at("/proc/cpuinfo", 10, "Could not extract CPU thread count!").unwrap());
    threads = threads
        .replace("siblings", "")
        .replace(":", "")
        .trim()
        .to_string();
    let threads_text = String::from(" (".to_owned() + &threads + ")");
    return threads_text;
}

pub fn read_uptime() -> f32 {
    let uptime = fs::read_to_string("/proc/uptime").expect("Could not read uptime from /proc/uptime");
    //  Read first float from uptime
    let up = uptime.split_whitespace().next().unwrap_or("");
    //  up is now returned as f32 so we can properly format it using format_uptime()
    return up.parse().unwrap();
}
