use crate::macos::mach_ffi::{io_registry_entry_t, IOObjectRelease};
use crate::macos::mach_ffi::{
    kIOMasterPortDefault, vm_statistics64, IORegistryEntryCreateCFProperties,
    IOServiceGetMatchingService, IOServiceMatching,
};
use crate::traits::ReadoutError::MetricNotAvailable;
use crate::traits::*;
use core_foundation::base::{TCFType, ToVoid};
use core_foundation::dictionary::{
    CFMutableDictionary, CFMutableDictionaryRef,
};
use core_foundation::number::{CFNumber, CFNumberRef};
use core_foundation::string::{CFString};
use mach::kern_return::KERN_SUCCESS;
use std::ffi::CString;
use sysctl::{Ctl, Sysctl};

mod mach_ffi;

pub struct MacOSBatteryReadout {
    power_info: Result<MacOSIOPMPowerSource, ReadoutError>,
}

pub struct MacOSProductReadout {
    os_product_version_ctl: Option<Ctl>,
}

pub struct MacOSKernelReadout {
    os_type_ctl: Option<Ctl>,
    os_release_ctl: Option<Ctl>,
}

pub struct MacOSGeneralReadout {
    cpu_brand_ctl: Option<Ctl>,
    boot_time_ctl: Option<Ctl>,
    hostname_ctl: Option<Ctl>,
    hw_model_ctl: Option<Ctl>,
    local_ip: Option<String>,
}

pub struct MacOSMemoryReadout {
    page_size: u64,
    physical_memory: u64,
}

#[derive(Debug, Default)]
struct MacOSIOPMPowerSource {
    battery_installed: Option<bool>,
    state_of_charge: Option<usize>,
    charging: Option<bool>,
}

pub struct MacOSPackageReadout;

impl BatteryReadout for MacOSBatteryReadout {
    fn new() -> Self {
        MacOSBatteryReadout {
            power_info: MacOSIOPMPowerSource::new(),
        }
    }

    fn percentage(&self) -> Result<u8, ReadoutError> {
        let power_info = self.power_info.as_ref()?;

        Ok(power_info
            .state_of_charge
            .ok_or_else(|| ReadoutError::Other(String::from(
                "State of charge property was not present in the dictionary that was returned from IOKit.",
            )))? as u8)
    }

    fn status(&self) -> Result<BatteryState, ReadoutError> {
        let power_info = self.power_info.as_ref()?;

        if let Some(charging) = power_info.charging {
            return Ok(if charging {
                BatteryState::Charging
            } else {
                BatteryState::Discharging
            });
        }

        Err(ReadoutError::Other(String::from(
            "Charging state was not present in the dictionary that was returned from IOKit.",
        )))
    }
}

impl MacOSIOPMPowerSource {
    fn new() -> Result<Self, ReadoutError> {
        let power_source_dict = MacOSIOPMPowerSource::get_power_source_dict()?;
        let battery_data_dict = (*power_source_dict.get(&CFString::new("BatteryData").to_void()))
            as CFMutableDictionaryRef;

        let battery_data_dict: CFMutableDictionary<_> =
            unsafe { CFMutableDictionary::wrap_under_get_rule(battery_data_dict) };

        let mut instance: MacOSIOPMPowerSource = std::default::Default::default();

        unsafe {
            if let Some(battery_installed) =
                power_source_dict.find(&CFString::new("BatteryInstalled").to_void())
            {
                let number = CFNumber::wrap_under_get_rule((*battery_installed) as CFNumberRef);
                instance.battery_installed = Some(number.to_i32() != Some(0));
            }

            if let Some(state_of_charge) =
                battery_data_dict.find(&CFString::new("StateOfCharge").to_void())
            {
                let number = CFNumber::wrap_under_get_rule((*state_of_charge) as CFNumberRef);
                instance.state_of_charge = Some(number.to_i32().unwrap() as usize);
            }

            if let Some(charging) = power_source_dict.find(&CFString::new("IsCharging").to_void()) {
                let number = CFNumber::wrap_under_get_rule((*charging) as CFNumberRef);
                instance.charging = Some(number.to_i32() != Some(0));
            }
        }

        Ok(instance)
    }

