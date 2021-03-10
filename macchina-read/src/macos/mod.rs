use crate::traits::*;
use sysctl::{Sysctl, Ctl, SysctlError};
use crate::traits::ReadoutError::MetricNotAvailable;
use mach::vm_statistics::{vm_statistics_data_t};
use crate::macos::mach_ffi::{IOPSCopyPowerSourcesInfo, IOPSGetProvidingPowerSourceType};
use core_foundation::base::CFRelease;
use core_foundation::string::{CFStringGetCStringPtr, kCFStringEncodingUTF8};

mod mach_ffi;

impl From<SysctlError> for ReadoutError {
    fn from(e: SysctlError) -> Self {
        ReadoutError::SysctlError(format!("Error while accessing system control: {:?}", e))
    }
}

pub struct MacOSBatteryReadout;

enum MacOSPowerSource {
    Battery,
    AC,
    UPS
}

pub struct MacOSProductReadout;

pub struct MacOSKernelReadout {
    os_type_ctl: Option<Ctl>,
    os_release_ctl: Option<Ctl>,
}

pub struct MacOSGeneralReadout {
    cpu_brand_ctl: Option<Ctl>,
    boot_time_ctl: Option<Ctl>,
    hostname_ctl: Option<Ctl>,
    hw_model_ctl: Option<Ctl>,
}

pub struct MacOSMemoryReadout {
    page_size_ctl: Option<Ctl>,
}

pub struct MacOSPackageReadout;

impl BatteryReadout for MacOSBatteryReadout {
    fn new() -> Self {
        MacOSBatteryReadout
    }

    fn percentage(&self) -> Result<String, ReadoutError> {
        if self.get_power_source() != MacOSPowerSource::Battery { return Err(MetricNotAvailable); }

        Ok(String::new())
    }

    fn status(&self) -> Result<String, ReadoutError> {
        if self.get_power_source() != MacOSPowerSource::Battery { return Err(MetricNotAvailable); }

        Ok(String::new())
    }
}

impl MacOSBatteryReadout {
    fn get_power_source(&self) -> MacOSPowerSource {
        let power_info = unsafe { IOPSCopyPowerSourcesInfo() };
        let providing_power_cfstr = unsafe { IOPSGetProvidingPowerSourceType(power_info) };
        let cstr_ptr = unsafe { CFStringGetCStringPtr(providing_power_cfstr, kCFStringEncodingUTF8) };
        let mut power_source = unsafe {
            match std::ffi::CStr::from_ptr(cstr_ptr).to_str() {
                Some(mach_ffi::kIOPMBatteryPowerKey) => MacOSPowerSource::Battery,
                Some(mach_ffi::kIOPMACPowerKey) => MacOSPowerSource::AC,
                Some(mach_ffi::kIOPMUPSPowerKey) => MacOSPowerSource::UPS,
            }
        };

        unsafe { CFRelease(power_info); };

        power_source
    }
}

impl KernelReadout for MacOSKernelReadout {
    fn new() -> Self {
        MacOSKernelReadout {
            os_type_ctl: Ctl::new("kern.ostype").ok(),
            os_release_ctl: Ctl::new("kern.osrelease").ok(),
        }
    }

    fn os_release(&self) -> Result<String, ReadoutError> {
        Ok(self.os_release_ctl.as_ref().ok_or(MetricNotAvailable)?.value_string()?)
    }

    fn os_type(&self) -> Result<String, ReadoutError> {
        Ok(self.os_type_ctl.as_ref().ok_or(MetricNotAvailable)?.value_string()?)
    }
}

impl GeneralReadout for MacOSGeneralReadout {
    fn new() -> Self {
        MacOSGeneralReadout {
            cpu_brand_ctl: Ctl::new("machdep.cpu.brand_string").ok(),
            boot_time_ctl: Ctl::new("kern.boottime").ok(),
            hostname_ctl: Ctl::new("kern.hostname").ok(),
            hw_model_ctl: Ctl::new("hw.model").ok(),
        }
    }

    fn username(&self) -> Result<String, ReadoutError> {
        crate::shared::whoami()
    }

    fn hostname(&self) -> Result<String, ReadoutError> {
        Ok(self.hostname_ctl.as_ref().ok_or(MetricNotAvailable)?.value_string()?)
    }

    fn desktop_environment(&self) -> Result<String, ReadoutError> {
        Ok(String::from("Aqua"))
    }

    fn window_manager(&self) -> Result<String, ReadoutError> {
        Ok(String::from("Quartz Compositor"))
    }

