#[allow(unused_imports)]
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
            return extra::ucfirst(fallback);
        }
    }
}

/// Read current terminal instance namethrough `ps`
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

    extra::ucfirst(
        String::from_utf8(name.stdout)
            .expect("'ps' process stdout was not valid UTF-8")
            .trim(),
    )
}

/// Read current shell instance name through `ps`
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

    let shell_path =
        String::from_utf8(output.stdout).expect("'ps' process stdout was not valid UTF-8");
    String::from(shell_path.trim())
}

/// Extract package count through `pacman -Qq | wc -l`
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

/// Read username through `whoami`
pub fn username() -> String {
    let output = Command::new("whoami")
        .output()
        .expect("Failed to get username using 'whoami'");
    let username =
        String::from_utf8(output.stdout).expect("\"whoami\" process stdout was not proper UTF-8");
    pop_newline(username)
}

/// Read distribution name through `lsb_release`
// NAME="Arch Linux"
pub fn distribution() -> String {
    let grep = Command::new("cat")
        .arg("/etc/os-release")
        .stdout(Stdio::piped())
        .spawn()
        .expect("ERROR: failed to spawn \"cat\" process");

    let grep_out = grep.stdout.expect("ERROR: failed to open \"cat\" stdout");

    let head = Command::new("head")
        .args(&["-n", "1"])
        .stdin(Stdio::from(grep_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("ERROR: failed to start \"head\" process");

    let output = head
        .wait_with_output()
        .expect("ERROR: failed to wait for \"head\" process to exit");

    let distribution =
        String::from_utf8(output.stdout).expect("'ps' process stdout was not valid UTF-8");
    pop_newline(String::from(
        distribution.replace("\"", "").replace("NAME=", ""),
    ))
}

/// Read processor information from `/proc/cpuinfo`
pub fn cpu_model_name() -> String {
    let grep = Command::new("grep")
        .arg("model name")
        .arg("/proc/cpuinfo")
        .stdout(Stdio::piped())
        .spawn()
        .expect("ERROR: failed to spawn \"grep\" process");

    let grep_out = grep.stdout.expect("ERROR: failed to open \"grep\" stdout");

    let head = Command::new("head")
        .args(&["-n", "1"])
        .stdin(Stdio::from(grep_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("ERROR: failed to start \"head\" process");

    let output = head
        .wait_with_output()
        .expect("ERROR: failed to wait for \"head\" process to exit");
    let mut cpu = String::from_utf8(output.stdout)
        .expect("ERROR: \"head\" process output was not valid UTF-8");
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
