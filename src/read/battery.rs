#![allow(unused_imports)]
use crate::{extra, Fail};
use std::fs;
use std::process::{Command, Stdio};

/// Read battery percentage from `acpi` command
#[cfg(target_os = "linux")]
pub fn percentage(fail: &mut Fail) -> String {
    if extra::which("acpi") {
        let acpi = Command::new("acpi")
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to spawn \"acpi\" process");
        let acpi_out = acpi.stdout.expect("Error: failed to open \"acpi\" stdout");
        let awk = Command::new("awk")
            .arg("-F:|,")
            .arg("{print $3}")
            .stdin(Stdio::from(acpi_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to spawn \"awk\" process");
        let output = awk
            .wait_with_output()
            .expect("ERROR: failed to wait for \"acpi\" process to exit");
        let mut percentage =
            String::from_utf8(output.stdout).expect("\"awk\" process output was not valid uff8");
        if percentage.is_empty() {
            fail.battery.fail_component();
            return String::new();
        }
        percentage = extra::pop_newline(percentage);
        percentage = extra::pop_percent(percentage);
        percentage = extra::pop_whitespace(percentage);

        return percentage;
    }
    fail.battery.fail_component();
    String::new()
}

/// Read battery status from `acpi` command.
#[cfg(target_os = "linux")]
pub fn status(fail: &mut Fail) -> String {
    if extra::which("acpi") {
        let acpi = Command::new("acpi")
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to spawn \"acpi\" process");
        let acpi_out = acpi.stdout.expect("Error: failed to open \"acpi\" stdout");
        let awk = Command::new("awk")
            .arg("-F:|,")
            .arg("{print $2}")
            .stdin(Stdio::from(acpi_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to spawn \"awk\" process");
        let output = awk
            .wait_with_output()
            .expect("ERROR: failed to wait for \"acpi\" process to exit");
        let mut status =
            String::from_utf8(output.stdout).expect("\"awk\" process output was not valid uff8");
        if status.is_empty() {
            fail.battery.fail_component();
            return String::new();
        }
        status = extra::pop_newline(status);
        status = extra::pop_whitespace(status);
        return status;
    }
    fail.battery.fail_component();
    String::new()
}

/// Read battery percentage using `envstat -d acpibat0 | rg charging:`
#[cfg(target_os = "netbsd")]
pub fn status(fail: &mut Fail) -> String {
    if extra::which("rg") {
        let envstat = Command::new("envstat")
            .args(&["-d", "acpibat0"])
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to spawn \"envstat\" process");

        let envstat_out = envstat
            .stdout
            .expect("ERROR: failed to open \"envstat\" stdout");

        let grep = Command::new("rg")
            .arg("charging:")
            .stdin(Stdio::from(envstat_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to spawn \"rg\" process");

        let output = grep
            .wait_with_output()
            .expect("ERROR: failed to wait for \"rg\" process to exit");
        let mut status = String::from_utf8(output.stdout)
            .expect("ERROR: \"grep\" process output was not valid UTF-8");
        status = status.replace("charging:", "").trim().to_string();
        if status.is_empty() {
            fail.battery.fail_component();
            return String::new();
        }
        return status;
    }
    fail.battery.fail_component();
    String::new()
}

/// Read battery status using `envstat -d acpibat0 | rg -oP '(?<=\().*(?=\))'`
#[cfg(target_os = "netbsd")]
pub fn percentage(fail: &mut Fail) -> String {
    if extra::which("rg") {
        let envstat = Command::new("envstat")
            .args(&["-d", "acpibat0"])
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to spawn \"envstat\" process");

        let envstat_out = envstat
            .stdout
            .expect("ERROR: failed to open \"envstat\" stdout");

        let rg = Command::new("rg")
            .args(&["-o", "-P", r"(?<=\().*(?=\))"])
            .stdin(Stdio::from(envstat_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to spawn \"rg\" process");
        let output = rg
            .wait_with_output()
            .expect("ERROR: failed to wait for \"rg\" process to exit");
        let perc_str = String::from_utf8(output.stdout)
            .expect("ERROR: \"rg\" process output was not valid UTF-8");
        let percentage = perc_str.trim().split(".").next().unwrap_or("").to_string();

        if percentage.is_empty() {
            fail.battery.fail_component();
        }
        return percentage;
    }
    fail.battery.fail_component();
    String::new()
}
