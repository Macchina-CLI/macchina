use crate::extra;
use crate::traits::*;
use nix::unistd;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use sysctl::{Ctl, Sysctl};

pub struct LinuxBatteryReadout;

pub struct LinuxKernelReadout {
    os_release_ctl: Option<Ctl>,
    os_type_ctl: Option<Ctl>,
}

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
        LinuxKernelReadout {
            os_release_ctl: Ctl::new("kernel.osrelease").ok(),
            os_type_ctl: Ctl::new("kernel.ostype").ok(),
        }
    }

    fn os_release(&self) -> Result<String, ReadoutError> {
        Ok(self
            .os_release_ctl
            .as_ref()
            .ok_or(ReadoutError::MetricNotAvailable)?
            .value_string()?)
    }

    fn os_type(&self) -> Result<String, ReadoutError> {
        Ok(self
            .os_type_ctl
            .as_ref()
            .ok_or(ReadoutError::MetricNotAvailable)?
            .value_string()?)
    }
}

impl GeneralReadout for LinuxGeneralReadout {
    fn new() -> Self {
        LinuxGeneralReadout
    }

    fn machine(&self) -> Result<String, ReadoutError> {
        let product_readout = LinuxProductReadout::new();

        let name = product_readout.name()?;
        let family = product_readout.family().unwrap_or_default();
        let version = product_readout.version().unwrap_or_default();

        if family == name && family == version {
            return Ok(family);
        } else if version.is_empty() || version.len() <= 15 {
            let vendor = product_readout.vendor().unwrap_or_default();

            if !vendor.is_empty() {
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
                Ok(String::from(hostname))
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
            let pacman_output = Command::new("pacman")
                .args(&["-Q", "-q"])
                .stdout(Stdio::piped())
                .spawn()
                .expect("ERROR: failed to start \"pacman\" process")
                .stdout
                .expect("ERROR: failed to open \"pacman\" stdout");

            let count = Command::new("wc")
                .arg("-l")
                .stdin(Stdio::from(pacman_output))
                .stdout(Stdio::piped())
                .spawn()
                .expect("ERROR: failed to start \"wc\" process");

            let final_output = count
                .wait_with_output()
                .expect("ERROR: failed to wait for \"wc\" process to exit");
            return Ok(String::from_utf8(final_output.stdout)
                .expect("ERROR: \"pacman -Qq | wc -l\" output was not valid UTF-8")
                .trim()
                .to_string());
        } else if extra::which("dpkg") {
            let dpkg_output = Command::new("dpkg")
                .arg("-l")
                .stdout(Stdio::piped())
                .spawn()
                .expect("ERROR: failed to spawn \"dpkg\" process")
                .stdout
                .expect("ERROR: failed to open \"dpkg\" stdout");

            let count = Command::new("wc")
                .arg("-l")
                .stdin(Stdio::from(dpkg_output))
                .stdout(Stdio::piped())
                .spawn()
                .expect("ERROR: failed to spawn \"wc\" process");

            let final_output = count
                .wait_with_output()
                .expect("ERROR: failed to wait for \"wc\" process to exit");
            return Ok(String::from_utf8(final_output.stdout)
                .expect("ERROR: \"dpkg -l | wc -l\" output was not valid UTF-8")
                .trim()
                .to_string());
        } else if extra::which("qlist") {
            let qlist_output = Command::new("qlist")
                .arg("-I")
                .stdout(Stdio::piped())
                .spawn()
                .expect("ERROR: failed to spawn \"qlist\" process")
                .stdout
                .expect("ERROR: failed to open \"qlist\" stdout");

            let count = Command::new("wc")
                .arg("-l")
                .stdin(Stdio::from(qlist_output))
                .stdout(Stdio::piped())
                .spawn()
                .expect("ERROR: failed to spawn \"wc\" process");

            let final_output = count
                .wait_with_output()
                .expect("ERROR: failed to wait for \"wc\" process to exit");
            return Ok(String::from_utf8(final_output.stdout)
                .expect("ERROR: \"qlist -I | wc -l\" output was not valid UTF-8")
                .trim()
                .to_string());
        } else if extra::which("xbps-query") {
            let xbps_output = Command::new("xbps-query")
                .arg("-l")
                .stdout(Stdio::piped())
                .spawn()
                .expect("ERROR: failed to spawn \"xbps-query\" process")
                .stdout
                .expect("ERROR: failed to open \"xbps-query\" stdout");

            let grep_output = Command::new("grep")
                .arg("ii")
                .stdin(Stdio::from(xbps_output))
                .stdout(Stdio::piped())
                .spawn()
                .expect("ERROR: failed to spawn \"grep\" process")
                .stdout
                .expect("ERROR: failed to read \"grep\" stdout");

            let count = Command::new("wc")
                .arg("-l")
                .stdin(Stdio::from(grep_output))
                .stdout(Stdio::piped())
                .spawn()
                .expect("ERROR: failed to spawn \"wc\" process");

            let final_output = count
                .wait_with_output()
                .expect("ERROR: failed to wait for \"wc\" process to exit");

            return Ok(String::from_utf8(final_output.stdout)
                .expect("ERROR: \"xbps-query -l | grep ii | wc -l\" output was not valid UTF-8")
                .trim()
                .to_string());
        }

        Err(ReadoutError::MetricNotAvailable)
    }
}
