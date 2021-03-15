use crate::extra;
use crate::traits::*;
use nix::unistd;
use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};
use libc;
use std::cell::UnsafeCell;
use android_properties::AndroidProperty;
use uname::uname;

pub struct AndroidBatteryReadout;

pub struct AndroidKernelReadout {
    os_info: uname::Info,
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
            os_info: uname().unwrap(),
        }
    }

    fn os_release(&self) -> Result<String, ReadoutError> {
        Ok(self.os_info.release.to_string())
    }

    fn os_type(&self) -> Result<String, ReadoutError> {
        Ok(self.os_info.sysname.to_string())
    }
}

impl GeneralReadout for AndroidGeneralReadout {
    fn new() -> Self {
        AndroidGeneralReadout
    }

    fn machine(&self) -> Result<String, ReadoutError> {
        let product_readout = AndroidProductReadout::new();

        let vendor = product_readout.vendor().unwrap_or_default();
        let name = product_readout.name().unwrap_or_default();

        Ok(format!("{} {}", vendor, name))
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
        let product_readout = AndroidProductReadout::new();

        let product = product_readout.product().unwrap_or_default();
        let version = product_readout.version().unwrap_or_default();

        Ok(format!("{} {}", product, version))
    }

    fn desktop_environment(&self) -> Result<String, ReadoutError> {
        Err(ReadoutError::MetricNotAvailable)
    }

    fn window_manager(&self) -> Result<String, ReadoutError> {
        Err(ReadoutError::MetricNotAvailable)
    }

    fn terminal(&self) -> Result<String, ReadoutError> {
        if let Ok(termux_version) = std::env::var("TERMUX_VERSION") {
            return Ok(String::from("Termux"));
        }

        // TODO: investigate other terminal emulators
        Err(ReadoutError::MetricNotAvailable)
    }

    fn shell(&self, shorthand: bool) -> Result<String, ReadoutError> {
        if let Ok(path) = std::env::var("SHELL") {
            if shorthand {
                let path = Path::new(&path);
                return Ok(path.file_stem().unwrap().to_str().unwrap().into());
            }

            return Ok(path);
        }

        Err(ReadoutError::Other(String::from(
            "Unable to read shell for current user.",
        )))
    }

    fn cpu_model_name(&self) -> Result<String, ReadoutError> {
        let max_freq = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq")
            .expect("")// TODO
            .chars()
            .filter(|c| c.is_digit(10))
            .collect();
        let khz = max_freq.parse::<u32>().unwrap_or(0);
        Err(ReadoutError::MetricNotAvailable)
        // Ok(crate::shared::cpu_model_name())
    }

    fn uptime(&self) -> Result<String, ReadoutError> {
        use std::time::{Duration, Instant};

        let mut time = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let ret = unsafe { libc::clock_gettime(libc::CLOCK_BOOTTIME, &mut time) };

        if ret != 0 {
            return Err(ReadoutError::Other(String::from(
              "ERROR: failed to get boot time."
            )));
        }

        let duration = Duration::new(time.tv_sec as u64, time.tv_nsec as u32);

        Ok(duration.as_secs_f64().to_string())
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

    fn vendor(&self) -> Result<String, ReadoutError> {
        Ok(extra::ucfirst(AndroidProperty::new("ro.product.brand")
            .value()
            .unwrap_or_default()
        ))
    }

    fn family(&self) -> Result<String, ReadoutError> {
        Ok(String::from("Linux, Android"))
    }

    fn name(&self) -> Result<String, ReadoutError> {
        Ok(AndroidProperty::new("ro.product.model")
            .value()
            .unwrap_or_default()
        )
    }

    fn version(&self) -> Result<String, ReadoutError> {
        Ok(AndroidProperty::new("ro.build.version.release")
            .value()
            .unwrap_or_default()
        )
    }

    fn product(&self) -> Result<String, ReadoutError> {
        Ok(String::from("Android"))
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
