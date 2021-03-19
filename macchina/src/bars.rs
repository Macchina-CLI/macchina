use macchina_read::traits::{BatteryReadout, MemoryReadout, ReadoutError};

/// Returns a usize [0..10] based on the battery percentage,
/// `display::show_bar` takes this function as a parameter to handle
/// displaying the bar
pub fn battery(percentage: u8) -> usize {
    match percentage {
        0..=10 => 1,
        11..=20 => 2,
        21..=30 => 3,
        31..=40 => 4,
        41..=50 => 5,
        51..=60 => 6,
        61..=70 => 7,
        71..=80 => 8,
        81..=90 => 9,
        91..=100 => 10,
        // 0 is reserved for errors
        _ => 0,
    }
}

/// Returns a usize [0..10] based on the memory usage,
/// `display::show_bar` takes this function as a parameter to handle
/// displaying the bar
pub fn memory(used: u64, total: u64) -> usize {
    let used = used as f64;
    let total = total as f64;

    (used / total * 10f64).ceil() as usize
}
