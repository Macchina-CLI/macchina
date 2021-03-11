use macchina_read::traits::{MemoryReadout, ReadoutError, BatteryReadout};

/// Returns a usize [0..10] based on the battery percentage,
/// `display::show_bar` takes this function as a parameter to handle
/// displaying the bar
pub fn battery() -> Result<usize, ReadoutError> {
    let res = match crate::READOUTS.battery.percentage()?
        .parse::<usize>().expect("Percentage could not be parsed.")
    {
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
    };

    Ok(res)
}

/// Returns a usize [0..10] based on the memory usage,
/// `display::show_bar` takes this function as a parameter to handle
/// displaying the bar
pub fn memory() -> Result<usize, ReadoutError> {
    let used = crate::READOUTS.memory.used()? as f64;
    let total = crate::READOUTS.memory.total()? as f64;

    Ok((used / total * 10f64).ceil() as usize)
}
