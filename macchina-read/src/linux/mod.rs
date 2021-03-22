use crate::extra;
use crate::traits::*;
use local_ipaddress;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use sysctl::{Ctl, Sysctl};

impl From<sqlite::Error> for ReadoutError {
    fn from(e: sqlite::Error) -> Self {
        ReadoutError::Other(e.to_string())
    }
}

pub struct LinuxBatteryReadout;

pub struct LinuxKernelReadout {
    os_release_ctl: Option<Ctl>,
    os_type_ctl: Option<Ctl>,
}

pub struct LinuxGeneralReadout {
    hostname_ctl: Option<Ctl>,
    local_ip: Option<String>,
}

pub struct LinuxMemoryReadout;

pub struct LinuxProductReadout;

pub struct LinuxPackageReadout;

impl BatteryReadout for LinuxBatteryReadout {
    fn new() -> Self {
        LinuxBatteryReadout
    }

    fn percentage(&self) -> Result<u8, ReadoutError> {
        let mut bat_path = Path::new("/sys/class/power_supply/BAT0/capacity");
        if !Path::exists(bat_path) {
            bat_path = Path::new("/sys/class/power_supply/BAT1/capacity");
        }

        let percentage_text = extra::pop_newline(fs::read_to_string(bat_path)?);
        let percentage_parsed = percentage_text.parse::<u8>();

        match percentage_parsed {
            Ok(p) => Ok(p),
            Err(e) => Err(ReadoutError::Other(format!(
                "Could not parse the value '{}' of {} into a \
            digit: {:?}",
                percentage_text,
                bat_path.to_str().unwrap_or_default(),
                e
            ))),
        }
    }

    fn status(&self) -> Result<BatteryState, ReadoutError> {
        let mut bat_path = Path::new("/sys/class/power_supply/BAT0/status");
        if !Path::exists(bat_path) {
            bat_path = Path::new("/sys/class/power_supply/BAT1/status");
        }

        let status_text = extra::pop_newline(fs::read_to_string(bat_path)?).to_lowercase();
        match &status_text[..] {
            "charging" => Ok(BatteryState::Charging),
            "discharging" | "full" => Ok(BatteryState::Discharging),
            s => Err(ReadoutError::Other(format!(
                "Got unexpected value '{}' from {}.",
                s,
                bat_path.to_str().unwrap_or_default()
            ))),
        }
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
        LinuxGeneralReadout {
            hostname_ctl: Ctl::new("kernel.hostname").ok(),
            local_ip: local_ipaddress::get(),
        }
    }

