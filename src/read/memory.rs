#![allow(dead_code)]
use std::process::Command;
use std::process::Stdio;

fn get_value(value: &str) -> u64 {
    let cat = Command::new("cat")
        .arg("/proc/meminfo")
        .stdout(Stdio::piped())
        .spawn()
        .expect("ERROR: failed to spawn \"cat\" process");

    let cat_out = cat.stdout.expect("ERROR: failed to open \"cat\" stdout");

    let grep = Command::new("grep")
        .arg(value)
        .stdin(Stdio::from(cat_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("ERROR: failed to start \"grep\" process");

    let mem = grep
        .wait_with_output()
        .expect("ERROR: failed to wait for \"grep\" process to exit");
    // Collect only the value of MemTotal
    let s_mem_kb: String = String::from_utf8(mem.stdout)
        .expect("\"grep\" process stdout was not valid UTF-8")
        .chars()
        .filter(|c| c.is_digit(10))
        .collect();
    s_mem_kb.parse::<u64>().unwrap_or(0)
}

/// Read __MemTotal__ from `/proc/meminfo`
pub fn memtotal() -> u64 {
    get_value("MemTotal")
}

/// Read __MemFree__ from `/proc/meminfo`
pub fn memfree() -> u64 {
    get_value("MemFree")
}

/// Read __Buffers__ from `/proc/meminfo`
pub fn buffers() -> u64 {
    get_value("Buffers")
}

/// Read __Cached__ from `/proc/meminfo`
pub fn cached() -> u64 {
    get_value("^Cached")
}

/// Read __SReclaimable__ from `/proc/meminfo`
pub fn sreclaimable() -> u64 {
    get_value("SReclaimable")
}

#[cfg(target_os = "linux")]
/// Calculate memory utilization,
/// used = memtotal - memfree - cached - sreclaimable - buffers
pub fn used() -> u64 {
    if sreclaimable() != 0 {
        return memtotal() - memfree() - cached() - sreclaimable() - buffers();
    }
    memtotal() - memfree() - cached() - buffers()
}

#[cfg(target_os = "netbsd")]
/// Calculate memory utilization,
/// used = memtotal - memfree - cached - sreclaimable - buffers
pub fn used() -> u64 {
    memtotal() - memfree()
}
