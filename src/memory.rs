use crate::extra;

// Memory Utilization is extracted from /proc/meminfo
// Used memory is calculated using the following formula:
// used = memtotal - memfree - cached - sreclaimable - buffers

pub fn memtotal() -> u64 {
    let mem = String::from(
        extra::get_line_at("/proc/meminfo", 0, "Could not extract used memory!").unwrap(),
    );
    let s_mem_kb: String = mem.chars().filter(|c| c.is_digit(10)).collect();
    s_mem_kb.parse::<u64>().unwrap()
}

pub fn memfree() -> u64 {
    let mem = String::from(
        extra::get_line_at("/proc/meminfo", 1, "Could not extract used memory!").unwrap(),
    );
    let s_mem_kb: String = mem.chars().filter(|c| c.is_digit(10)).collect();
    s_mem_kb.parse::<u64>().unwrap()
}

pub fn buffers() -> u64 {
    let mem = String::from(
        extra::get_line_at("/proc/meminfo", 3, "Could not extract used memory!").unwrap(),
    );
    let s_mem_kb: String = mem.chars().filter(|c| c.is_digit(10)).collect();
    s_mem_kb.parse::<u64>().unwrap()
}

pub fn cached() -> u64 {
    let mem = String::from(
        extra::get_line_at("/proc/meminfo", 4, "Could not extract used memory!").unwrap(),
    );
    let s_mem_kb: String = mem.chars().filter(|c| c.is_digit(10)).collect();
    s_mem_kb.parse::<u64>().unwrap()
}

pub fn sreclaimable() -> u64 {
    let mem = String::from(
        extra::get_line_at("/proc/meminfo", 23, "Could not extract used memory!").unwrap(),
    );
    let s_mem_kb: String = mem.chars().filter(|c| c.is_digit(10)).collect();
    s_mem_kb.parse::<u64>().unwrap()
}

pub fn used() -> u64 {
    memtotal() - memfree() - cached() - sreclaimable() - buffers()
}