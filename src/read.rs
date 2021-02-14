extern crate nix;
use crate::{extra, format, PATH_TO_BATTERY_PERCENTAGE, PATH_TO_BATTERY_STATUS};
use nix::unistd;
use std::env;
use std::fs;
use std::process::{Command, Stdio};

pub fn desktop_session() -> String {
    return String::from(env!("DESKTOP_SESSION"));
}

/// Read battery percentage from `/sys/class/power_supply/BAT0/capacity`
pub fn battery_percentage() -> String {
    let percentage = fs::read_to_string(PATH_TO_BATTERY_PERCENTAGE);

    let ret = match percentage {
        Ok(ret) => ret,
        Err(_e) => return String::new(),
    };

    extra::pop_newline(ret)
}

/// Read battery status from `/sys/class/power_supply/BAT0/status`
pub fn battery_status() -> String {
    let status = fs::read_to_string(PATH_TO_BATTERY_STATUS);
    let ret = match status {
        Ok(ret) => ret,
        Err(_e) => return String::new(),
    };
    extra::pop_newline(ret)
}

/// Read current terminal instance name using `ps`
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
        .expect("'ps' process stdout was not valid UTF-8")
        .trim()
        .to_string();

    let name = Command::new("ps")
        .arg("-p")
        .arg(terminal_ppid)
        .arg("o")
        .arg("comm=")
        .output()
        .expect("Failed to get current terminal instance name using 'ps -p <PID> o comm='");

    String::from_utf8(name.stdout)
        .expect("'ps' process stdout was not valid UTF-8")
        .trim()
        .to_string()
}

/// Read current shell instance name using `ps`
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

/// Extract package count using `pacman -Qq | wc -l`
pub fn package_count() -> String {
    let wh = Command::new("which")
        .arg("pacman")
        .output()
        .expect("Failed to start 'which' process");

    let which = String::from_utf8(wh.stdout).expect("'which' process stdout was not valid UTF-8");

    // Continue only if pacman exists
    if !which.is_empty() {
        let pacman = Command::new("pacman")
            .arg("-Q")
            .arg("-q")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start 'pacman' process");

        let pac_out = pacman.stdout.expect("Failed to open pacman stdout");

        let count = Command::new("wc")
            .arg("-l")
            .stdin(Stdio::from(pac_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start 'wc' process");

        let output = count.wait_with_output().expect("Failed to wait on wc");
        return String::from_utf8(output.stdout)
            .expect("package_count: output was not valid UTF-8")
            .trim()
            .to_string();
    }
    // If /usr/bin/pacman does not exist, package_count will return 0
    return String::from("0");
}

/// Read kernel version from `/proc/version`
pub fn kernel_version() -> String {
    let output = fs::read_to_string("/proc/version");
    let ret = match output {
        Ok(ret) => ret.split_whitespace().nth(2).unwrap().to_string(),
        Err(_e) => return String::from("Could not obtain kernel version"),
    };
    ret
}

/// Read hostname from `/etc/hostname`
pub fn hostname() -> String {
    let output = fs::read_to_string("/etc/hostname");
    let ret = match output {
        Ok(ret) => extra::pop_newline(ret),
        Err(_e) => return String::from("Could not obtain hostname"),
    };
    ret
}

/// Read distribution name from `/etc/os-release`
pub fn operating_system() -> String {
    let mut os = String::from(
        extra::get_line_at("/etc/os-release", 0, "Could not obtain distribution name").unwrap(),
    );
    if !os.contains("NAME=\"") {
        return os.replace("NAME=", "");
    }
    os.pop();
    os.replace("NAME=\"", "")
}

/// Read processor information from `/proc/cpuinfo`
pub fn cpu_model_name() -> String {
    let mut cpu = String::from(
        extra::get_line_at("/proc/cpuinfo", 4, "Could not obtain processor model name").unwrap(),
    );

    cpu = cpu
        .replace("model name", "")
        .replace(":", "")
        .trim()
        .to_string();
    cpu
}

/// Read uptime (first float) from `/proc/uptime`
pub fn uptime() -> String {
    let uptime = fs::read_to_string("/proc/uptime");
    let ret = match uptime {
        Ok(ret) => format::uptime(ret.split_whitespace().next().unwrap().to_string()),
        Err(_e) => return String::from("Could not obtain uptime"),
    };
    ret
}
