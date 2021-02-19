use crate::extra;
use crate::memory;
use crate::read;
use extra::percent_of_total;

/// Returns a usize (0 .. 10) based on the battery percentage,
/// `display::show_bar` takes this usize as a parameter to handle
/// displaying the bar
pub fn battery() -> usize {
    match read::battery_percentage().parse::<u32>().unwrap() {
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
        _ => 0,
    }
}

/// Returns a usize (0 .. 10) based on the memory usage,
/// `display::show_bar` takes this usize as a parameter to handle
/// displaying the bar
pub fn memory() -> usize {
    let u = memory::used();

    if u <= percent_of_total(10) {
        return 1;
    } else if u <= percent_of_total(20) {
        return 2;
    } else if u <= percent_of_total(30) {
        return 3;
    } else if u <= percent_of_total(40) {
        return 4;
    } else if u <= percent_of_total(50) {
        return 5;
    } else if u <= percent_of_total(60) {
        return 6;
    } else if u <= percent_of_total(70) {
        return 7;
    } else if u <= percent_of_total(80) {
        return 8;
    } else if u <= percent_of_total(90) {
        return 9;
    } else if u <= percent_of_total(100) {
        return 10;
    }

    0
}
