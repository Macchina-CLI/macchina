use crate::{extra, Fail};
use std::process::{Command, Stdio};

#[cfg(target_os = "linux")]
/// Extract package count for debian-based, arch-based distros or NetBSD
pub fn package_count(fail: &mut Fail) -> String {
    // Instead of having a condition for the millions of distros.
    // This function will try and extract package count by checking
    // if a certain package manager is installed
    if extra::which("pacman") {
        let pacman = Command::new("pacman")
            .args(&["-Q", "-q"])
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to start \"pacman\" process")
            .stdout
            .expect("ERROR: failed to open \"pacman\" stdout");

        let count = Command::new("wc")
            .arg("-l")
            .stdin(Stdio::from(pacman))
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to start \"wc\" process");

        let output = count
            .wait_with_output()
            .expect("ERROR: failed to wait for \"wc\" process to exit");
        return String::from_utf8(output.stdout)
            .expect("ERROR: \"pacman -Qq | wc -l\" output was not valid UTF-8")
            .trim()
            .to_string();
    } else if extra::which("dpkg") {
        let dpkg = Command::new("dpkg")
            .arg("-l")
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to start \"dpkg\" process")
            .stdout
            .expect("ERROR: failed to open \"dpkg\" stdout");

        let count = Command::new("wc")
            .arg("-l")
            .stdin(Stdio::from(dpkg))
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to start \"wc\" process");

        let output = count
            .wait_with_output()
            .expect("ERROR: failed to wait for \"wc\" process to exit");
        return String::from_utf8(output.stdout)
            .expect("ERROR: \"dpkg -l | wc -l\" output was not valid UTF-8")
            .trim()
            .to_string();
    } else if extra::which("emerge") {
        let ls = Command::new("ls")
            .arg("/var/db/pkg/*")
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to start \"ls\" process");

        let ls_out = ls.stdout.expect("ERROR: failed to open \"ls\" stdout");

        let count = Command::new("wc")
            .arg("-l")
            .stdin(Stdio::from(ls_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to start \"wc\" process");

        let output = count
            .wait_with_output()
            .expect("ERROR: failed to wait for \"wc\" process to exit");
        return String::from_utf8(output.stdout)
            .expect("ERROR: \"dpkg -l | wc -l\" output was not valid UTF-8")
            .trim()
            .to_string();
    }
    fail.packages.fail_component();
    return String::from("0");
}

#[cfg(target_os = "netbsd")]
/// Extract package count using `pkg_info | wc -l`
pub fn package_count(fail: &mut Fail) -> String {
    if extra::which("pkg_info") {
        let pkg_info = Command::new("pkg_info")
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to spawn \"pkg_info\" process");

        let pkg_out = pkg_info
            .stdout
            .expect("ERROR: failed to open \"pkg_info\" stdout");

        let count = Command::new("wc")
            .arg("-l")
            .stdin(Stdio::from(pkg_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to start \"wc\" process");

        let output = count
            .wait_with_output()
            .expect("ERROR: failed to wait on for \"wc\" process to exit");
        return String::from_utf8(output.stdout)
            .expect("ERROR: \"pkg_info | wc -l\" output was not valid UTF-8")
            .trim()
            .to_string();
    }
    fail.packages.fail_component();
    return String::from("0");
}
