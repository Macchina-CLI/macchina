use crate::extra;
use crate::traits::*;
use nix::unistd;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use sysctl::{Ctl, Sysctl};

pub struct AndroidBatteryReadout;

pub struct AndroidKernelReadout {
    os_release_ctl: Option<Ctl>,
    os_type_ctl: Option<Ctl>,
}

pub struct AndroidGeneralReadout;

pub struct AndroidMemoryReadout;

pub struct AndroidProductReadout;

pub struct AndroidPackageReadout;

impl BatteryReadout for AndroidBatteryReadout {
    fn new() -> Self {
        AndroidBatteryReadout
    }

    fn percentage(&self) -> Result<String, ReadoutError> {
        Err(ReadoutError::MetricNotAvailable)
        // let mut bat_path = Path::new("/sys/class/power_supply/BAT0/capacity");
        // if !Path::exists(bat_path) {
            // bat_path = Path::new("/sys/class/power_supply/BAT1/capacity");
        // }
//
        // Ok(extra::pop_newline(fs::read_to_string(bat_path)?))
    }

    fn status(&self) -> Result<String, ReadoutError> {
        Err(ReadoutError::MetricNotAvailable)
        // let mut bat_path = Path::new("/sys/class/power_supply/BAT0/status");
        // if !Path::exists(bat_path) {
            // bat_path = Path::new("/sys/class/power_supply/BAT1/status");
        // }
//
        // Ok(extra::pop_newline(fs::read_to_string(bat_path)?))
    }
}

impl KernelReadout for AndroidKernelReadout {
    fn new() -> Self {
        AndroidKernelReadout {
            os_release_ctl: Ctl::new("kernel/osrelease").ok(),
            os_type_ctl: Ctl::new("kernel/ostype").ok(),
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

impl GeneralReadout for AndroidGeneralReadout {
    fn new() -> Self {
        AndroidGeneralReadout
    }

    fn machine(&self) -> Result<String, ReadoutError> {
        let product_readout = AndroidProductReadout::new();

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
        // Err(ReadoutError::MetricNotAvailable)
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
        // getprop ro.build.version.release
        let release = Command::new("getprop")
            .arg("ro.build.version.release")
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to spawn \"getprop\" process");

        let output = release
            .wait_with_output()
            .expect("ERROR: failed to wait for \"getprop ro.build.version.release\" process to exit");

        let version = String::from_utf8(output.stdout)
            .expect("ERROR: \"getprop ro.build.version.release\" output was not valid UTF-8");

        Ok(format!("Android {}", version))
    }

    fn desktop_environment(&self) -> Result<String, ReadoutError> {
        Err(ReadoutError::MetricNotAvailable)
        // crate::shared::desktop_environment()
    }

    fn window_manager(&self) -> Result<String, ReadoutError> {
        Err(ReadoutError::MetricNotAvailable)
        // crate::shared::window_manager()
    }

    fn terminal(&self) -> Result<String, ReadoutError> {
        // Err(ReadoutError::MetricNotAvailable)
        crate::shared::terminal()
    }

    fn shell(&self, shorthand: bool) -> Result<String, ReadoutError> {

        // Err(ReadoutError::MetricNotAvailable)
        crate::shared::shell(shorthand)
    }

    fn cpu_model_name(&self) -> Result<String, ReadoutError> {
        Err(ReadoutError::MetricNotAvailable)
        // Ok(crate::shared::cpu_model_name())
    }

    fn uptime(&self) -> Result<String, ReadoutError> {
        // boot=$(date -d"$(uptime -s)" +%s)
        // now=$(date +%s)
        // s=$((now - boot))
        Err(ReadoutError::MetricNotAvailable)
        // crate::shared::uptime()
    }
}

impl MemoryReadout for AndroidMemoryReadout {
    fn new() -> Self {
        AndroidMemoryReadout
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

impl ProductReadout for AndroidProductReadout {
    fn new() -> Self {
        AndroidProductReadout
    }

    fn version(&self) -> Result<String, ReadoutError> {
        Err(ReadoutError::MetricNotAvailable)
        // Ok(extra::pop_newline(fs::read_to_string(
            // "/sys/class/dmi/id/product_version",
        // )?))
    }

    fn vendor(&self) -> Result<String, ReadoutError> {
        Err(ReadoutError::MetricNotAvailable)
        // Ok(extra::pop_newline(fs::read_to_string(
            // "/sys/class/dmi/id/sys_vendor",
        // )?))
    }

    fn family(&self) -> Result<String, ReadoutError> {
        Err(ReadoutError::MetricNotAvailable)
        // Ok(extra::pop_newline(fs::read_to_string(
            // "/sys/class/dmi/id/product_family",
        // )?))
    }

    fn name(&self) -> Result<String, ReadoutError> {
        Err(ReadoutError::MetricNotAvailable)
        // Ok(extra::pop_newline(fs::read_to_string(
            // "/sys/class/dmi/id/product_name",
        // )?))
    }
}

impl PackageReadout for AndroidPackageReadout {
    fn new() -> Self {
        AndroidPackageReadout
    }

    fn count_pkgs(&self) -> Result<String, ReadoutError> {
        if extra::which("dpkg") {
            let dpkg_output = Command::new("dpkg")
                .arg("-l")
                .stdout(Stdio::piped())
                .spawn()
                .expect("ERROR: failed to spawn \"dpkg\" process")
                .stdout
                .expect("ERROR: failed to open \"dpkg\" stdout");

            let grep_output = Command::new("grep")
                .arg("ii")
                .stdin(Stdio::from(dpkg_output))
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
                .expect("ERROR: \"dpkg -l | grep ii |  wc -l\" output was not valid UTF-8")
                .trim()
                .to_string());
        }

        Err(ReadoutError::MetricNotAvailable)
    }
}
