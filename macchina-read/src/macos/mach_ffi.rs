#![allow(non_camel_case_types, dead_code, unused)]

use mach::mach_types::{host_t, host_name_port_t};
use mach::vm_types::integer_t;
use mach::message::mach_msg_type_number_t;
use mach::kern_return;
use mach::kern_return::kern_return_t;
use mach::boolean;

use core_foundation::base::{CFTypeRef, CFRelease, TCFTypeRef, mach_port_t, CFAllocatorRef};
use core_foundation::dictionary::{CFDictionaryRef, CFMutableDictionaryRef};
use core_foundation::array::{CFArrayRef};
use libc::c_char;
use core_foundation::string::{CFStringRef};
use objc_foundation::NSString;

extern "C" {
    pub fn host_statistics(host_priv: host_t,
                           flavor: integer_t,
                           host_info_out: *mut integer_t,
                           host_info_out_cnt: *mut mach_msg_type_number_t) -> kern_return_t;

    pub fn mach_host_self() -> host_name_port_t;
}