    fn machine(&self) -> Result<String, ReadoutError> {
        let product_readout = LinuxProductReadout::new();

        let name = product_readout
            .name()?
            .replace("To be filled by O.E.M.", "")
            .trim()
            .to_string();

        let family = product_readout
            .family()
            .unwrap_or_default()
            .replace("To be filled by O.E.M.", "")
            .trim()
            .to_string();

        let version = product_readout
            .version()
            .unwrap_or_default()
            .replace("To be filled by O.E.M.", "")
            .trim()
            .to_string();

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
        Ok(self
            .hostname_ctl
            .as_ref()
            .ok_or(ReadoutError::MetricNotAvailable)?
            .value_string()?)
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

    fn uptime(&self) -> Result<usize, ReadoutError> {
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
        Ok(crate::shared::get_meminfo_value("Cached"))
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

    /// Returns the __number of installed packages__ for the following package managers:
    /// - pacman
    /// - apk _(using apk info )_
    /// - emerge _(using qlist)_
    /// - apt _(using dpkg)_
    /// - xbps _(using xbps-query)_
    /// - rpm
    fn count_pkgs(&self) -> Vec<(PackageManager, usize)> {
        let mut packages = Vec::new();
        // Instead of having a condition for each distribution.
        // we will try and extract package count by checking
        // if a certain package manager is installed
        if extra::which("pacman") {
            match LinuxPackageReadout::count_pacman() {
                Some(c) => packages.push((PackageManager::Pacman, c)),
                _ => (),
            }
        } else if extra::which("dpkg") {
            match LinuxPackageReadout::count_apt() {
                Some(c) => packages.push((PackageManager::Apt, c)),
                _ => (),
            }
        } else if extra::which("qlist") {
            match LinuxPackageReadout::count_portage() {
                Some(c) => packages.push((PackageManager::Portage, c)),
                _ => (),
            }
        } else if extra::which("xbps-query") {
            match LinuxPackageReadout::count_xbps() {
                Some(c) => packages.push((PackageManager::Xbps, c)),
                _ => (),
            }
        } else if extra::which("apk") {
            match LinuxPackageReadout::count_apk() {
                Some(c) => packages.push((PackageManager::Apk, c)),
                _ => (),
            }
        } else if extra::which("rpm") {
            match LinuxPackageReadout::count_rpm() {
                Ok(c) => packages.push((PackageManager::Pacman, c)),
                _ => (),
            }
        }

        packages
    }
}

impl LinuxPackageReadout {
    fn count_rpm() -> Result<usize, ReadoutError> {
        let path = "/var/lib/rpm/rpmdb.sqlite";
        let connection = sqlite::open(path);
        match connection {
            Ok(con) => {
                let mut statement = con.prepare("SELECT COUNT(*) FROM Installtid")?;
                statement.next()?;

                return match statement.read::<Option<i64>>(0) {
                    Ok(Some(count)) => Ok(count as usize),
                    Ok(_) => Ok(0),
                    Err(err) => Err(ReadoutError::Other(format!(
                        "Could not read package count \
                    from sqlite database table 'Installtid': {:?}",
                        err
                    ))),
                };
            }
            Err(_) => Err(ReadoutError::MetricNotAvailable),
        }
    }

    fn count_pacman() -> Option<usize> {
        use std::fs::read_dir;
        use std::path::Path;

        let pacman_folder = Path::new("/var/lib/pacman/local");
        if pacman_folder.exists() {
            match read_dir(pacman_folder) {
                Ok(read_dir) => return Some(read_dir.count() - 1),
                _ => (),
            };
        }

        // Returns the number of installed packages using
        // pacman -Qq | wc -l
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

        String::from_utf8(final_output.stdout)
            .expect("ERROR: \"pacman -Qq | wc -l\" output was not valid UTF-8")
            .trim()
            .parse::<usize>()
            .ok()
    }

    fn count_apt() -> Option<usize> {
        // Returns the number of installed packages using
        // dpkg -l | wc -l
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

        String::from_utf8(final_output.stdout)
            .expect("ERROR: \"dpkg -l | wc -l\" output was not valid UTF-8")
            .trim()
            .parse::<usize>()
            .ok()
    }

    fn count_portage() -> Option<usize> {
        // Returns the number of installed packages using:
        // qlist -I | wc -l
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

        String::from_utf8(final_output.stdout)
            .expect("ERROR: \"qlist -I | wc -l\" output was not valid UTF-8")
            .trim()
            .parse::<usize>()
            .ok()
    }

    fn count_xbps() -> Option<usize> {
        // Returns the number of installed packages using:
        // xbps-query | grep ii | wc -l
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

        String::from_utf8(final_output.stdout)
            .expect("ERROR: \"xbps-query -l | grep ii | wc -l\" output was not valid UTF-8")
            .trim()
            .parse::<usize>()
            .ok()
    }

    fn count_apk() -> Option<usize> {
        // Returns the number of installed packages using:
        // apk info | wc -l
        let apk_output = Command::new("apk")
            .arg("info")
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to start \"apk\" process")
            .stdout
            .expect("ERROR: failed to open \"apk\" stdout");

        let count = Command::new("wc")
            .arg("-l")
            .stdin(Stdio::from(apk_output))
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to start \"wc\" process");

        let final_output = count
            .wait_with_output()
            .expect("ERROR: failed to wait for \"wc\" process to exit");

        String::from_utf8(final_output.stdout)
            .expect("ERROR: \"apk info | wc -l\" output was not valid UTF-8")
            .trim()
            .parse::<usize>()
            .ok()
    }
}
