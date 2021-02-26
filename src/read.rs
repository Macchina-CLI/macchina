use crate::{extra, format, PATH_TO_BATTERY_PERCENTAGE, PATH_TO_BATTERY_STATUS};
use extra::pop_newline;
use nix::unistd;
use std::{
    env, fs,
    process::{Command, Stdio},
};

/// Read desktop environment name from $DESKTOP_SESSION environment variable
/// or from the fallback environment variable $XDG_CURRENT_DESKTOP
pub fn desktop_environment() -> String {
    let desktop_env = env::var("DESKTOP_SESSION");
    match desktop_env {
        Ok(ret) => {
            if !ret.contains("/") {
                return ret.to_string();
            }
            format::desktop_environment(ret.to_string())
        }
        Err(_e) => {
            let fallback = env::var("XDG_CURRENT_DESKTOP").ok();
            let fallback = fallback
                .as_ref()
                .map(String::as_str)
                .and_then(|s| if s.is_empty() { None } else { Some(s) })
                .unwrap_or("Unknown");
            return String::from(fallback);
        }
    }
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

    let shell_name =
        String::from_utf8(output.stdout).expect("'ps' process stdout was not valid UTF-8");
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
    // If pacman is not installed, return 0
    return String::from("0");
}

/// Read kernel version from `/proc/version`
pub fn kernel_version() -> String {
    let output = fs::read_to_string("/proc/version");
    let ret = match output {
        Ok(ret) => ret.split_whitespace().nth(2).unwrap().to_string(),
        Err(_e) => return String::from("Unknown"),
    };
    ret
}

/// Read hostname using nix::unistd::gethostname()
pub fn hostname() -> String {
    let mut buf = [0u8; 64];
    let hostname_cstr = unistd::gethostname(&mut buf);
    match hostname_cstr {
        Ok(hostname_cstr) => {
            let hostname = hostname_cstr.to_str().unwrap_or("Unknown");
            return String::from(hostname);
        }
        Err(_e) => {
            return String::from("Unknown");
        }
    };
}

pub fn username() -> String {
    let output = Command::new("whoami")
        .output()
        .expect("Failed to get username using 'whoami'");
    let username =
        String::from_utf8(output.stdout).expect("'whoami' process stdout was not proper UTF-8");
    pop_newline(username)
}

/// Read distribution name from `/etc/os-release`
pub fn operating_system() -> String {
    let mut os = String::from(extra::get_line_at("/etc/os-release", 0, "Unknown").unwrap());
    if !os.contains("NAME=\"") {
        return os.replace("NAME=", "");
    }
    os.pop();
    os.replace("NAME=\"", "")
}

/// Read processor information from `/proc/cpuinfo`
pub fn cpu_model_name() -> String {
    let mut cpu = String::from(extra::get_line_at("/proc/cpuinfo", 4, "Unknown").unwrap());
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
        Err(_e) => return String::from("Unknown"),
    };
    ret
}
