#![allow(unused_imports)]
#![allow(dead_code)]
use crate::{extra, format, Fail, PATH_TO_BATTERY_PERCENTAGE, PATH_TO_BATTERY_STATUS};
use extra::{pop_newline, ucfirst};
use nix::unistd;
use std::{
    env, fs,
    process::{Command, Stdio},
};

/// Read username through `whoami`
pub fn username() -> String {
    let output = Command::new("whoami")
        .output()
        .expect("ERROR: failed to start \"whoami\" process");
    let username = String::from_utf8(output.stdout)
        .expect("ERROR: \"whoami\" process stdout was not valid UTF-8");
    pop_newline(username)
}

/// Read hostname using `nix::unistd::gethostname()`
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

/// Read distribution name through `cat /etc/os-release | head -n1`
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

/// Read desktop environment name from `DESKTOP_SESSION` environment variable
/// or from the fallback environment variable `XDG_CURRENT_DESKTOP`
pub fn desktop_environment(fail: &mut Fail) -> String {
    let desktop_env = env::var("DESKTOP_SESSION");
    match desktop_env {
        Ok(ret) => {
            if !ret.contains("/") {
                return extra::ucfirst(ret.to_string());
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

            // Checks if the desktop environment couldn't be extracted, and fails the component if true.
            if fallback == "Unknown" {
                fail.desktop_env.failed = true;
            }
            return extra::ucfirst(fallback);
        }
    }
}

/// Read distribution name through `cat /etc/os-release | head -n 1`
pub fn window_manager(fail: &mut Fail) -> String {
    let wh = Command::new("which")
        .arg("wmctrl")
        .output()
        .expect("ERROR: failed to start \"which\" process");
    let which =
        String::from_utf8(wh.stdout).expect("ERROR: \"which\" process stdout was not valid UTF-8");
    if !which.is_empty() {
        let grep = Command::new("wmctrl")
            .arg("-m")
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to spawn \"wmctrl\" process");

        let grep_out = grep.stdout.expect("ERROR: failed to open \"cat\" stdout");

        let head = Command::new("grep")
            .arg("Name:")
            .stdin(Stdio::from(grep_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to spawn \"head\" process");

        let output = head
            .wait_with_output()
            .expect("ERROR: failed to wait for \"head\" process to exit");

        let window_manager = String::from_utf8(output.stdout)
            .expect("ERROR: \"wmctrl -m | grep Name:\" process stdout was not valid UTF-8");

        let window_man_name = pop_newline(String::from(window_manager.replace("Name:", "").trim()));
        // Fail element if it isn't detected
        if window_man_name == "N/A" {
            fail.window_man.failed = true;
        }
        return window_man_name;
    }
    // Fail element if wmctrl isn't installed
    if which.is_empty() {
        fail.window_man.failed = true;
    }
    String::from("Unknown")
}

/// Read current terminal name using `ps`
pub fn terminal(fail: &mut Fail) -> String {
    //  ps -p $(ps -p $$ -o ppid=) o comm=
    //  $$ doesn't work natively in rust but its value can be
    //  accessed through nix::unistd::getppid()

    // the way this argument is processed is through 3 phases
    // 1. acquiring the value of ps -p $$ -o ppid=
    // 2. passing this value to ps -p o comm=
    // 3. the end result is ps -p $(ps -p $$ -o ppid=) o comm=, the command whose stdout is captured and printed
    let ppid = Command::new("ps")
        .arg("-p")
        .arg(unistd::getppid().to_string())
        .arg("-o")
        .arg("ppid=")
        .output()
        .expect("ERROR: failed to fetch PPID of the running terminal");

    let terminal_ppid = String::from_utf8(ppid.stdout)
        .expect("ERROR: \"ps\" process stdout was not valid UTF-8")
        .trim()
        .to_string();

    let name = Command::new("ps")
        .arg("-p")
        .arg(terminal_ppid)
        .arg("-o")
        .arg("comm=")
        .output()
        .expect("ERROR: failed to fetch the name of the running terminal");

    let terminal_name = extra::ucfirst(
        String::from_utf8(name.stdout)
            .expect("ERROR: \"ps\" process stdout was not valid UTF-8")
            .trim(),
    );
    if terminal_name.is_empty() {
        fail.terminal.failed = true;
        return String::from("Unknown");
    }
    terminal_name
}

/// Read current shell name/absolute path using `ps`
pub fn shell(shorthand: bool, fail: &mut Fail) -> String {
    //  ps -p $$ -o comm=
    //  $$ doesn't work natively in rust but its value can be
    //  accessed through nix::unistd::getppid()
    if shorthand {
        let output = Command::new("ps")
            .arg("-p")
            .arg(unistd::getppid().to_string())
            .arg("-o")
            .arg("comm=")
            .output()
            .expect("ERROR: failed to fetch the name of the running shell");

        let shell_name = String::from_utf8(output.stdout)
            .expect("read_terminal: stdout to string conversion failed")
            .trim()
            .to_string();
        if shell_name.is_empty() {
            fail.shell.failed = true;
            fail.shell.extraction_method = String::from(
                "(ERROR:DISABLED) Uptime (Shorthand:ON) -> Extracted using ps -p $$ -o comm=",
            );
            return String::from("Unknown");
        }
        return shell_name;
    }
    // If shell shorthand is false, we use "ps -p $$ -o args=" instead of "ps -p $$ -o comm="
    // to print the full path of the current shell instance name
    let output = Command::new("ps")
        .arg("-p")
        .arg(unistd::getppid().to_string())
        .arg("-o")
        .arg("args=")
        .output()
        .expect("ERROR: failed to fetch the path of the running shell");

    let shell_path = String::from_utf8(output.stdout)
        .expect("ERROR: \"ps\" process stdout was not valid UTF-8")
        .trim()
        .to_string();

    if shell_path.is_empty() {
        fail.shell.failed = true;
        fail.shell.extraction_method = String::from(
            "(ERROR:DISABLED) Uptime (Shorthand:OFF) -> Extracted using ps -p $$ -o args=",
        );
        return String::from("Unknown");
    }
    shell_path
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
        .expect("ERROR: failed to spawn \"head\" process");

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
pub fn uptime(time_shorthand: bool, fail: &mut Fail) -> String {
    let uptime = fs::read_to_string("/proc/uptime");
    let ret = match uptime {
        Ok(ret) => format::uptime(
            ret.split_whitespace().next().unwrap().to_string(),
            time_shorthand,
        ),
        Err(_e) => {
            fail.uptime.failed = true;
            return String::from("Unknown");
        }
    };
    ret
}
