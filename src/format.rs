use bytesize::ByteSize;
use libmacchina::traits::{BatteryState, PackageManager, ReadoutError};

/// This function should return a new `String` constructed from the value \
/// returned by `traits::GeneralReadout::uptime()`
pub fn uptime(uptime: usize, long: bool) -> String {
    let mut fmt = String::new();
    let uptime: f32 = uptime as f32;
    // uptime is formatted to "x days, y hours, z minutes" if the system
    // has been up for longer than 60 seconds, and "x seconds" if not.

    // "x days", "y hours" or "z minutes" might not show up if their value is 0.
    // for example, if the system has been up for less than a day,
    // this function will return "y hours, z minutes".
    if uptime > 60.0 {
        let up_days = (uptime / 60.0 / 60.0 / 24.0).floor();
        let up_hours = (uptime / 60.0 / 60.0 % 24.0).floor();
        let up_minutes = (uptime / 60.0 % 60.0).floor();
        match long {
            false => {
                if up_days != 0.0 {
                    fmt.push_str(&up_days.to_string());
                    fmt.push_str("d ");
                }
                if up_hours != 0.0 {
                    fmt.push_str(&up_hours.to_string());
                    fmt.push_str("h ");
                }
                if up_minutes != 0.0 {
                    fmt.push_str(&up_minutes.to_string());
                    fmt.push('m');
                }
            }
            true => {
                if up_days != 0.0 {
                    fmt.push_str(&up_days.to_string());
                    if (up_days - 1.0).abs() < 0.001 {
                        fmt.push_str(" day ");
                    } else {
                        fmt.push_str(" days ");
                    }
                }
                if up_hours != 0.0 {
                    fmt.push_str(&up_hours.to_string());
                    if (up_hours - 1.0).abs() < 0.001 {
                        fmt.push_str(" hour ");
                    } else {
                        fmt.push_str(" hours ");
                    }
                }
                if up_minutes != 0.0 {
                    fmt.push_str(&up_minutes.to_string());
                    if (up_minutes - 1.0).abs() < 0.001 {
                        fmt.push_str(" minute");
                    } else {
                        fmt.push_str(" minutes");
                    }
                }
            }
        }
    }
    // uptime is formatted to seconds only if the
    // system has been up for fewer than 60 seconds
    else {
        let up_seconds = (uptime % 60.0).floor();
        if up_seconds != 0.0 {
            fmt = up_seconds.to_string();
            fmt.push('s');
        }
    }

    fmt.trim().to_string()
}

/// This function should return a new `String` constructed from the values \
/// returned by `traits::GeneralReadout::username()` and `traits::GeneralReadout::hostname()`
pub fn host(username: &str, hostname: &str) -> String {
    format!("{}@{}", username, hostname)
}

/// This function should return a new `String` constructed from the values \
/// returned by `traits::BatteryReadout::percentage()` and `traits::BatteryReadout::status()`
pub fn battery(percentage: u8, state: BatteryState) -> String {
    // Holds either "Charging" or "Discharging" values
    if percentage != 100 {
        format!("{}% & {}", percentage, Into::<&'static str>::into(state))
    } else {
        String::from("Full")
    }
}

/// This function should return a new `String` constructed from the values \
/// returned by `traits::MemoryReadout::used()` and `traits::MemoryReadout::total()`
pub fn memory(total: u64, used: u64) -> String {
    let total = ByteSize::kb(total);
    let used = ByteSize::kb(used);

    format!("{}/{}", used, total)
}

/// This function should return a new `String` constructed from the value \
/// returned by `traits::GeneralReadout::cpu_model_name()`
pub fn cpu_only(model_name: &str) -> String {
    model_name.replace("(TM)", "™").replace("(R)", "®")
}

pub fn cpu(model_name: &str, cpu_cores: usize) -> String {
    format!("{} ({})", cpu_only(model_name), cpu_cores)
}

pub fn cpu_usage(used: usize) -> String {
    format!("{}%", used)
}

pub fn packages(packages: Vec<(PackageManager, usize)>) -> Result<String, ReadoutError> {
    let len = packages.len();
    if len == 0 {
        return Err(ReadoutError::Other(String::from(
            "No packages found — Do you have a package manager installed?",
        )));
    }

    // pre-allocate an estimated size to reduce the number
    // of reallocations when manipulating the string
    let mut string = String::with_capacity(len * 7);

    for (i, (pm, count)) in packages.iter().enumerate() {
        let add_comma = if i + 1 < len { ", " } else { "" };
        string.push_str(&format!("{} ({}){}", count, pm.to_string(), add_comma));
    }

    Ok(string)
}
