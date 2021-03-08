#![allow(dead_code)]
use std::fs;
use std::process::Command;
use std::process::Stdio;

/// Obtain the value of a specified field from `/proc/meminfo` needed to calculate memory usage
fn get_value(value: &str) -> u64 {
    let file = fs::File::open("/proc/meminfo");
    match file {
        Ok(content) => {
            let grep = Command::new("grep")
                .arg(value)
                .stdin(Stdio::from(content))
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
        Err(_e) => {
            return 0;
        }
    }
}

/// Read __MemTotal__ using `get_value()`
pub fn memtotal() -> u64 {
    get_value("MemTotal")
}

/// Read __MemFree__ using `get_value()`
pub fn memfree() -> u64 {
    get_value("MemFree")
}

/// Read __Buffers__ using `get_value()`
pub fn buffers() -> u64 {
    get_value("Buffers")
}

/// Read __Cached__ using `get_value()`
pub fn cached() -> u64 {
    get_value("^Cached")
}

/// Read __SReclaimable__ using `get_value()`
pub fn sreclaimable() -> u64 {
    get_value("SReclaimable")
}

#[cfg(target_os = "linux")]
/// Calculate memory utilization:
/// `Used = MemTotal - MemFree - Cached - SReclaimable - Buffers`
pub fn used() -> u64 {
    if sreclaimable() != 0 {
        return memtotal() - memfree() - cached() - sreclaimable() - buffers();
    }
    memtotal() - memfree() - cached() - buffers()
}

#[cfg(target_os = "netbsd")]
/// Calculate memory utilization:
/// `Used = MemTotal - MemFree`
pub fn used() -> u64 {
    memtotal() - memfree()
}
