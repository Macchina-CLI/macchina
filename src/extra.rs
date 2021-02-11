use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

use crate::memory;

/// Pop '__\n__' from the end of a string if it is found
pub fn pop_newline(mut string: String) -> String {
    if string.ends_with('\n') {
        string.pop();
    }
    String::from(string)
}

/// Return the content of the specified line from the specified path.
/// If something goes wrong, display the specified error message
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

/// Return `perc`% of 100%. This is used to determine
/// how many blocks to display in the memory bar
pub fn percent_of_total(perc: u64) -> u64 {
    let new_perc = (perc as f64 / 100.0) * memory::memtotal() as f64;
    new_perc as u64
}

/// Check if a _String_ is a valid integer
pub fn is_int(s: String) -> Result<(), String> {
    let b = s.chars().all(char::is_numeric);
    if b == true {
        return Ok(());
    }
    Err(String::from("This argument only accepts integers"))
}
