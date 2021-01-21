use crate::read;

pub fn format_uptime(up: String) -> String {
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
    return formatted_uptime.trim().to_string();
}

pub fn format_battery() -> String {
    let percentage = read::read_battery_percentage();
    let status = read::read_battery_status();
    // Some computers stop charging before they reach 100%
    // so we will consider the battery to be full when
    // the battery percentage is within bat_full_range
    let bat_full_range: std::ops::RangeInclusive<i32> = 98..=100;
    if !bat_full_range.contains(&percentage.parse().unwrap()) {
        return String::from(percentage + "% - " + &status);
    }
    return String::from(&status);
}
