use std::process::Command;

/// Linux: Read kernel release using `sysctl -nb kernel.osrelease`
#[cfg(target_os = "linux")]
pub fn osrelease() -> String {
    let output = Command::new("sysctl")
        .args(&["-n", "-b", "kernel.osrelease"])
        .output()
        .expect("ERROR: failed to fetch \"kernel.osrelease\" using \"sysctl\"");

    let osrelease = String::from_utf8(output.stdout)
        .expect("ERROR: \"sysctl\" process stdout was not valid UTF-8");
    String::from(osrelease)
}

/// Linux: Read kernel type using `sysctl -nb kernel.ostype`
#[cfg(target_os = "linux")]
pub fn ostype() -> String {
    // sysctl -e -n -b kernel.osrelease
    let output = Command::new("sysctl")
        .args(&["-n", "-b", "kernel.ostype"])
        .output()
        .expect("ERROR: failed to fetch \"kernel.ostype\" using \"sysctl\"");

    let osrelease = String::from_utf8(output.stdout)
        .expect("ERROR: \"sysctl\" process stdout was not valid UTF-8");
    String::from(osrelease)
}

#[cfg(target_os = "netbsd")]
/// NetBSD: Read kernel release using `sysctl -nb kern.osrelease`
pub fn osrelease() -> String {
    let output = Command::new("sysctl")
        .args(&["-n", "-b", "kern.osrelease"])
        .output()
        .expect("ERROR: failed to fetch \"kernel.osrelease\" using \"sysctl\"");

    let osrelease = String::from_utf8(output.stdout)
        .expect("ERROR: \"sysctl\" process stdout was not valid UTF-8");
    String::from(osrelease)
}

#[cfg(target_os = "netbsd")]
/// NetBSD: Read kernel type using `sysctl -nb kern.ostype`
pub fn ostype() -> String {
    // sysctl -e -n -b kernel.osrelease
    let output = Command::new("sysctl")
        .args(&["-n", "-b", "kern.ostype"])
        .output()
        .expect("ERROR: failed to fetch \"kernel.ostype\" using \"sysctl\"");

    let osrelease = String::from_utf8(output.stdout)
        .expect("ERROR: \"sysctl\" process stdout was not valid UTF-8");
    String::from(osrelease)
}
