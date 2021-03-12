//! This module contains all the traits and types for creating a cross-platform api to query
//! different readouts from various operating systems. For each operating system, there must be an
//! implementation of these traits.

use ReadoutError::MetricNotAvailable;

/// This enum contains possible error types when doing sensor & variable readouts.
#[derive(Debug)]
pub enum ReadoutError {
    /// A specific metric might not be available on all systems (e. g. battery percentage on a
    /// desktop). If you encounter this error, it means that the requested value is not available.
    MetricNotAvailable,

    /// A readout for a metric might be available, but fails due to missing dependencies or other
    /// unsatisfied requirements.
    Other(String),
}

/// This trait provides the necessary functions for querying battery statistics from the host
/// computer. A desktop computer might not be able to provide values such as `percentage` and
/// `status`, which means a `ReadoutError` can be returned.
///
/// # Example
///
/// ```
/// use macchina_read::traits::BatteryReadout;
/// use macchina_read::traits::ReadoutError;
///
/// //You can add fields to this struct which will then need to be initialized in the
/// //BatteryReadout::new() function.
/// pub struct MacOSBatteryReadout;
///
/// impl BatteryReadout for MacOSBatteryReadout {
///     fn new() -> Self {
///         MacOSBatteryReadout {}
///     }
///
///     fn percentage(&self) -> Result<String, ReadoutError> {
///         //get the battery percentage somehow...
///         Ok(String::from("100")) //always fully charged
///     }
///
///     fn status(&self) -> Result<String, ReadoutError> {
///         //check if battery is being charged...
///         Ok(String::from("TRUE")) //always charging.
///     }
/// }
/// ```
pub trait BatteryReadout {
    /// Creates a new instance of the structure which implements this trait.
    fn new() -> Self;

    /// This function is used for querying the current battery percentage. The expected value is
    /// a string in the range of `0` to `100`.
    fn percentage(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function is used for querying the current battery charging state. If the battery is
    /// currently being charged, we expect a return value of `TRUE`, otherwise `FALSE`.
    fn status(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }
}

/// This trait is used for implementing common functions for reading kernel properties, such as
/// kernel name and version.
///
/// # Example
///
/// ```
/// use macchina_read::traits::KernelReadout;
/// use macchina_read::traits::ReadoutError;
///
/// pub struct MacOSKernelReadout;
///
/// impl KernelReadout for MacOSKernelReadout {
///     fn new() -> Self {
///         MacOSKernelReadout {}
///     }
///
///     fn os_release(&self) -> Result<String, ReadoutError> {
///         // Get kernel version
///         Ok(String::from("20.0.1"))
///     }
///
///     fn os_type(&self) -> Result<String, ReadoutError> {
///         // Get kernel name
///         Ok(String::from("Darwin"))
///     }
/// }
/// ```
pub trait KernelReadout {
    /// Creates a new instance of the structure which implements this trait.
    fn new() -> Self;

