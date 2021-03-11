use crate::extra;
use crate::traits::*;
use nix::unistd;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

pub struct LinuxBatteryReadout;

pub struct LinuxKernelReadout;

pub struct LinuxGeneralReadout;

pub struct LinuxMemoryReadout;

pub struct LinuxProductReadout;

pub struct LinuxPackageReadout;

impl BatteryReadout for LinuxBatteryReadout {
    fn new() -> Self {
        LinuxBatteryReadout
    }

    fn percentage(&self) -> Result<String, ReadoutError> {
        let mut bat_path = Path::new("/sys/class/power_supply/BAT0/capacity");
        if !Path::exists(bat_path) {
            bat_path = Path::new("/sys/class/power_supply/BAT1/capacity");
        }

        Ok(extra::pop_newline(fs::read_to_string(bat_path)?))
    }

    fn status(&self) -> Result<String, ReadoutError> {
        let mut bat_path = Path::new("/sys/class/power_supply/BAT0/status");
        if !Path::exists(bat_path) {
            bat_path = Path::new("/sys/class/power_supply/BAT1/status");
        }

        Ok(extra::pop_newline(fs::read_to_string(bat_path)?))
    }
}

impl KernelReadout for LinuxKernelReadout {
    fn new() -> Self {
        LinuxKernelReadout
    }

    fn os_release(&self) -> Result<String, ReadoutError> {
        let output = Command::new("sysctl")
            .args(&["-n", "-b", "kernel.osrelease"])
            .output()
            .expect("ERROR: failed to fetch \"kernel.osrelease\" using \"sysctl\"");

        let osrelease = String::from_utf8(output.stdout)
            .expect("ERROR: \"sysctl\" process stdout was not valid UTF-8");

        Ok(String::from(osrelease))
    }

    fn os_type(&self) -> Result<String, ReadoutError> {
        // sysctl -e -n -b kernel.osrelease
        let output = Command::new("sysctl")
            .args(&["-n", "-b", "kernel.ostype"])
            .output()
            .expect("ERROR: failed to fetch \"kernel.ostype\" using \"sysctl\"");

        let osrelease = String::from_utf8(output.stdout)
            .expect("ERROR: \"sysctl\" process stdout was not valid UTF-8");

        Ok(String::from(osrelease))
    }
}

impl GeneralReadout for LinuxGeneralReadout {
    fn new() -> Self {
        LinuxGeneralReadout
    }

    fn machine(&self) -> Result<String, ReadoutError> {
        let product_readout = LinuxProductReadout::new();

        let name = product_readout.name()?;
        let family = product_readout.family().unwrap_or(String::new());
        let version = product_readout.version().unwrap_or(String::new());

        if family == name && family == version {
            return Ok(family);
        } else if version.is_empty() || version.len() <= 15 {
            let vendor = product_readout.vendor().unwrap_or(String::new());

            if vendor.len() > 0 {
                return Ok(format!("{} {} {}", vendor, family, name));
            }
        }

        Ok(version)
    }

    fn username(&self) -> Result<String, ReadoutError> {
        crate::shared::whoami()
    }

    fn hostname(&self) -> Result<String, ReadoutError> {
        let mut buf = [0u8; 64];
        let hostname_cstr = unistd::gethostname(&mut buf);
        match hostname_cstr {
            Ok(hostname_cstr) => {
                let hostname = hostname_cstr.to_str().unwrap_or("Unknown");
                return Ok(String::from(hostname));
            }
            Err(_e) => Err(ReadoutError::Other(String::from(
                "ERROR: failed to fetch hostname from \"unistd::gethostname()\"",
            ))),
        }
    }

    fn distribution(&self) -> Result<String, ReadoutError> {
        crate::shared::distribution()
    }

    fn desktop_environment(&self) -> Result<String, ReadoutError> {
        crate::shared::desktop_environment()
    }

