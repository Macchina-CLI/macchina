extern "C" {
    pub fn host_statistics(host_priv: mach::mach_types::host_t, flavor:
    mach::vm_types::integer_t, host_info_out: *mut mach::vm_types::integer_t, host_info_out_cnt:
                           *mut mach::message::mach_msg_type_number_t) -> mach::kern_return::kern_return_t;

    pub fn mach_host_self() -> mach::mach_types::host_name_port_t;
}