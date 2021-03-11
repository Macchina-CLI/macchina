use crate::{extra, Fail};
use std::process::{Command, Stdio};

#[cfg(target_os = "linux")]
/// Extract package count for debian-based, arch-based distros or NetBSD
pub fn package_count(fail: &mut Fail) -> String {
    // Instead of having a condition for each distribution,
    // we will try and extract package count by checking
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
    } else if extra::which("qlist") {
        let qlist = Command::new("qlist")
            .arg("-l")
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to start \"qlist\" process");

        let qlist_out = qlist
            .stdout
            .expect("ERROR: failed to open \"qlist\" stdout");

        let count = Command::new("wc")
            .arg("-l")
            .stdin(Stdio::from(qlist_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to start \"wc\" process");

        let output = count
            .wait_with_output()
            .expect("ERROR: failed to wait for \"wc\" process to exit");
        return String::from_utf8(output.stdout)
            .expect("ERROR: \"ls /var/db/pkg | wc -l\" output was not valid UTF-8")
            .trim()
            .to_string();
    } else if extra::which("xbps-query") {
        let xbps = Command::new("xbps-query")
            .arg("-l")
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to start \"xbps-query\" process");

        let xbps_out = xbps
            .stdout
            .expect("ERROR: failed to open \"xbps-query\" stdout");

        let grep = Command::new("grep")
            .arg("ii")
            .stdin(Stdio::from(xbps_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to start \"grep\" process");

        let grep_out = grep.stdout.expect("ERROR: failed to read \"grep\" stdout");

        let count = Command::new("wc")
            .arg("-l")
            .stdin(Stdio::from(grep_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to start \"wc\" process");

        let output = count
            .wait_with_output()
            .expect("ERROR: failed to wait for \"wc\" process to exit");
        return String::from_utf8(output.stdout)
            .expect("ERROR: \"xbps-query -l | grep ii | wc -l\" output was not valid UTF-8")
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
