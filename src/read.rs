use crate::extra;
use std::{env, fs, io};

/// Read battery percentage from __/sys/class/power_supply/BAT0/capacity__
pub fn read_battery_percentage() -> String {
    let mut percentage = fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
        .expect("Could not read battery percentage from /sys/class/power_supply/BAT0/capacity");
    if percentage.ends_with('\n') {
        percentage.pop();
    }
    return String::from(&percentage);
}

/// Read battery status from __/sys/class/power_supply/BAT0/status__
pub fn read_battery_status() -> String {
    let mut status = fs::read_to_string("/sys/class/power_supply/BAT0/status")
        .expect("Could not read battery percentage from /sys/class/power_supply/BAT0/status");
    if status.ends_with('\n') {
        status.pop();
    }
    return status;
}

/// Read terminal from __TERM__ environment variable
pub fn read_terminal() -> String {
    if option_env!("TERM").expect("Is $TERM set?").to_string() != "" {
        return option_env!("TERM").unwrap().to_string();
    }
    return String::from("is $TERM set?");
}

/// Read shell from __SHELL__ environment variable
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

/// Read kernel version from __/proc/sys/kernel/osrelease__
pub fn read_kernel_version() -> Result<String, io::Error> {
    let kernel_version = fs::read_to_string("/proc/sys/kernel/osrelease")?;
    let mut kernel_version_str = String::from(kernel_version);
    kernel_version_str.pop();
    Ok(kernel_version_str)
}

/// Read hostname from __/etc/hostname__
pub fn read_hostname() -> Result<String, io::Error> {
    let hostname = fs::read_to_string("/etc/hostname")?;
    let mut hostname_str = String::from(hostname);
    if hostname_str.ends_with('\n') {
        hostname_str.pop();
    }
    Ok(hostname_str)
}

/// Read operating system name from __/etc/os-release__
pub fn read_operating_system() -> String {
    let mut os = String::from(extra::get_line_at("/etc/os-release", 0, "Could not extract OS name!").unwrap());
    if !os.contains("NAME=\"") {
        return os.replace("NAME=", "");
    }
    os.pop();
    os.replace("NAME=\"", "")
}

/// Read processor information from __/proc/cpuinfo__
pub fn read_cpu_model_name(shorthand: bool) -> String {
    let mut cpu = String::from(extra::get_line_at("/proc/cpuinfo", 4, "Could not extract CPU model name!").unwrap());
    cpu = cpu
        .replace("model name", "")
        .replace(":", "")
        .trim()
        .to_string();
    if shorthand && cpu.contains("Intel(R) Core(TM)") {
        cpu = cpu.replace("Intel(R) Core(TM)", "").replace("CPU ", "");
        return cpu.trim().to_string();
    }
    cpu
}

/// Read processor thread count from __/proc/cpuinfo__
pub fn read_cpu_threads() -> String {
    let mut threads = String::from(extra::get_line_at("/proc/cpuinfo", 10, "Could not extract CPU thread count!").unwrap());
    threads = threads
        .replace("siblings", "")
        .replace(":", "")
        .trim()
        .to_string();
    String::from(" (".to_owned() + &threads + ")")
}

/// Read first float (uptime) from __/proc/uptime
pub fn read_uptime() -> Result <String, io::Error> {
    let uptime = fs::read_to_string("/proc/uptime")?;
    //  Read first float from uptime
    let up = uptime.split_whitespace().next().unwrap_or("").to_string();
    Ok(up)
}