    fn get_power_source_dict() -> Result<CFMutableDictionary, ReadoutError> {
        let io_service_name = CString::new("IOPMPowerSource").expect("Unable to create c string");
        let service = unsafe { IOServiceMatching(io_service_name.as_ptr()) };
        let entry: io_registry_entry_t =
            unsafe { IOServiceGetMatchingService(kIOMasterPortDefault, service) };
        let mut dict_data: Option<CFMutableDictionary> = None;

        if entry != 0 {
            let mut dict: CFMutableDictionaryRef = std::ptr::null_mut();
            let dict_ptr = (&mut dict) as *mut CFMutableDictionaryRef;

            let kern_return =
                unsafe { IORegistryEntryCreateCFProperties(entry, dict_ptr, std::ptr::null(), 0) };

            if kern_return == KERN_SUCCESS {
                dict_data = Some(unsafe { CFMutableDictionary::wrap_under_create_rule(dict) });
            }

            unsafe {
                IOObjectRelease(entry);
            }

            if kern_return != KERN_SUCCESS {
                return Err(ReadoutError::Other(format!(
                    "Creating the dictionary for the IOService failed with return code: {}",
                    kern_return
                )));
            }
        }

        dict_data.ok_or_else(|| ReadoutError::Other(String::from(
            "Unable to get the 'IOPMPowerSource' service from IOKit :( Are you on a desktop system?",
        )))
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
        Ok(self
            .os_release_ctl
            .as_ref()
            .ok_or(MetricNotAvailable)?
            .value_string()?)
    }

    fn os_type(&self) -> Result<String, ReadoutError> {
        Ok(self
            .os_type_ctl
            .as_ref()
            .ok_or(MetricNotAvailable)?
            .value_string()?)
    }

    fn pretty_kernel(&self) -> Result<String, ReadoutError> {
        Ok(format!("{} {}", self.os_type()?, self.os_release()?))
    }
}

impl GeneralReadout for MacOSGeneralReadout {
    fn new() -> Self {
        MacOSGeneralReadout {
            cpu_brand_ctl: Ctl::new("machdep.cpu.brand_string").ok(),
            boot_time_ctl: Ctl::new("kern.boottime").ok(),
            hostname_ctl: Ctl::new("kern.hostname").ok(),
            hw_model_ctl: Ctl::new("hw.model").ok(),
            local_ip: local_ipaddress::get(),
        }
    }

    fn username(&self) -> Result<String, ReadoutError> {
        crate::shared::whoami()
    }

    fn hostname(&self) -> Result<String, ReadoutError> {
        Ok(self
            .hostname_ctl
            .as_ref()
            .ok_or(MetricNotAvailable)?
            .value_string()?)
    }

    fn distribution(&self) -> Result<String, ReadoutError> {
        Err(ReadoutError::Warning(String::from(
            "Since you're on macOS, there is no distribution to be read from the system.",
        )))
    }

    fn local_ip(&self) -> Result<String, ReadoutError> {
        Ok(self
            .local_ip
            .as_ref()
            .ok_or(ReadoutError::MetricNotAvailable)?
            .to_string())
    }

    fn desktop_environment(&self) -> Result<String, ReadoutError> {
        Ok(String::from("Aqua"))
    }

    fn window_manager(&self) -> Result<String, ReadoutError> {
        Ok(String::from("Quartz Compositor"))
    }

    fn terminal(&self) -> Result<String, ReadoutError> {
        use std::env::var;

        let mut terminal: Option<String> = None;
        if let Ok(mut terminal_str) = var("TERM_PROGRAM") {
            terminal_str = terminal_str.to_lowercase();
            terminal = match terminal_str.as_str() {
                "iterm.app" => Some(String::from("iTerm2")),
                "apple_terminal" => Some(String::from("Apple Terminal")),
                "hyper" => Some(String::from("HyperTerm")),
                s => Some(String::from(s)),
            }
        }

        if let Some(terminal) = terminal {
            if let Ok(version) = var("TERM_PROGRAM_VERSION") {
                return Ok(format!("{} (Version {})", terminal, version));
            }

            return Ok(terminal)
        }

        if let Ok(terminal_env) = var("TERM") {
            return Ok(terminal_env);
        }

        Err(MetricNotAvailable)
    }

