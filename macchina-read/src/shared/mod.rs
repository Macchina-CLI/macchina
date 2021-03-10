use crate::traits::ReadoutError;
use crate::extra;
use std::ffi::CStr;
use std::process::Command;
use nix::unistd;

#[cfg(any(target_os = "linux", target_os = "netbsd"))]
pub(crate) fn uptime(fail: &mut Fail) -> String {
    let uptime = fs::read_to_string("/proc/uptime");
    match uptime {
        Ok(ret) => return ret.split_whitespace().next().unwrap().to_string(),
        Err(_e) => {
            fail.uptime.fail_component();
            return String::from("Unknown");
        }
    };
}

/// Read distribution name from `/etc/os-release`
#[cfg(any(target_os = "linux", target_os = "netbsd"))]
pub(crate) fn distribution() -> String {
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
#[cfg(any(target_os = "linux", target_os = "netbsd"))]
pub(crate) fn desktop_environment(fail: &mut Fail) -> String {
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
#[cfg(any(target_os = "linux", target_os = "netbsd"))]
pub(crate) fn window_manager(fail: &mut Fail) -> String {
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
#[cfg(target_family = "unix")]
pub(crate) fn terminal() -> Result<String, ReadoutError> {
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
        return Err(ReadoutError::Other(String::from("Terminal name was empty.")));
    }

    Ok(terminal_name)
}

fn get_passwd_struct() -> Result<*mut libc::passwd, ReadoutError> {
    let uid: libc::uid_t = unsafe { libc::geteuid() };

    //do not call free on passwd pointer according to man page.
    let passwd = unsafe { libc::getpwuid(uid) };

    if passwd != std::ptr::null_mut() {
        return Ok(passwd);
    }

    Err(ReadoutError::Other(String::from("Reading the account information failed.")))
}

#[cfg(target_family = "unix")]
pub(crate) fn whoami() -> Result<String, ReadoutError> {
    let passwd = get_passwd_struct()?;

    let name = unsafe { CStr::from_ptr((*passwd).pw_name) };
    if let Ok(str) = name.to_str() {
        return Ok(String::from(str));
    }

    Err(ReadoutError::Other(String::from("Unable to read username for current uid.")))
}

#[cfg(target_family = "unix")]
pub(crate) fn shell() -> Result<String, ReadoutError> {
    let passwd = get_passwd_struct()?;

    let shell_name = unsafe { CStr::from_ptr((*passwd).pw_shell) };
    if let Ok(str) = shell_name.to_str() {
        return Ok(String::from(str));
    }

    Err(ReadoutError::Other(String::from("Unable to read default shell for current uid.")))
}

/// Read processor information from `/proc/cpuinfo`
#[cfg(any(target_os = "linux", target_os = "netbsd"))]
pub(crate) fn cpu_model_name() -> String {
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