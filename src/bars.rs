use extra::percent_of_total;

use crate::extra;
use crate::memory;
use crate::read;

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

pub fn memory() -> usize {
    let u: u64 = memory::used();
    if u > 0 && u < percent_of_total(10) {
        return 1;
    } else if u > 11 && u <= percent_of_total(20) {
        return 2;
    } else if u > 21 && u <= percent_of_total(30) {
        return 2;
    } else if u > 31 && u <= percent_of_total(40) {
        return 2;
    } else if u > 41 && u <= percent_of_total(50) {
        return 2;
    } else if u > 51 && u <= percent_of_total(60) {
        return 2;
    } else if u > 61 && u <= percent_of_total(70) {
        return 2;
    } else if u > 71 && u <= percent_of_total(80) {
        return 2;
    } else if u > 81 && u <= percent_of_total(90) {
        return 2;
    } else if u > 91 && u <= memory::memtotal() {
        return 2;
    }
    0
}
