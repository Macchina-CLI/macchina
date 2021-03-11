use crate::traits::*;
use crate::extra;
use nix::unistd;
use std::process::{Command, Stdio};

pub struct NetBSDBatteryReadout;

pub struct NetBSDKernelReadout;

pub struct NetBSDGeneralReadout;

pub struct NetBSDMemoryReadout;

pub struct NetBSDProductReadout;

pub struct NetBSDPackageReadout;

impl BatteryReadout for NetBSDBatteryReadout {

    fn new() -> Self {
        NetBSDBatteryReadout
    }

    fn percentage(&self) -> Result<String, ReadoutError> { 
        if extra::which("rg") {
            let envstat = Command::new("envstat")
                .args(&["-d", "acpibat0"])
                .stdout(Stdio::piped())
                .spawn()
                .expect("ERROR: failed to spawn \"envstat\" process");
    
            let envstat_out = envstat
                .stdout
                .expect("ERROR: failed to open \"envstat\" stdout");
    
            let rg = Command::new("rg")
                .args(&["-o", "-P", r"(?<=\().*(?=\))"])
                .stdin(Stdio::from(envstat_out))
                .stdout(Stdio::piped())
                .spawn()
                .expect("ERROR: failed to spawn \"rg\" process");
            let output = rg
                .wait_with_output()
                .expect("ERROR: failed to wait for \"rg\" process to exit");
            let perc_str = String::from_utf8(output.stdout)
                .expect("ERROR: \"rg\" process output was not valid UTF-8");
            let percentage = perc_str.trim().split(".").next().unwrap_or("").to_string();
    
            if percentage.is_empty() {
                return Err(ReadoutError::MetricNotAvailable);
            }

            return Ok(percentage)
        }
        
        Err(ReadoutError::MetricNotAvailable)
    }

    fn status(&self) -> Result<String, ReadoutError> { 
        if extra::which("rg") {
            let envstat = Command::new("envstat")
                .args(&["-d", "acpibat0"])
                .stdout(Stdio::piped())
                .spawn()
                .expect("ERROR: failed to spawn \"envstat\" process");
    
            let envstat_out = envstat
                .stdout
                .expect("ERROR: failed to open \"envstat\" stdout");
    
            let grep = Command::new("rg")
                .arg("charging:")
                .stdin(Stdio::from(envstat_out))
                .stdout(Stdio::piped())
                .spawn()
                .expect("ERROR: failed to spawn \"rg\" process");
    
            let output = grep
                .wait_with_output()
                .expect("ERROR: failed to wait for \"rg\" process to exit");
            let mut status = String::from_utf8(output.stdout)
                .expect("ERROR: \"grep\" process output was not valid UTF-8");
            status = status.replace("charging:", "").trim().to_string();
            if status.is_empty() {
                return Err(ReadoutError::MetricNotAvailable)
            }

            return Ok(status)
        }
        
        Err(ReadoutError::MetricNotAvailable)
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

    fn pretty_kernel(&self) -> Result<String, ReadoutError> { Err(ReadoutError::MetricNotAvailable) }

}

impl GeneralReadout for NetBSDGeneralReadout {

    fn new() -> Self {
        NetBSDGeneralReadout
    }

    fn machine(&self) -> Result<String, ReadoutError> {
        let product_readout = NetBSDProductReadout::new();

        let vendor = product_readout.vendor().unwrap_or(String::new());
        let product = product_readout.product().unwrap_or(String::new());
        let version = product_readout.version().unwrap_or(String::new());

        if version == product && version == vendor {
            return Ok(vendor);
        }
        
        Ok(format!("{} {} {}", vendor, product, version))
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
            Err(_e) => Err(ReadoutError::Other(String::from("Failed to retrieve hostname from 'gethostname'.")))
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

    fn count_pkgs(&self) -> Result<String, ReadoutError> { 
        if extra::which("pkg_info") {
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
            return Ok(String::from_utf8(output.stdout)
                .expect("ERROR: \"pkg_info | wc -l\" output was not valid UTF-8")
                .trim()
                .to_string());
        }
        
        Err(ReadoutError::MetricNotAvailable)
    }

}