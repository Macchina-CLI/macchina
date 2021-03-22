#![allow(non_camel_case_types, dead_code, unused)]

use mach::boolean;
use mach::kern_return;
use mach::kern_return::kern_return_t;
use mach::mach_types::{host_name_port_t, host_t};
use mach::message::mach_msg_type_number_t;
use mach::vm_types::{integer_t, natural_t};

use core_foundation::array::CFArrayRef;
use core_foundation::base::{mach_port_t, CFAllocatorRef, CFRelease, CFTypeRef, TCFTypeRef};
use core_foundation::dictionary::{CFDictionaryRef, CFMutableDictionaryRef};
use core_foundation::string::CFStringRef;
use libc::c_char;
use std::os::raw::c_uint;

type host_flavor_t = integer_t;
type host_info64_t = *mut integer_t;
pub type io_object_t = mach_port_t;
pub type io_service_t = io_object_t;
pub type IOOptionBits = c_uint;
pub type io_registry_entry_t = io_object_t;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct vm_statistics64 {
    pub free_count: natural_t,
    pub active_count: natural_t,
    pub inactive_count: natural_t,
    pub wire_count: natural_t,
    pub zero_fill_count: u64,
    pub reactivations: u64,
    pub pageins: u64,
    pub pageouts: u64,
    pub faults: u64,
    pub cow_faults: u64,
    pub lookups: u64,
    pub hits: u64,
    pub purges: u64,
    pub purgeable_count: natural_t,
    pub speculative_count: natural_t,
    pub decompressions: u64,
    pub compressions: u64,
    pub swapins: u64,
    pub swapouts: u64,
    pub compressor_page_count: natural_t,
    pub throttled_count: natural_t,
    pub external_page_count: natural_t,
    pub internal_page_count: natural_t,
    pub total_uncompressed_pages_in_compressor: u64,
}

extern "C" {
    pub fn host_statistics64(
        host_priv: host_t,
        flavor: host_flavor_t,
        host_info64_out: host_info64_t,
        host_info64_out_cnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;

    pub fn mach_host_self() -> host_name_port_t;

    #[link_name = "kIOMasterPortDefault"]
    pub static kIOMasterPortDefault: mach_port_t;

    pub fn IOServiceMatching(name: *const c_char) -> CFMutableDictionaryRef;

    pub fn IOServiceGetMatchingService(
        master_port: mach_port_t,
        matching: CFDictionaryRef,
    ) -> io_service_t;

    pub fn IORegistryEntryCreateCFProperties(
        entry: io_registry_entry_t,
        properties: *mut CFMutableDictionaryRef,
        allocator: CFAllocatorRef,
        options: IOOptionBits,
    ) -> kern_return_t;

    pub fn IOObjectRelease(object: io_object_t) -> kern_return_t;
}
