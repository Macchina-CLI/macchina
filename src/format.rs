use crate::{battery, extra, general, kernel, memory, product, Fail};
use bytesize::ByteSize;

/// Construct a new _String_ from the value
/// returned by `read::uptime`
pub fn uptime(up: String, shorthand: bool) -> String {
    let mut formatted_uptime = String::new();
    let uptime: f32 = up.parse().unwrap();
    // Uptime is formatted to dd:hh:mm if the system has been up for longer than 60 seconds
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
                    formatted_uptime.push_str("m");
                }
            }
            false => {
                if up_days != 0.0 {
                    if up_days == 1.0 {
                        formatted_uptime.push_str(&up_days.to_string());
                        formatted_uptime.push_str(" day ");
                    } else {
                        formatted_uptime.push_str(&up_days.to_string());
                        formatted_uptime.push_str(" days ");
                    }
                }
                if up_hours != 0.0 {
                    if up_hours == 1.0 {
                        formatted_uptime.push_str(&up_hours.to_string());
                        formatted_uptime.push_str(" hour ");
                    } else {
                        formatted_uptime.push_str(&up_hours.to_string());
                        formatted_uptime.push_str(" hours ");
                    }
                }
                if up_minutes != 0.0 {
                    if up_minutes == 1.0 {
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
            formatted_uptime.push_str("s");
        }
    }
    formatted_uptime.trim().to_string()
}

/// Construct a new _String_ from the values
/// returned by `read::hostname` and `read::username`
pub fn host(fail: &mut Fail) -> String {
    let username = general::username();
    let hostname = general::hostname();
    if !username.is_empty() && hostname != "Unknown" {
        return username + "@" + &hostname;
    } else {
        fail.host.failed = true;
        return String::from("Unknown");
    }
}

/// Construct a new _String_ from the values
/// returned by `read::battery_percentage` and `read::battery_status`
pub fn battery(fail: &mut Fail) -> String {
    let percentage = battery::percentage(fail);
    let status = battery::status(fail);
    if !percentage.is_empty() && !status.is_empty() {
        if percentage != "100" {
            return String::from(percentage + "% & " + &status);
        }
        return String::from(&status);
    }
    String::from("Unknown")
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
    String::from(general::cpu_model_name() + " (" + &num_cpus::get().to_string() + ")")
        .replace("(TM)", "™")
        .replace("(R)", "®")
}

/// Construct a new _String_ from the values
/// returned by `product::sys_vendor` and `product::product_family` or `product::product_version`
#[cfg(target_os = "linux")]
pub fn machine() -> String {
    if product::product_family() == product::product_name()
        && product::product_family() == product::product_version()
    {
        return product::product_family();
    } else if product::product_version().is_empty() || product::product_version().len() <= 15 {
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

#[cfg(target_os = "netbsd")]
/// Construct a new _String_ from the values
/// returned by `product::sys_vendor` and `product::product_family` or `product::product_version`
pub fn machine() -> String {
    if product::system_version() == product::system_product()
        && product::system_version() == product::system_vendor()
    {
        return product::system_vendor();
    }
    product::system_vendor() + " " + &product::system_product() + " " + &product::system_version()
}

pub fn desktop_environment(mut session_name: String) -> String {
    let last_occurence_index = session_name.rfind("/").unwrap() + 1;
    session_name.replace_range(0..last_occurence_index, "");
    return extra::ucfirst(&session_name);
}

pub fn kernel() -> String {
    let ostype = kernel::ostype();
    let osrelease = kernel::osrelease();
    if !(ostype.is_empty() || osrelease.is_empty()) {
        return ostype + " " + &osrelease;
    }
    String::from("Unknown")
}
