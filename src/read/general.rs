#![allow(unused_imports)]
#![allow(dead_code)]

use crate::{extra, format, Fail};
use extra::{pop_newline, ucfirst};
use nix::unistd;
use std::{
    env, fs,
    process::{Command, Stdio},
};

/// Read username using `whoami`
pub fn username(fail: &mut Fail) -> String {
    let output = Command::new("whoami")
        .output()
        .expect("ERROR: failed to start \"whoami\" process");
    let username = String::from_utf8(output.stdout)
        .expect("ERROR: \"whoami\" process stdout was not valid UTF-8");
    if !username.is_empty() {
        return pop_newline(username);
    }
    fail.host.fail_component();
    return String::new();
}

/// Read hostname using `nix::unistd::gethostname()`
pub fn hostname(fail: &mut Fail) -> String {
    let mut buf = [0u8; 64];
    let hostname_cstr = unistd::gethostname(&mut buf);
    match hostname_cstr {
        Ok(hostname_cstr) => {
            let hostname = hostname_cstr.to_str().unwrap_or("Unknown");
            return String::from(hostname);
        }
        Err(_e) => {
            fail.host.fail_component();
            return String::from("Unknown");
        }
    };
}

/// Read distribution name from `/etc/os-release`
pub fn distribution() -> String {
    let file = fs::File::open("/etc/os-release");
    match file {
        Ok(content) => {
            let head = Command::new("head")
                .args(&["-n", "1"])
                .stdin(Stdio::from(content))
                .stdout(Stdio::piped())
                .spawn()
                .expect("ERROR: failed to start \"head\" process");

            let output = head
                .wait_with_output()
                .expect("ERROR: failed to wait for \"head\" process to exit");

            let distribution = String::from_utf8(output.stdout)
                .expect("ERROR: \"ps\" process stdout was not valid UTF-8");
            return pop_newline(String::from(
                distribution.replace("\"", "").replace("NAME=", ""),
            ));
        }
        Err(_e) => {
            return String::from("Unknown");
        }
    }
}

/// Read desktop environment name from `DESKTOP_SESSION` environment variable
/// or from the fallback environment variable `XDG_CURRENT_DESKTOP`
pub fn desktop_environment(fail: &mut Fail) -> String {
    let desktop_env = env::var("DESKTOP_SESSION");
    match desktop_env {
        Ok(ret) => {
            if ret.contains("/") {
                return format::desktop_environment(ret.to_string());
            }
            extra::ucfirst(ret.to_string())
        }
        Err(_e) => {
            let fallback = env::var("XDG_CURRENT_DESKTOP").ok();
            let fallback = fallback
                .as_ref()
                .map(String::as_str)
                .and_then(|s| if s.is_empty() { None } else { Some(s) })
                .unwrap_or("Unknown");
            if fallback == "Unknown" {
                fail.desktop_env.fail_component();
            }
            return extra::ucfirst(fallback);
        }
    }
}

/// Read window manager using `wmctrl -m | grep Name:`
pub fn window_manager(fail: &mut Fail) -> String {
    if extra::which("wmctrl") {
        let wmctrl = Command::new("wmctrl")
            .arg("-m")
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to spawn \"wmctrl\" process");

        let wmctrl_out = wmctrl
            .stdout
            .expect("ERROR: failed to open \"wmctrl\" stdout");

        let grep = Command::new("grep")
            .arg("Name:")
            .stdin(Stdio::from(wmctrl_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to spawn \"grep\" process");

        let output = grep
            .wait_with_output()
            .expect("ERROR: failed to wait for \"grep\" process to exit");

        let window_manager = String::from_utf8(output.stdout)
            .expect("ERROR: \"wmctrl -m | grep Name:\" process stdout was not valid UTF-8");

        let window_man_name = pop_newline(String::from(window_manager.replace("Name:", "").trim()));
        if window_man_name == "N/A" {
            fail.window_man.fail_component();
        }
        return window_man_name;
    }
    fail.window_man.fail_component();
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
        .expect("ERROR: failed to start \"ps\" process");

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
        .expect("ERROR: failed to start \"ps\" output");

    let terminal_name = extra::ucfirst(
        String::from_utf8(name.stdout)
            .expect("ERROR: \"ps\" process stdout was not valid UTF-8")
            .trim(),
    );
    if terminal_name.is_empty() {
        fail.terminal.fail_component();
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
            .expect("ERROR: failed to start \"ps\" process");

        let shell_name = String::from_utf8(output.stdout)
            .expect("read_terminal: stdout to string conversion failed")
            .trim()
            .to_string();
        if shell_name.is_empty() {
            fail.shell.fail_component();
            fail.shell.extraction_method = String::from(
                "(ERROR:DISABLED) Uptime (Shorthand:ON) -> Extracted using ps -p $$ -o comm=",
            );
            return String::from("Unknown");
        }
        return extra::ucfirst(shell_name);
    }
    // If shell shorthand is false, run "ps -p $$ -o args=" instead of "ps -p $$ -o comm="
    // to print the full path of the current shell instance name
    let output = Command::new("ps")
        .arg("-p")
        .arg(unistd::getppid().to_string())
        .arg("-o")
        .arg("args=")
        .output()
        .expect("ERROR: failed to start \"ps\" process");

    let shell_path = String::from_utf8(output.stdout)
        .expect("ERROR: \"ps\" process stdout was not valid UTF-8")
        .trim()
        .to_string();

    if shell_path.is_empty() {
        fail.shell.fail_component();
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


#[cfg(target_os = "macos")]
pub fn uptime(fail: &mut Fail) -> String {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    const KERN_BOOTTIME: i32 = 21;
    const CTL_KERN: i32 = 1;

    let mut name = [CTL_KERN, KERN_BOOTTIME];
    let mut time = libc::timeval { tv_sec: 0, tv_usec: 0 };
    let ptr: *mut libc::timeval = &mut time;
    let mut size = std::mem::size_of::<libc::timeval>();

    let result = unsafe {
        libc::sysctl(name.as_mut_ptr().into(), name.len() as u32,
                     ptr as *mut libc::c_void, &mut size,
                     std::ptr::null_mut(), 0)
    };

    if result == 0 {
        let duration = Duration::new(time.tv_sec as u64, (time.tv_usec * 1000) as
            u32);

        let bootup_timestamp = UNIX_EPOCH + duration;

        if let Ok(duration) = SystemTime::now().duration_since(bootup_timestamp) {
            let seconds_since_boot = duration.as_secs_f64();
            return seconds_since_boot.to_string();
        }
    }

    fail.uptime.fail_component();
    String::from("0")
}

/// Read uptime from `/proc/uptime`
#[cfg(any(target_os = "linux", target_os = "netbsd"))]
pub fn uptime(fail: &mut Fail) -> String {
    let uptime = fs::read_to_string("/proc/uptime");
    match uptime {
        Ok(ret) => return ret.split_whitespace().next().unwrap().to_string(),
        Err(_e) => {
            fail.uptime.fail_component();
            return String::from("Unknown");
        }
    };
}
