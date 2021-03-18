//! This module provides additional functionalities

use std::env;
use std::path::Path;

/// Pop `\n` from the end of a string if it is found.
pub fn pop_newline(mut string: String) -> String {
    if string.ends_with('\n') {
        string.pop();
    }

    string
}

/// Return `perc`% of 100%. \
/// This is used to determine how many used
/// glyphs to display in the memory bar.
pub fn percent_of_total(perc: u64, total: u64) -> u64 {
    let new_perc = (perc as f64 / 100.0) * total as f64;
    new_perc as u64
}

/// Check if the `String` that is passed
/// to this function is a valid integer.
pub fn is_int(s: String) -> Result<(), String> {
    if s.chars().all(char::is_numeric) {
        return Ok(());
    }

    Err(String::from("this argument only accepts integers."))
}

/// Uppercase the first letter of a `String` or `&str`.
pub fn ucfirst<S: AsRef<str>>(s: S) -> String {
    let mut c = s.as_ref().chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Search all directories in `PATH` for a program e.g. __ps__, __grep__, etc.
///
/// This can be used to check if a particular program exists before running a command \
/// that could return an error in case the program is not installed.
///
/// # Example
/// ```
/// if macchina_read::extra::which("program") {
///     // Run the program.
/// }
/// ```
///
/// - Returns `TRUE` if _program_ is in `PATH`, or false if it isn't.
pub fn which<P>(program_name: P) -> bool
where
    P: AsRef<Path>,
{
    let exists = env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(&program_name);
                if full_path.exists() {
                    Some(full_path)
                } else {
                    None
                }
            })
            .next()
    });

    exists.is_some()
}