    fn window_manager(&self) -> Result<String, ReadoutError> {
        crate::shared::window_manager()
    }

    fn terminal(&self) -> Result<String, ReadoutError> {
        crate::shared::terminal()
    }

    fn shell(&self, shorthand: bool) -> Result<String, ReadoutError> {
        crate::shared::shell(shorthand)
    }

    fn cpu_model_name(&self) -> Result<String, ReadoutError> {
        Ok(crate::shared::cpu_model_name())
    }

    fn uptime(&self) -> Result<String, ReadoutError> {
        crate::shared::uptime()
    }
}

impl MemoryReadout for LinuxMemoryReadout {
    fn new() -> Self {
        LinuxMemoryReadout
    }

    fn total(&self) -> Result<u64, ReadoutError> {
        Ok(crate::shared::get_meminfo_value("MemTotal"))
    }

    fn free(&self) -> Result<u64, ReadoutError> {
        Ok(crate::shared::get_meminfo_value("MemFree"))
    }

    fn buffers(&self) -> Result<u64, ReadoutError> {
        Ok(crate::shared::get_meminfo_value("Buffers"))
    }

    fn cached(&self) -> Result<u64, ReadoutError> {
        Ok(crate::shared::get_meminfo_value("^Cached"))
    }

    fn reclaimable(&self) -> Result<u64, ReadoutError> {
        Ok(crate::shared::get_meminfo_value("SReclaimable"))
    }

    fn used(&self) -> Result<u64, ReadoutError> {
        let total = self.total().unwrap();
        let free = self.free().unwrap();
        let cached = self.cached().unwrap();
        let reclaimable = self.reclaimable().unwrap();
        let buffers = self.buffers().unwrap();

        if reclaimable != 0 {
            return Ok(total - free - cached - reclaimable - buffers);
        }

        Ok(total - free - cached - buffers)
    }
}

impl ProductReadout for LinuxProductReadout {
    fn new() -> Self {
        LinuxProductReadout
    }

    fn version(&self) -> Result<String, ReadoutError> {
        Ok(extra::pop_newline(fs::read_to_string(
            "/sys/class/dmi/id/product_version",
        )?))
    }

    fn vendor(&self) -> Result<String, ReadoutError> {
        Ok(extra::pop_newline(fs::read_to_string(
            "/sys/class/dmi/id/sys_vendor",
        )?))
    }

    fn family(&self) -> Result<String, ReadoutError> {
        Ok(extra::pop_newline(fs::read_to_string(
            "/sys/class/dmi/id/product_family",
        )?))
    }

    fn name(&self) -> Result<String, ReadoutError> {
        Ok(extra::pop_newline(fs::read_to_string(
            "/sys/class/dmi/id/product_name",
        )?))
    }
}

impl PackageReadout for LinuxPackageReadout {
    fn new() -> Self {
        LinuxPackageReadout
    }

    fn count_pkgs(&self) -> Result<String, ReadoutError> {
        // Instead of having a condition for each distribution.
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
            return Ok(String::from_utf8(output.stdout)
                .expect("ERROR: \"pacman -Qq | wc -l\" output was not valid UTF-8")
                .trim()
                .to_string());
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
            return Ok(String::from_utf8(output.stdout)
                .expect("ERROR: \"dpkg -l | wc -l\" output was not valid UTF-8")
                .trim()
                .to_string());
        } else if extra::which("qlist") {
            let qlist = Command::new("qlist")
                .arg("-I")
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
            return Ok(String::from_utf8(output.stdout)
                .expect("ERROR: \"qlist -I | wc -l\" output was not valid UTF-8")
                .trim()
                .to_string());
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

            return Ok(String::from_utf8(output.stdout)
                .expect("ERROR: \"xbps-query -l | grep ii | wc -l\" output was not valid UTF-8")
                .trim()
                .to_string());
        }

        Err(ReadoutError::MetricNotAvailable)
    }
}
