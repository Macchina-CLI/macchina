use crate::extra;
use crate::traits::*;
use nix::unistd;
use regex::Regex;
use std::process::{Command, Stdio};

pub struct NetBSDBatteryReadout;

pub struct NetBSDKernelReadout;

pub struct NetBSDGeneralReadout {
    local_ip: Option<String>,
}

pub struct NetBSDMemoryReadout;

pub struct NetBSDProductReadout;

pub struct NetBSDPackageReadout;

impl BatteryReadout for NetBSDBatteryReadout {
    fn new() -> Self {
        NetBSDBatteryReadout
    }

    fn percentage(&self) -> Result<u8, ReadoutError> {
        if extra::which("envstat") {
            let envstat = Command::new("envstat")
                .args(&["-s", "acpibat0:charge"])
                .stdout(Stdio::piped())
                .output()
                .expect("ERROR: failed to spawn \"envstat\" process");

            let envstat_out = String::from_utf8(envstat.stdout)
                .expect("ERROR: \"envstat\" process stdout was not valid UTF-8");
            if envstat_out.is_empty() {
                return Err(ReadoutError::MetricNotAvailable);
            } else {
                let re = Regex::new(r"\(([^()]*)\)").unwrap();
                let caps = re.captures(&envstat_out);
                match caps {
                    Some(c) => {
                        let percentage = c
                            .get(1)
                            .map_or("", |m| m.as_str())
                            .to_string()
                            .replace("%", "");
                        let percentage_f = percentage.parse::<f32>().unwrap();
                        let percentage_i = percentage_f.round() as u8;
                        return Ok(percentage_i);
                    }
                    None => return Err(ReadoutError::MetricNotAvailable),
                }
            }
        }

        Err(ReadoutError::MetricNotAvailable)
    }

    fn status(&self) -> Result<BatteryState, ReadoutError> {
        if extra::which("envstat") {
            let envstat = Command::new("envstat")
                .args(&["-s", "acpibat0:charging"])
                .stdout(Stdio::piped())
                .output()
                .expect("ERROR: failed to spawn \"envstat\" process");

            let envstat_out = String::from_utf8(envstat.stdout)
                .expect("ERROR: \"envstat\" process stdout was not valid UTF-8");

            if envstat_out.is_empty() {
                return Err(ReadoutError::MetricNotAvailable);
            } else {
                if envstat_out.contains("TRUE") {
                    return Ok(BatteryState::Charging);
                } else {
                    return Ok(BatteryState::Discharging);
                }
            }
        }

        Err(ReadoutError::Other(format!("envstat is not installed")))
    }
}

impl KernelReadout for NetBSDKernelReadout {
    fn new() -> Self {
        NetBSDKernelReadout
    }

    fn os_release(&self) -> Result<String, ReadoutError> {
        let output = Command::new("sysctl")
            .args(&["-n", "-b", "kern.osrelease"])
            .output()
            .expect("ERROR: failed to fetch \"kernel.osrelease\" using \"sysctl\"");

        let osrelease = String::from_utf8(output.stdout)
            .expect("ERROR: \"sysctl\" process stdout was not valid UTF-8");

        Ok(String::from(osrelease))
    }

    fn os_type(&self) -> Result<String, ReadoutError> {
        // sysctl -e -n -b kernel.osrelease
        let output = Command::new("sysctl")
            .args(&["-n", "-b", "kern.ostype"])
            .output()
            .expect("ERROR: failed to fetch \"kernel.ostype\" using \"sysctl\"");

        let osrelease = String::from_utf8(output.stdout)
            .expect("ERROR: \"sysctl\" process stdout was not valid UTF-8");

        Ok(String::from(osrelease))
    }

    fn pretty_kernel(&self) -> Result<String, ReadoutError> {
        Err(ReadoutError::MetricNotAvailable)
    }
}

impl GeneralReadout for NetBSDGeneralReadout {
    fn new() -> Self {
        NetBSDGeneralReadout {
            local_ip: local_ipaddress::get(),
        }
    }

    fn machine(&self) -> Result<String, ReadoutError> {
        let product_readout = NetBSDProductReadout::new();

        let vendor = product_readout
            .vendor()
            .unwrap_or(String::new())
            .replace("To be filled by O.E.M.", "")
            .trim()
            .to_string();

        let product = product_readout
            .product()
            .unwrap_or(String::new())
            .replace("To be filled by O.E.M.", "")
            .trim()
            .to_string();

        let version = product_readout
            .version()
            .unwrap_or(String::new())
            .replace("To be filled by O.E.M.", "")
            .trim()
            .to_string();

        if version == product && version == vendor {
            return Ok(vendor);
        }

        Ok(format!("{} {} {}", vendor, product, version))
    }

