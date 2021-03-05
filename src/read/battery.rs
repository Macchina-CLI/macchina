#![allow(unused_imports)]
use crate::{extra, Fail, PATH_TO_BATTERY_PERCENTAGE, PATH_TO_BATTERY_STATUS};
use std::fs;
use std::process::{Command, Stdio};

/// Read battery percentage from `/sys/class/power_supply/BAT0/capacity`
#[cfg(target_os = "linux")]
pub fn percentage(fail: &mut Fail) -> String {
    let percentage = fs::read_to_string(PATH_TO_BATTERY_PERCENTAGE);
    let ret = match percentage {
        Ok(ret) => ret,
        Err(_e) => {
            fail.battery.failed = true;
            return String::new();
        }
    };
    extra::pop_newline(ret)
}

/// Read battery status from `/sys/class/power_supply/BAT0/status`
#[cfg(target_os = "linux")]
pub fn status(fail: &mut Fail) -> String {
    let status = fs::read_to_string(PATH_TO_BATTERY_STATUS);
    let ret = match status {
        Ok(ret) => ret,
        Err(_e) => {
            fail.battery.failed = true;
            return String::new();
        }
    };
    extra::pop_newline(ret)
}

/// Read battery percentage using `envstat -d acpibat0 | grep charging:`
#[cfg(target_os = "netbsd")]
pub fn status(fail: &mut Fail) -> String {
    let envstat = Command::new("envstat")
        .arg("-d")
        .arg("acpibat0")
        .stdout(Stdio::piped())
        .spawn()
        .expect("ERROR: failed to spawn \"envstat\" process");

    let envstat_out = envstat
        .stdout
        .expect("ERROR: failed to open \"envstat\" stdout");

    let grep = Command::new("grep")
        .arg("charging:")
        .stdin(Stdio::from(envstat_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("ERROR: failed to spawn \"grep\" process");

    let output = grep
        .wait_with_output()
        .expect("ERROR: failed to wait for \"grep\" process to exit");
    let mut status = String::from_utf8(output.stdout)
        .expect("ERROR: \"grep\" process output was not valid UTF-8");
    status = status.replace("charging:", "").trim().to_string();
    if status.is_empty() {
        fail.battery.failed = true;
        return String::new();
    }
    status
}

/// Read battery status through `envstat -d acpibat0 | grep -oP '(?<=\().*(?=\))'`
#[cfg(target_os = "netbsd")]
pub fn percentage(fail: &mut Fail) -> String {
    let envstat = Command::new("envstat")
        .arg("-d")
        .arg("acpibat0")
        .stdout(Stdio::piped())
        .spawn()
        .expect("ERROR: failed to spawn \"envstat\" process");

    let envstat_out = envstat
        .stdout
        .expect("ERROR: failed to open \"envstat\" stdout");

    let grep = Command::new("grep")
        .arg("-o")
        .arg("-P")
        .arg(r"(?<=\().*(?=\))")
        .stdin(Stdio::from(envstat_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("ERROR: failed to spawn \"grep\" process");

    let output = grep
        .wait_with_output()
        .expect("ERROR: failed to wait for \"grep\" process to exit");
    let perc_str = String::from_utf8(output.stdout)
        .expect("ERROR: \"grep\" process output was not valid UTF-8");
    let percentage = perc_str.trim().split(".").next().unwrap_or("").to_string();

    if percentage.is_empty() {
        fail.battery.failed = true;
    }
    percentage
}