    fn shell(&self, shorthand: bool) -> Result<String, ReadoutError> {
        crate::shared::shell(shorthand)
    }

    fn cpu_model_name(&self) -> Result<String, ReadoutError> {
        Ok(self
            .cpu_brand_ctl
            .as_ref()
            .ok_or(MetricNotAvailable)?
            .value_string()?)
    }

    fn uptime(&self) -> Result<usize, ReadoutError> {
        use libc::timeval;
        use std::time::{Duration, SystemTime, UNIX_EPOCH};

        let time = self
            .boot_time_ctl
            .as_ref()
            .ok_or(MetricNotAvailable)?
            .value_as::<timeval>()?;
        let duration = Duration::new(time.tv_sec as u64, (time.tv_usec * 1000) as u32);
        let bootup_timestamp = UNIX_EPOCH + duration;

        if let Ok(duration) = SystemTime::now().duration_since(bootup_timestamp) {
            let seconds_since_boot = duration.as_secs();
            return Ok(seconds_since_boot as usize);
        }

        Err(ReadoutError::Other(String::from(
            "Error calculating boot time since unix \
            epoch.",
        )))
    }

    fn machine(&self) -> Result<String, ReadoutError> {
        let mac_model = self
            .hw_model_ctl
            .as_ref()
            .ok_or(MetricNotAvailable)?
            .value_string()?;

        Ok(mac_model)
    }

    fn os_name(&self) -> Result<String, ReadoutError> {
        let product_readout = MacOSProductReadout::new();

        let version = product_readout.version()?;
        let name = product_readout.product()?;
        let major_version_name =
            macos_version_to_name(&product_readout.operating_system_version()?);

        Ok(format!("{} {} {}", name, version, major_version_name))
    }
}

impl MemoryReadout for MacOSMemoryReadout {
    fn new() -> Self {
        let page_size = match Ctl::new("hw.pagesize").unwrap().value().unwrap() {
            sysctl::CtlValue::S64(s) => s,
            _ => panic!("Could not get vm page size."),
        };

        let physical_mem = match Ctl::new("hw.memsize").unwrap().value().unwrap() {
            sysctl::CtlValue::S64(s) => s,
            _ => panic!("Could not get physical memory size."),
        };

        MacOSMemoryReadout {
            page_size,
            physical_memory: physical_mem,
        }
    }

    fn total(&self) -> Result<u64, ReadoutError> {
        Ok(self.physical_memory / 1024)
    }

    fn free(&self) -> Result<u64, ReadoutError> {
        let vm_stats = MacOSMemoryReadout::mach_vm_stats()?;
        let free_count: u64 =
            (vm_stats.free_count + vm_stats.inactive_count - vm_stats.speculative_count) as u64;

        Ok(((free_count * self.page_size) / 1024) as u64)
    }

    fn reclaimable(&self) -> Result<u64, ReadoutError> {
        let vm_stats = MacOSMemoryReadout::mach_vm_stats()?;
        Ok((vm_stats.purgeable_count as u64 * self.page_size / 1024) as u64)
    }

    fn used(&self) -> Result<u64, ReadoutError> {
        let vm_stats = MacOSMemoryReadout::mach_vm_stats()?;
        let used: u64 =
            ((vm_stats.active_count + vm_stats.wire_count) as u64 * self.page_size / 1024) as u64;

        Ok(used)
    }
}