    fn local_ip(&self) -> Result<String, ReadoutError> {
        Ok(self
            .local_ip
            .as_ref()
            .ok_or(ReadoutError::MetricNotAvailable)?
            .to_string())
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
                "Failed to retrieve hostname from 'gethostname'.",
            ))),
        }
    }

    fn distribution(&self) -> Result<String, ReadoutError> {
        Err(ReadoutError::Warning(String::from(
            "Since you're on NetBSD, there is no distribution to be read from the system.",
        )))
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

    fn uptime(&self) -> Result<usize, ReadoutError> {
        crate::shared::uptime()
    }

    fn os_name(&self) -> Result<String, ReadoutError> {
        let kernel_readout = NetBSDKernelReadout::new();

        let os_type = kernel_readout.os_type()?;
        let os_release = kernel_readout.os_release()?;

        if !(os_type.is_empty() || os_release.is_empty()) {
            return Ok(format!("{} {}", os_type, os_release));
        }

        Err(ReadoutError::MetricNotAvailable)
    }
}

impl MemoryReadout for NetBSDMemoryReadout {
    fn new() -> Self {
        NetBSDMemoryReadout
    }

    fn total(&self) -> Result<u64, ReadoutError> {
        Ok(crate::shared::get_meminfo_value("MemTotal"))
    }

    fn free(&self) -> Result<u64, ReadoutError> {
        Ok(crate::shared::get_meminfo_value("MemFree"))
    }

    fn used(&self) -> Result<u64, ReadoutError> {
        let total = self.total().unwrap();
        let free = self.free().unwrap();

        Ok(total - free)
    }
}

impl ProductReadout for NetBSDProductReadout {
    fn new() -> Self {
        NetBSDProductReadout
    }

    fn version(&self) -> Result<String, ReadoutError> {
        let output = Command::new("sysctl")
            .args(&["-n", "-b", "machdep.dmi.system-version"])
            .output()
            .expect("ERROR: failed to start \"sysctl\" process");

        let sysver = String::from_utf8(output.stdout)
            .expect("ERROR: \"sysctl\" process stdout was not valid UTF-8");

        Ok(String::from(sysver))
    }

    fn vendor(&self) -> Result<String, ReadoutError> {
        let output = Command::new("sysctl")
            .args(&["-n", "-b", "machdep.dmi.system-vendor"])
            .output()
            .expect("ERROR: failed to start \"sysctl\" process");

        let sysven = String::from_utf8(output.stdout)
            .expect("ERROR: \"sysctl\" process stdout was not valid UTF-8");

        Ok(String::from(sysven))
    }

    fn product(&self) -> Result<String, ReadoutError> {
        let output = Command::new("sysctl")
            .args(&["-n", "-b", "machdep.dmi.system-product"])
            .output()
            .expect("ERROR: failed to start \"sysctl\" process");

        let sysprod = String::from_utf8(output.stdout)
            .expect("ERROR: \"sysctl\" process stdout was not valid UTF-8");

        Ok(String::from(sysprod))
    }
}

impl PackageReadout for NetBSDPackageReadout {
    fn new() -> Self {
        NetBSDPackageReadout
    }

    fn count_pkgs(&self) -> Vec<(PackageManager, usize)> {
        let mut packages = Vec::new();
        // Instead of having a condition for each distribution.
        // we will try and extract package count by checking
        // if a certain package manager is installed
        if extra::which("pkgin") {
            match NetBSDPackageReadout::count_pkgin() {
                Some(c) => packages.push((PackageManager::Pkgsrc, c)),
                _ => (),
            }
        }

        packages
    }
}

impl NetBSDPackageReadout {
    /// Counts the number of packages for the pkgin package manager
    fn count_pkgin() -> Option<usize> {
        let pkg_info = Command::new("pkg_info")
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to spawn \"pkg_info\" process");

        let pkg_out = pkg_info
            .stdout
            .expect("ERROR: failed to open \"pkg_info\" stdout");

        let count = Command::new("wc")
            .arg("-l")
            .stdin(Stdio::from(pkg_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to start \"wc\" process");

        let output = count
            .wait_with_output()
            .expect("ERROR: failed to wait on for \"wc\" process to exit");
        String::from_utf8(output.stdout)
            .expect("ERROR: \"pkg_info | wc -l\" output was not valid UTF-8")
            .trim()
            .parse::<usize>()
            .ok()
    }
}
