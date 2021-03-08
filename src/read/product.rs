#![allow(unused_imports)]
use crate::extra;
use std::fs;
use std::process::Command;

#[cfg(target_os = "linux")]
/// Read product version from `/sys/class/dmi/id/product_version`
pub fn product_version() -> String {
    let name = fs::read_to_string("/sys/class/dmi/id/product_version");
    let ret = match name {
        Ok(ret) => ret,
        Err(_e) => return String::new(),
    };
    extra::pop_newline(ret)
}

#[cfg(target_os = "linux")]
/// Read system vendor from `/sys/class/dmi/id/sys_vendor`
pub fn product_vendor() -> String {
    let name = fs::read_to_string("/sys/class/dmi/id/sys_vendor");
    let ret = match name {
        Ok(ret) => ret,
        Err(_e) => return String::new(),
    };
    extra::pop_newline(ret)
}

#[cfg(target_os = "linux")]
/// Read product family from `/sys/class/dmi/id/product_family`
pub fn product_family() -> String {
    let name = fs::read_to_string("/sys/class/dmi/id/product_family");
    let ret = match name {
        Ok(ret) => ret,
        Err(_e) => return String::new(),
    };
    extra::pop_newline(ret)
}

#[cfg(target_os = "linux")]
/// Read product name from `/sys/class/dmi/id/product_name`
pub fn product_name() -> String {
    let name = fs::read_to_string("/sys/class/dmi/id/product_name");
    let ret = match name {
        Ok(ret) => ret,
        Err(_e) => return String::new(),
    };
    extra::pop_newline(ret)
}

#[cfg(target_os = "netbsd")]
/// NetBSD: Read system vendor using `sysctl -nb machdep.dmi.system-vendor`
pub fn system_vendor() -> String {
    let output = Command::new("sysctl")
        .args(&["-n", "-b", "machdep.dmi.system-vendor"])
        .output()
        .expect("ERROR: failed to start \"sysctl\" process");

    let sysven = String::from_utf8(output.stdout)
        .expect("ERROR: \"sysctl\" process stdout was not valid UTF-8");
    String::from(sysven)
}

#[cfg(target_os = "netbsd")]
/// NetBSD: Read system version using `sysctl -nb machdep.dmi.system-version`
pub fn system_version() -> String {
    let output = Command::new("sysctl")
        .args(&["-n", "-b", "machdep.dmi.system-version"])
        .output()
        .expect("ERROR: failed to start \"sysctl\" process");

    let sysver = String::from_utf8(output.stdout)
        .expect("ERROR: \"sysctl\" process stdout was not valid UTF-8");
    String::from(sysver)
}

#[cfg(target_os = "netbsd")]
/// NetBSD: Read system product using `sysctl -nb machdep.dmi.system-product`
pub fn system_product() -> String {
    let output = Command::new("sysctl")
        .args(&["-n", "-b", "machdep.dmi.system-product"])
        .output()
        .expect("ERROR: failed to start \"sysctl\" process");

    let sysprod = String::from_utf8(output.stdout)
        .expect("ERROR: \"sysctl\" process stdout was not valid UTF-8");
    String::from(sysprod)
}
