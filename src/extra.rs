use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

pub fn get_line_at(path: &str, line_num: usize, msg: &str) -> Result<String, Error> {
    let path = Path::new(path);
    let file = File::open(path).expect(&msg);
    let content = BufReader::new(&file);
    let mut lines = content.lines();
    lines.nth(line_num).expect("Line out-of-bounds")
}