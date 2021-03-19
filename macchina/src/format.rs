use bytesize::ByteSize;
use macchina_read::traits::{PackageManager, ReadoutError, BatteryState};

//TODO: rework documentation

/// This function should return a new `String` constructed from the value \
/// returned by `READOUTS.general.uptime()`
pub fn uptime(uptime: usize, shorthand: bool) -> String {
    let mut formatted_uptime = String::new();
    let uptime: f32 = uptime as f32;
    // Uptime is formatted to "x days, y hours, z minutes" if the system
    // has been up for longer than 60 seconds, and "x seconds" if not.

    // "x days", "y hours" or "z minutes" might not show up if their value is 0.
    // for example, if the system has been up for less than a day,
    // this function will return "y hours, z minutes".
    if uptime > 60.0 {
        let up_days = (uptime / 60.0 / 60.0 / 24.0).floor();
        let up_hours = (uptime / 60.0 / 60.0 % 24.0).floor();
        let up_minutes = (uptime / 60.0 % 60.0).floor();
        match shorthand {
            true => {
                if up_days != 0.0 {
                    formatted_uptime.push_str(&up_days.to_string());
                    formatted_uptime.push_str("d ");
                }
                if up_hours != 0.0 {
                    formatted_uptime.push_str(&up_hours.to_string());
                    formatted_uptime.push_str("h ");
                }
                if up_minutes != 0.0 {
                    formatted_uptime.push_str(&up_minutes.to_string());
                    formatted_uptime.push('m');
                }
            }
            false => {
                if up_days != 0.0 {
                    if (up_days - 1.0).abs() < 0.001 {
                        formatted_uptime.push_str(&up_days.to_string());
                        formatted_uptime.push_str(" day ");
                    } else {
                        formatted_uptime.push_str(&up_days.to_string());
                        formatted_uptime.push_str(" days ");
                    }
                }
                if up_hours != 0.0 {
                    if (up_hours - 1.0).abs() < 0.001 {
                        formatted_uptime.push_str(&up_hours.to_string());
                        formatted_uptime.push_str(" hour ");
                    } else {
                        formatted_uptime.push_str(&up_hours.to_string());
                        formatted_uptime.push_str(" hours ");
                    }
                }
                if up_minutes != 0.0 {
                    if (up_minutes - 1.0).abs() < 0.001 {
                        formatted_uptime.push_str(&up_minutes.to_string());
                        formatted_uptime.push_str(" minute");
                    } else {
                        formatted_uptime.push_str(&up_minutes.to_string());
                        formatted_uptime.push_str(" minutes");
                    }
                }
            }
        }
    }
    // Uptime is formatted to seconds only if the system has been up for fewer than 60 seconds
    else {
        let up_seconds = (uptime % 60.0).floor();
        if up_seconds != 0.0 {
            formatted_uptime = up_seconds.to_string();
            formatted_uptime.push('s');
        }
    }

    formatted_uptime.trim().to_string()
}

/// This function should return a new `String` constructed from the values \
/// returned by `READOUTS.general.username()` and `READOUTS.general.hostname()`
pub fn host(username: &String, hostname: &String) -> String {
    format!("{}@{}", username, hostname)
}

/// This function should return a new `String` constructed from the values \
/// returned by `READOUTS.battery.percentage()` and `READOUTS.battery.status()`
pub fn battery(percentage: u8, state: BatteryState) -> String {
    // Holds either "Charging" or "Discharging" values
    if percentage != 100 {
        format!("{}% & {}", percentage, Into::<&'static str>::into(state))
    } else {
        String::from("Full")
    }
}

/// This function should return a new `String` constructed from the values \
/// returned by `READOUTS.memory.total()` and `READOUTS.memory.used()`
pub fn memory(total: u64, used: u64) -> String {
    let total = ByteSize::kb(total);
    let used = ByteSize::kb(used);

    format!("{}/{}", used, total)
}

/// This function should return a new `String` constructed from the values \
/// returned by `READOUTS.general.cpu_model_name()` and `num_cpus::get()`
pub fn cpu(model_name: &String) -> String {
    format!("{} ({})", model_name, num_cpus::get())
        .replace("(TM)", "™")
        .replace("(R)", "®")
}

pub fn packages(
    packages: Vec<(PackageManager, usize)>,
) -> Result<String, ReadoutError> {
    let len = packages.len();
    if len == 0 {
        return Err(ReadoutError::Other(String::from("No packages found.")));
    }

    // pre-allocate some estimated size
    let mut string = String::with_capacity(len * 7);

    for (i, (pm, count)) in packages.iter().enumerate() {
        let add_comma = if i + 1 < len { ", " } else { "" };

        string.push_str(&format!("{} ({}){}", count, pm.to_string(), add_comma));
    }

    Ok(string)
}
