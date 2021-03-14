use crate::traits::*;
use crate::windows::bindings::windows::win32::system_services::PSTR;

mod bindings {
    ::windows::include_bindings!();
}

use bindings::{
    windows::win32::windows_programming::GetComputerNameExA,
    windows::win32::windows_programming::GetUserNameA,
};

pub struct WindowsBatteryReadout;

impl BatteryReadout for WindowsBatteryReadout {
    fn new() -> Self {
        WindowsBatteryReadout {}
    }
}

pub struct WindowsKernelReadout;

impl KernelReadout for WindowsKernelReadout {
    fn new() -> Self {
        WindowsKernelReadout {}
    }
}

pub struct WindowsMemoryReadout;

impl MemoryReadout for WindowsMemoryReadout {
    fn new() -> Self {
        WindowsMemoryReadout {}
    }
}

pub struct WindowsGeneralReadout;

impl GeneralReadout for WindowsGeneralReadout {
    fn new() -> Self {
        WindowsGeneralReadout {}
    }

    fn username(&self) -> Result<String, ReadoutError> {
        let mut size = 0;
        unsafe { GetUserNameA(PSTR(std::ptr::null_mut()), &mut size) };

        if size == 0 {
            return Err(ReadoutError::Other(String::from(
                "Call to \"GetUserNameA\" failed.",
            )));
        }

        let mut username = Vec::with_capacity(size as usize);
        if !unsafe { GetUserNameA(PSTR(username.as_mut_ptr()), &mut size) }.as_bool() {
            return Err(ReadoutError::Other(String::from(
                "Call to \"GetUserNameA\" failed.",
            )));
        }

        unsafe {
            username.set_len(size as usize);
        }

        let mut str = match String::from_utf8(username) {
            Ok(str) => str,
            Err(e) => {
                return Err(ReadoutError::Other(format!(
                    "String from \"GetUserNameA\" \
            was not valid UTF-8: {}",
                    e
                )))
            }
        };

        str.pop(); //remove null terminator from string.

        Ok(str)
    }

    fn hostname(&self) -> Result<String, ReadoutError> {
        use bindings::windows::win32::windows_programming::COMPUTER_NAME_FORMAT;

        let mut size = 0;
        unsafe {
            GetComputerNameExA(
                COMPUTER_NAME_FORMAT::ComputerNameDnsHostname,
                PSTR(std::ptr::null_mut()),
                &mut size,
            )
        };

        if size == 0 {
            return Err(ReadoutError::Other(String::from(
                "Call to \"GetComputerNameExA\" failed.",
            )));
        }

        let mut hostname = Vec::with_capacity(size as usize);
        if unsafe {
            GetComputerNameExA(
                COMPUTER_NAME_FORMAT::ComputerNameDnsHostname,
                PSTR(hostname.as_mut_ptr()),
                &mut size,
            )
        } == false
        {
            return Err(ReadoutError::Other(String::from(
                "Call to \"GetComputerNameExA\" failed.",
            )));
        }

        unsafe { hostname.set_len(size as usize) };

        let str = match String::from_utf8(hostname) {
            Ok(str) => str,
            Err(e) => {
                return Err(ReadoutError::Other(format!(
                    "String from \"GetComputerNameExA\" \
            was not valid UTF-8: {}",
                    e
                )))
            }
        };

        Ok(str)
    }
}

pub struct WindowsProductReadout;

impl ProductReadout for WindowsProductReadout {
    fn new() -> Self {
        WindowsProductReadout {}
    }
}

pub struct WindowsPackageReadout;

impl PackageReadout for WindowsPackageReadout {
    fn new() -> Self {
        WindowsPackageReadout {}
    }
}
