use crate::{memory, product, read};
use bytesize::ByteSize;

/// Construct a new _String_ from the value
/// returned by `read::uptime`
pub fn uptime(up: String) -> String {
    let mut formatted_uptime = String::new();
    let uptime: f32 = up.parse().unwrap();
    // Uptime is formatted to dd:hh:mm if the system has been up for longer than 60 seconds
    if uptime > 60.0 {
        let up_days = (uptime / 60.0 / 60.0 / 24.0).floor();
        let up_hours = (uptime / 60.0 / 60.0 % 24.0).floor();
        let up_minutes = (uptime / 60.0 % 60.0).floor();
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
            formatted_uptime.push_str("m");
        }
    }
    // Uptime is formatted to seconds only if the system has been up for fewer than 60 seconds
    else {
        let up_seconds = (uptime % 60.0).floor();
        if up_seconds != 0.0 {
            formatted_uptime = up_seconds.to_string();
            formatted_uptime.push_str("s");
        }
    }
    formatted_uptime.trim().to_string()
}

/// Construct a new _String_ from the values
/// returned by `read::hostname` and `read::username`
pub fn host() -> String {
    read::username() + "@" + &read::hostname()
}

/// Construct a new _String_ from the values
/// returned by `read::battery_percentage` and `read::battery_status`
pub fn battery() -> String {
    let percentage = read::battery_percentage();
    let status = read::battery_status();
    if !percentage.is_empty() && !status.is_empty() {
        if percentage != "100" {
            return String::from(percentage + "% - " + &status);
        }
        return String::from(&status);
    }
    String::from("Could not extract battery info")
}

/// Construct a new _String_ from the values
/// returned by `memory::used` and `memory::memtotal`
pub fn memory() -> String {
    let total = ByteSize::kb(memory::memtotal());
    let used = ByteSize::kb(memory::used());
    String::from(used.to_string() + "/" + &total.to_string())
}

/// Construct a new _String_ from the values
/// returned by `read::cpu_model_name` and `num_cpus::get`
pub fn cpu() -> String {
    String::from(read::cpu_model_name() + " (" + &num_cpus::get().to_string() + ")")
        .replace("(TM)", "™")
        .replace("(R)", "®")
}

/// Construct a new _String_ from the values
/// returned by `machine::sys_vendor` and `machine::product_family` or `machine::product_version`
pub fn machine() -> String {
    if product::product_version().is_empty() || product::product_version().len() <= 15 {
        return String::from(
            product::sys_vendor()
                + " "
                + &product::product_family()
                + " "
                + &product::product_name(),
        );
    }
    product::product_version()
}

pub fn desktop_environment(mut session_name: String) -> String {
    let last_occurence_index = session_name.rfind("/").unwrap() + 1;
    session_name.replace_range(0..last_occurence_index, "");
    return session_name;
}
