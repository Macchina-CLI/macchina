#![allow(unused_imports)]
use crate::extra;
use std::fs;
use std::process::Command;

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