    fn terminal(&self) -> Result<String, ReadoutError> {
        if let Some(terminal_env) = std::env::var("TERM").ok() {
            return Ok(terminal_env);
        }

        crate::shared::terminal()
    }

    fn shell(&self, shorthand: bool) -> Result<String, ReadoutError> {
        crate::shared::shell(shorthand)
    }

    fn cpu_model_name(&self) -> Result<String, ReadoutError> {
        Ok(self.cpu_brand_ctl.as_ref().ok_or(MetricNotAvailable)?.value_string()?)
    }

    fn uptime(&self) -> Result<String, ReadoutError> {
        use std::time::{Duration, SystemTime, UNIX_EPOCH};
        use libc::timeval;

        let time = self.boot_time_ctl.as_ref().ok_or(MetricNotAvailable)?.value_as::<timeval>()?;
        let duration = Duration::new(time.tv_sec as u64, (time.tv_usec * 1000) as
            u32);
        let bootup_timestamp = UNIX_EPOCH + duration;

        if let Ok(duration) = SystemTime::now().duration_since(bootup_timestamp) {
            let seconds_since_boot = duration.as_secs_f64();
            return Ok(seconds_since_boot.to_string());
        }

        Err(ReadoutError::Other(String::from("Error calculating boot time since unix \
            epoch.")))
    }

    fn machine(&self) -> Result<String, ReadoutError> {
        Ok(self.hw_model_ctl.as_ref().ok_or(MetricNotAvailable)?.value_string()?)
    }



}

impl MemoryReadout for MacOSMemoryReadout {
    fn new() -> Self {
        MacOSMemoryReadout {
            page_size_ctl: Ctl::new("hw.pagesize").ok()
        }
    }

    fn total(&self) -> Result<u64, ReadoutError> {
        let vm_stats = MacOSMemoryReadout::mach_vm_stats()?;
        let page_size = self.mach_hw_pagesize()?;

        let total = ((vm_stats.wire_count + vm_stats.active_count + vm_stats.inactive_count + vm_stats
            .free_count + vm_stats.speculative_count) as u64) * page_size / 1024;

        Ok(total)
    }

    fn free(&self) -> Result<u64, ReadoutError> {
        let vm_stats = MacOSMemoryReadout::mach_vm_stats()?;
        let page_size = self.mach_hw_pagesize()?;

        Ok((vm_stats.free_count as u64) * page_size / 1024)
    }

    fn buffers(&self) -> Result<u64, ReadoutError> {
        //todo
        Ok(0)
    }

    fn cached(&self) -> Result<u64, ReadoutError> {
        //todo
        Ok(0)
    }

    fn reclaimable(&self) -> Result<u64, ReadoutError> {
        //todo
        Ok(0)
    }

    fn used(&self) -> Result<u64, ReadoutError> {
        let total = self.total()?;
        let free = self.free()?;

        Ok(total - free)
    }
}

impl MacOSMemoryReadout {
    fn mach_vm_stats() -> Result<vm_statistics_data_t, ReadoutError> {
        use mach::message::{mach_msg_type_number_t};
        use mach::kern_return::KERN_SUCCESS;
        use mach::vm_types::{integer_t};
        use mach_ffi::*;

        const HOST_VM_INFO_COUNT: mach_msg_type_number_t =
            (std::mem::size_of::<vm_statistics_data_t>() /
                std::mem::size_of::<integer_t>()) as u32;

        const HOST_VM_INFO: integer_t = 2;

        let mut vmstat: vm_statistics_data_t = std::default::Default::default();
        let vmstat_ptr: *mut vm_statistics_data_t = &mut vmstat;
        let mut count: mach_msg_type_number_t = HOST_VM_INFO_COUNT;

        let ret_val = unsafe {
            host_statistics(mach_host_self(), HOST_VM_INFO, vmstat_ptr as *mut integer_t, &mut
                count as *mut mach_msg_type_number_t)
        };

        if ret_val == KERN_SUCCESS {
            return Ok(vmstat);
        }

        Err(ReadoutError::Other(String::from("Could not retrieve vm statistics from host.")))
    }

    fn mach_hw_pagesize(&self) -> Result<u64, ReadoutError> {
        if let Some(ctl) = &self.page_size_ctl {
            match ctl.value()? {
                sysctl::CtlValue::S64(x) => return Ok(x),
                _ => ()
            }
        }

        Err(ReadoutError::SysctlError(String::from("Could not read page size from system control")))
    }
}

impl ProductReadout for MacOSProductReadout {
    fn new() -> Self {
        MacOSProductReadout
    }
}

impl PackageReadout for MacOSPackageReadout {
    fn new() -> Self {
        MacOSPackageReadout
    }
}