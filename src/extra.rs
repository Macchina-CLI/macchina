use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

use crate::memory;

pub fn pop_newline(mut string: String) -> String {
    if string.ends_with('\n') {
        string.pop();
    }
    String::from(string)
}

pub fn get_line_at(
    path_to_file: &str,
    line_number: usize,
    error_message: &str,
) -> Result<String, Error> {
    let _path = Path::new(path_to_file);
    let file = File::open(path_to_file).expect(&error_message);
    let content = BufReader::new(&file);
    let mut lines = content.lines();
    lines.nth(line_number).expect("Line is out-of-bounds")
}

pub fn percent_of_total(perc: u64) -> u64 {
    let new_perc = (perc as f64 / 100.0) * memory::memtotal() as f64;
    new_perc as u64
}

pub fn is_int(s: String) -> Result<(), String> {
    let b = s.chars().all(char::is_numeric);
    if b == true {
        return Ok(());
    }
    Err(String::from("This argument only accepts integers"))
}