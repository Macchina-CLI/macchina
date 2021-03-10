use mach::mach_types::{host_t, host_name_port_t};
use mach::vm_types::integer_t;
use mach::message::mach_msg_type_number_t;
use mach::kern_return::kern_return_t;

use core_foundation::base::{CFTypeRef, CFRelease, TCFTypeRef};
use core_foundation::dictionary::{CFDictionaryRef};
use core_foundation::array::{CFArrayRef};
use core_foundation::string::{CFStringRef};

pub(crate) const kIOPSCurrentCapacityKey: &str = "Current Capacity";
pub(crate) const kIOPSIsChargingKey: &str = "Is Charging";

pub(crate) const kIOPMUPSPowerKey: &str = "UPS Power";
pub(crate) const kIOPMBatteryPowerKey: &str = "Battery Power";
pub(crate) const kIOPMACPowerKey: &str = "AC Power";

extern "C" {
    pub fn host_statistics(host_priv: host_t,
                           flavor: integer_t,
                           host_info_out: *mut integer_t,
                           host_info_out_cnt: *mut mach_msg_type_number_t) -> kern_return_t;

    pub fn mach_host_self() -> host_name_port_t;

    //iokit
    pub fn IOPSCopyPowerSourcesInfo() -> CFTypeRef;
    pub fn IOPSCopyPowerSourcesList(blob: CFTypeRef) -> CFArrayRef;
    pub fn IOPSGetPowerSourceDescription(blob: CFTypeRef, ps: CFTypeRef) -> CFDictionaryRef;

    pub fn IOPSGetProvidingPowerSourceType(snapshot: CFTypeRef) -> CFStringRef;

}