extern crate bytesize;
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
/// returned by `read::battery_percentage` and `read::battery_status`
pub fn battery(percentage: String, status: String) -> String {
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
pub fn memory(used: u64, total: u64) -> String {
    let total = ByteSize::kb(total);
    let used = ByteSize::kb(used);

    String::from(used.to_string() + "/" + &total.to_string())
}

/// Construct a new _String_ from the values
/// returned by `read::cpu_model_name` and `num_cpus::get`
pub fn cpu(cpu_model_name: String, logical_cores: usize) -> String {
    String::from(cpu_model_name + " (" + &logical_cores.to_string() + ")")
}

/// Construct a new _String_ from the values
/// returned by `machine::sys_vendor` and `machine::product_family` or `machine::product_version`
pub fn machine(
    product_version: String,
    sys_vendor: String,
    product_family: String,
    product_name: String,
) -> String {
    if product_version.is_empty() || product_version.len() <= 10 {
        return String::from(sys_vendor + " " + &product_family + " " + &product_name);
    }
    product_version
}

pub fn desktop_session(mut session_name: String) -> String {
    if !session_name.is_empty() {
        let last_occurence_index = session_name.rfind("/").unwrap() + 1;
        session_name.replace_range(0..last_occurence_index, "");

        // Uppercase first letter
        let first_letter = session_name
            .chars()
            .next()
            .unwrap()
            .to_uppercase()
            .to_string();
        // Remove first letter from original string
        session_name.remove(0);
        // Append to new string the uppercase
        // letter and rest of original string
        let new_string = first_letter + &session_name;

        return new_string;
    }
    String::from("Unknown")
}
