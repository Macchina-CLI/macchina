extern crate nix;
use crate::extra;
use nix::unistd;
use std::fs;
use std::process::{Command, Stdio};

/// Read battery percentage from __/sys/class/power_supply/BAT0/capacity__
pub fn battery_percentage() -> String {
    let percentage = fs::read_to_string("/sys/class/power_supply/BAT0/capacity");

    let ret = match percentage {
        Ok(ret) => ret,
        Err(_e) => return String::from("ERROR"),
    };

    extra::pop_newline(ret)
}

/// Read battery status from __/sys/class/power_supply/BAT0/status__
pub fn battery_status() -> String {
    let status = fs::read_to_string("/sys/class/power_supply/BAT0/status");

    let ret = match status {
        Ok(ret) => ret,
        Err(_e) => return String::from("ERROR"),
    };

    extra::pop_newline(ret)
}

/// Read current terminal instance using __ps__ command
pub fn terminal() -> String {
    //  ps -p $$ -o ppid=
    //  $$ doesn't work natively in rust but its value can be
    //  accessed through nix::unistd::getppid()
    let ppid = Command::new("ps")
        .arg("-p")
        .arg(unistd::getppid().to_string())
        .arg("-o")
        .arg("ppid=")
        .output()
        .expect("Failed to get current terminal instance PPID using 'ps -p <PID> o ppid='");

    let terminal_ppid = String::from_utf8(ppid.stdout)
        .expect("'ps' process stdout wasn't valid UTF-8")
        .trim()
        .to_string();

    let name = Command::new("ps")
        .arg("-p")
        .arg(terminal_ppid)
        .arg("o")
        .arg("comm=")
        .output()
        .expect("Failed to get current terminal instance name using 'ps -p <PID> o comm='");

    let terminal_name =
        String::from_utf8(name.stdout).expect("'ps' process stdout wasn't valid UTF-8");
    String::from(terminal_name.trim())
}

/// Read current shell instance name using __ps__ command
pub fn shell(shorthand: bool) -> String {
    //  ps -p $$ -o comm=
    //  $$ doesn't work natively in rust but its value can be
    //  accessed through nix::unistd::getppid()
    if shorthand {
        let output = Command::new("ps")
            .arg("-p")
            .arg(unistd::getppid().to_string())
            .arg("o")
            .arg("comm=")
            .output()
            .expect("Failed to get current shell instance name 'ps -p <PID> o args='");

        let shell_name = String::from_utf8(output.stdout)
            .expect("read_terminal: stdout to string conversion failed");
        return shell_name.trim().to_string();
    }

    // If shell shorthand is false, we use "args=" instead of "comm="
    // to print the full path of the current shell instance name
    let output = Command::new("ps")
        .arg("-p")
        .arg(unistd::getppid().to_string())
        .arg("o")
        .arg("args=")
        .output()
        .expect("Failed to get current shell instance name 'ps -p <PID> o args='");

    let shell_name = String::from_utf8(output.stdout)
        .expect("read_terminal: stdout to string conversion failed");
    String::from(shell_name.trim())
}

pub fn package_count() -> usize {
    if operating_system() == "Arch Linux" {
        let pacman = Command::new("pacman")
            .arg("-Q")
            .arg("-q")
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to start 'pacman' process");

        let pac_out =
            String::from_utf8(pacman.stdout).expect("'pacman' process stdout wasn't valid UTF-8");
        let packages: Vec<&str> = pac_out.split('\n').collect();

        return packages.len() - 1;
    }
    return 0;
}

/// Read kernel version by calling "uname -r"
pub fn kernel_version() -> String {
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to get kernel release using 'uname -r'");

    let kern_vers = String::from_utf8(output.stdout)
        .expect("read_kernel_version: stdout to string conversion failed");
    String::from(kern_vers.trim())
}

/// Read hostname using __unistd::gethostname()__
pub fn hostname() -> String {
    let mut buf = [0u8; 64];
    let hostname_cstr = unistd::gethostname(&mut buf).expect("Failed getting hostname");
    let hostname = hostname_cstr.to_str().expect("Hostname wasn't valid UTF-8");
    String::from(hostname)
}

/// Read operating system name from __/etc/os-release__
pub fn operating_system() -> String {
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
pub fn cpu_model_name() -> String {
    let mut cpu = String::from(
        extra::get_line_at("/proc/cpuinfo", 4, "Could not extract CPU model name!").unwrap(),
    );

    cpu = cpu
        .replace("model name", "")
        .replace(":", "")
        .trim()
        .to_string();
    cpu
}

/// Read first float (uptime) from __/proc/uptime
pub fn uptime() -> String {
    let uptime = fs::read_to_string("/proc/uptime").expect("Could not extract uptime!");
    //  Read first float from uptime
    let up = uptime.split_whitespace().next().unwrap_or("").to_string();
    up
}
