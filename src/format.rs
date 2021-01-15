use crate::read;

pub fn format_uptime(uptime: f32) -> String {
    let mut _uptime = String::new();
    // Uptime is formatted to dd:hh:mm if the system has been up for longer than 60 seconds
    if uptime > 60.0 {
        let up_days = (uptime / 60.0 / 60.0 / 24.0).floor();
        if up_days != 0.0 {
            _uptime = _uptime + &up_days.to_string() + "d ";
        }
        let up_hours = (uptime / 60.0 / 60.0 % 24.0).floor();
        if up_hours != 0.0 {
            _uptime = _uptime + &up_hours.to_string() + "h ";
        }
        let up_minutes = (uptime / 60.0 % 60.0).floor();
        if up_minutes != 0.0 {
            _uptime = _uptime + &up_minutes.to_string() + "m";
        }
    }
    // Uptime is formatted to ss if the system has been up for fewer than 60 seconds
    else {
        let up_seconds = (uptime % 60.0).floor();
        if up_seconds != 0.0 {
            _uptime = up_seconds.to_string() + "s";
        }
    }
    return _uptime.trim().to_string();
}

pub fn format_battery() -> String {
    let percentage = read::read_battery_percentage();
    let status = read::read_battery_status();
    // Some computers stop charging before they reach 100%
    // so we will consider the battery to be full when
    // the battery percentage is within bat_full_range
    // This range is inclusive
    let bat_full_range: std::ops::RangeInclusive<i32> = 98..=100;
    if !bat_full_range.contains(&percentage.parse().unwrap()) {
        return String::from(percentage + "% - " + &status);
    }
    return String::from(&status);
}
