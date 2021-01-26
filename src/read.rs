use crate::extra;
use std::process::{Command, Stdio};
use std::{env, fs, io};

/// Read battery percentage from __/sys/class/power_supply/BAT0/capacity__
pub fn read_battery_percentage() -> String {
    let mut percentage = fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
        .expect("Could not read battery percentage from /sys/class/power_supply/BAT0/capacity");
    percentage = extra::pop_newline(percentage);
    return String::from(&percentage);
}

/// Read battery status from __/sys/class/power_supply/BAT0/status__
pub fn read_battery_status() -> String {
    let mut status = fs::read_to_string("/sys/class/power_supply/BAT0/status")
        .expect("Could not read battery percentage from /sys/class/power_supply/BAT0/status");
    status = extra::pop_newline(status);
    return status;
}

/// Read terminal from __TERM__ environment variable
pub fn read_terminal() -> String {
    if option_env!("TERM").expect("Is $TERM set?").to_string() != "" {
        return option_env!("TERM").unwrap().to_string();
    }
    return String::from("is $TERM set?");
}

pub fn read_package_count() -> String {
    let pacman = Command::new("pacman")
        .arg("-Q")
        .arg("-q")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start pacman process");

    let pac_out = pacman.stdout.expect("Failed to open pacman stdout");

    let count = Command::new("wc")
        .arg("-l")
        .stdin(Stdio::from(pac_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start wc process");

    let output = count.wait_with_output().expect("Failed to wait on wc");
    return String::from_utf8(output.stdout)
        .expect("read_package_count: stdout to string conversion failed")
        .trim()
        .to_string();
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

/// Read kernel version by calling "uname -r"
pub fn read_kernel_version() -> String {
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to get kernel release using 'uname -r'");

    let kern_vers = String::from_utf8(output.stdout)
        .expect("read_kernel_version: stdout to string conversion failed");
    kern_vers.trim().to_string()
}

/// Read hostname by calling "uname -n"
pub fn read_hostname() -> String {
    let output = Command::new("uname")
        .arg("-n")
        .output()
        .expect("Failed to get hostname using 'uname -n'");

    let hostname = String::from_utf8(output.stdout)
        .expect("read_hostname: stdout to string conversion failed");
    hostname.trim().to_string()
}

/// Read operating system name from __/etc/os-release__
pub fn read_operating_system() -> String {
    let mut os = String::from(
        extra::get_line_at("/etc/os-release", 0, "Could not extract OS name!").unwrap(),
    );
    if !os.contains("NAME=\"") {
        return os.replace("NAME=", "");
    }
    os.pop();
    os.replace("NAME=\"", "")
}

/// Read processor information from __/proc/cpuinfo__
pub fn read_cpu_model_name(shorthand: bool) -> String {
    let mut cpu = String::from(
        extra::get_line_at("/proc/cpuinfo", 4, "Could not extract CPU model name!").unwrap(),
    );
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

/// Read first float (uptime) from __/proc/uptime
pub fn read_uptime() -> Result<String, io::Error> {
    let uptime = fs::read_to_string("/proc/uptime")?;
    //  Read first float from uptime
    let up = uptime.split_whitespace().next().unwrap_or("").to_string();
    Ok(up)
}

pub fn read_used_memory() -> String {
    let free = Command::new("free")
        .arg("-h")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start 'free' process");

    let free_out = free.stdout.expect("Failed to open 'free' stdout");

    let awk = Command::new("awk")
        .arg("FNR == 2 {print $3}")
        .stdin(Stdio::from(free_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start 'awk' process");

    let output = awk.wait_with_output().expect("Failed to wait on awk");
    return String::from_utf8(output.stdout)
        .expect("read_used_memory: stdout to string conversion failed")
        .trim()
        .to_string();
}

pub fn read_total_memory() -> String {
    let free = Command::new("free")
        .arg("-h")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start 'free' process");

    let free_out = free.stdout.expect("Failed to open 'free' stdout");

    let awk = Command::new("awk")
        .arg("FNR == 2 {print $2}")
        .stdin(Stdio::from(free_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start 'awk' process");

    let output = awk.wait_with_output().expect("Failed to wait on awk");
    return String::from_utf8(output.stdout)
        .expect("read_total_memory: stdout to string conversion failed")
        .trim()
        .to_string();
}
