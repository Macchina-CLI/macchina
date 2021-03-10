#![allow(dead_code)]

use std::fs;
use std::process::Command;
use std::process::Stdio;

#[cfg(target_os = "netbsd")]
/// Calculate memory utilization:
/// `Used = MemTotal - MemFree`
pub fn used() -> u64 {
    memtotal() - memfree()
}