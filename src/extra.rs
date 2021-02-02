use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

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