impl MacOSMemoryReadout {
    fn mach_vm_stats() -> Result<vm_statistics64, ReadoutError> {
        use mach::kern_return::KERN_SUCCESS;
        use mach::message::mach_msg_type_number_t;
        use mach::vm_types::integer_t;
        use mach_ffi::*;

        const HOST_VM_INFO_COUNT: mach_msg_type_number_t =
            (std::mem::size_of::<vm_statistics64>() / std::mem::size_of::<integer_t>()) as u32;

        const HOST_VM_INFO64: integer_t = 4;

        let mut vm_stat: vm_statistics64 = std::default::Default::default();
        let vm_stat_ptr: *mut vm_statistics64 = &mut vm_stat;
        let mut count: mach_msg_type_number_t = HOST_VM_INFO_COUNT;

        let ret_val = unsafe {
            host_statistics64(
                mach_host_self(),
                HOST_VM_INFO64,
                vm_stat_ptr as *mut integer_t,
                &mut count as *mut mach_msg_type_number_t,
            )
        };

        if ret_val == KERN_SUCCESS {
            return Ok(vm_stat);
        }

        Err(ReadoutError::Other(String::from(
            "Could not retrieve vm statistics from host.",
        )))
    }
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
struct NSOperatingSystemVersion {
    major_version: u64,
    minor_version: u64,
    patch_version: u64,
}

impl Into<String> for NSOperatingSystemVersion {
    fn into(self) -> String {
        format!(
            "{}.{}.{}",
            self.major_version, self.minor_version, self.patch_version
        )
    }
}

impl ProductReadout for MacOSProductReadout {
    fn new() -> Self {
        MacOSProductReadout {
            os_product_version_ctl: Ctl::new("kern.osproductversion").ok(),
        }
    }

    fn version(&self) -> Result<String, ReadoutError> {
        Ok(self.operating_system_version()?.into())
    }

    fn vendor(&self) -> Result<String, ReadoutError> {
        Ok(String::from("Apple"))
    }

    fn family(&self) -> Result<String, ReadoutError> {
        Ok(String::from("Unix, Macintosh"))
    }

    fn product(&self) -> Result<String, ReadoutError> {
        Ok(String::from("macOS"))
    }
}

impl MacOSProductReadout {
    fn operating_system_version(&self) -> Result<NSOperatingSystemVersion, ReadoutError> {
        let os_string = self
            .os_product_version_ctl
            .as_ref()
            .ok_or(ReadoutError::MetricNotAvailable)?
            .value_string()?;

        let mut string_parts = os_string.split('.');

        let mut operating_system_version = NSOperatingSystemVersion::default();

        if let Some(major) = string_parts.next() {
            operating_system_version.major_version = major.parse().unwrap_or_default()
        }

        if let Some(minor) = string_parts.next() {
            operating_system_version.minor_version = minor.parse().unwrap_or_default()
        }

        if let Some(patch) = string_parts.next() {
            operating_system_version.patch_version = patch.parse().unwrap_or_default()
        }

        Ok(operating_system_version)
    }
}

impl PackageReadout for MacOSPackageReadout {
    fn new() -> Self {
        MacOSPackageReadout
    }

    /// This methods check the `/usr/local/Cellar` and `/usr/local/Caskroom` folders which will
    /// contain all installed packages when using the Homebrew package manager. A manually call via
    /// `homebrew list` would be too expensive, since it is pretty slow.
    fn count_pkgs(&self) -> Vec<(PackageManager, usize)> {
        use std::fs::read_dir;
        use std::path::Path;

        let homebrew_root = Path::new("/usr/local");
        let cellar_folder = homebrew_root.join("Cellar");
        let caskroom_folder = homebrew_root.join("Caskroom");

        let cellar_count = match read_dir(cellar_folder) {
            Ok(read_dir) => read_dir.count(),
            Err(_) => 0,
        };

        let caskroom_count = match read_dir(caskroom_folder) {
            Ok(read_dir) => read_dir.count(),
            Err(_) => 0,
        };

        let total = cellar_count + caskroom_count;
        if total == 0 {
            return Vec::new();
        }

        vec![(PackageManager::Homebrew, total)]
    }
}

fn macos_version_to_name(version: &NSOperatingSystemVersion) -> &'static str {
    match (version.major_version, version.minor_version) {
        (10, 1) => "Puma",
        (10, 2) => "Jaguar",
        (10, 3) => "Panther",
        (10, 4) => "Tiger",
        (10, 5) => "Leopard",
        (10, 6) => "Snow Leopard",
        (10, 7) => "Lion",
        (10, 8) => "Mountain Lion",
        (10, 9) => "Mavericks",
        (10, 10) => "Yosemite",
        (10, 11) => "El Capitan",
        (10, 12) => "Sierra",
        (10, 13) => "High Sierra",
        (10, 14) => "Mojave",
        (10, 15) => "Catalina",
        (11, _) | (10, 16) => "Big Sur",
        _ => "Unknown",
    }
}