    /// This function should return the version of the kernel (e. g. `20.3.0` on macOS for Darwin).
    fn os_release(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the kernel name as a string (e. g. `Darwin` on macOS).
    fn os_type(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function is used for getting the kernel name and version in a pretty format.
    fn pretty_kernel(&self) -> Result<String, ReadoutError> {
        let os_type = self.os_type().unwrap_or_default();
        let os_release = self.os_release().unwrap_or_default();

        if !(os_type.is_empty() || os_release.is_empty()) {
            return Ok(format!("{} {}", os_type, os_release));
        }

        Err(ReadoutError::MetricNotAvailable)
    }
}

/// This trait provides common functions for querying the current memory state of the host
/// device, most notably `free` and `used`.
///
/// # Example
///
/// ```
/// use macchina_read::traits::MemoryReadout;
/// use macchina_read::traits::ReadoutError;
///
/// pub struct MacOSMemoryReadout;
///
/// impl MemoryReadout for MacOSMemoryReadout {
///     fn new() -> Self {
///         MacOSMemoryReadout {}
///     }
///
///     fn total(&self) -> Result<u64, ReadoutError> {
///         // Get the total physical memory for the machine
///         Ok(512 * 1024) // Return 512mb in kilobytes.
///     }
///
///     fn used(&self) -> Result<u64, ReadoutError> {
///         // Get the currently used memory.
///         Ok(256 * 1024) // Return 256mb in kilobytes.
///     }
/// }
///
/// ```
pub trait MemoryReadout {
    /// Creates a new instance of the structure which implements this trait.
    fn new() -> Self;

    /// This function should return the total available memory in kilobytes.
    fn total(&self) -> Result<u64, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the free available memory in kilobytes.
    fn free(&self) -> Result<u64, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the current memory value for buffers in kilobytes.
    fn buffers(&self) -> Result<u64, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the amount of cached content in memory in kilobytes.
    fn cached(&self) -> Result<u64, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the amount of reclaimable memory in kilobytes.
    fn reclaimable(&self) -> Result<u64, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the amount of currently used memory in kilobytes.
    fn used(&self) -> Result<u64, ReadoutError> {
        Err(MetricNotAvailable)
    }
}

/// This trait provides the interface for implementing functionality used for counting packages on
/// the host system. Almost all modern operating systems use some kind of package managers.
///
/// # Example
///
/// ```
/// use macchina_read::traits::PackageReadout;
/// use macchina_read::traits::ReadoutError;
///
/// pub struct MacOSPackageReadout;
///
/// impl PackageReadout for MacOSPackageReadout {
///     fn new() -> Self {
///         MacOSPackageReadout {}
///     }
///
///     fn count_pkgs(&self) -> Result<String, ReadoutError> {
///         // Check if homebrew 🍻 is installed and count installed pkgs...
///         Ok(String::from("100"))
///     }
/// }
/// ```
pub trait PackageReadout {
    /// Creates a new instance of the structure which implements this trait.
    fn new() -> Self;

    /// This function should return the amount of packages installed.
    fn count_pkgs(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }
}

/// This trait provides the interface for implementing functionality used for getting information
/// about the hosts operating system.
///
/// # Example
///
/// ```
/// use macchina_read::traits::ProductReadout;
/// use macchina_read::traits::ReadoutError;
///
/// pub struct MacOSProductReadout;
///
/// impl ProductReadout for MacOSProductReadout {
///     fn new() -> Self {
///         MacOSProductReadout {}
///     }
///
///     fn vendor(&self) -> Result<String, ReadoutError> {
///         Ok(String::from("Apple"))
///     }
///
///     fn family(&self) -> Result<String, ReadoutError> {
///         Ok(String::from("Unix, Macintosh"))
///     }
///
///     fn name(&self) -> Result<String, ReadoutError> {
///         // Get name of os release...
///         Ok(String::from("Big Sur"))
///     }
///
///     fn product(&self) -> Result<String, ReadoutError> {
///         Ok(String::from("macOS"))
///     }
/// }
/// ```
pub trait ProductReadout {
    /// Creates a new instance of the structure which implements this trait.
    fn new() -> Self;

    /// This function should return the version of the host's operating system.
    fn version(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the vendor name of the host's operating system.
    fn vendor(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the family name of the host's operating system.
    fn family(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the name of the host's operating system.
    fn name(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the product name of the hosts operating system.
    fn product(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }
}

/// This trait provides the interface for implementing functionality used for querying general
/// information about the running operating system and current user.
///
/// # Example
///
/// ```
/// use macchina_read::traits::GeneralReadout;
/// use macchina_read::traits::ReadoutError;
///
/// pub struct MacOSGeneralReadout;
///
/// impl GeneralReadout for MacOSGeneralReadout {
///
///     fn new() -> Self {
///         MacOSGeneralReadout {}
///     }
///
///     fn username(&self) -> Result<String, ReadoutError> {
///         //let username = NSUserName();
///         Ok(String::from("johndoe"))
///     }
///
///     // Implement other trait functions...
/// }
///
/// ```
pub trait GeneralReadout {
    /// Creates a new instance of the structure which implements this trait.
    fn new() -> Self;

    /// This function should return the username of the current logged on user.
    fn username(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the hostname of the hosts computer.
    fn hostname(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the name of the distribution of the operating system.
    fn distribution(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the name of the used desktop environment.
    fn desktop_environment(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the name of the used window manager.
    fn window_manager(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the name of the used terminal software.
    fn terminal(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the name of the current used shell (e. g. `bash` or `zsh`).
    ///
    /// *Params*:
    ///
    /// **_shorthand**: If the caller expects the full path to the used shell (e.g. `/bin/bash`) or
    /// just a shorthand of it (e.g. only the binary name).
    fn shell(&self, _shorthand: bool) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the full name of the cpu.
    fn cpu_model_name(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the uptime of the os in seconds.
    fn uptime(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the name of the physical machine (e.g. MacBookPro11,5 on a
    /// MacBook Pro).
    fn machine(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }

    /// This function should return the name of the OS in a pretty format (e.g. macOS 11.2.2 Big
    /// Sur)
    fn os_name(&self) -> Result<String, ReadoutError> {
        Err(MetricNotAvailable)
    }
}
