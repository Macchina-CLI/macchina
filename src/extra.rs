use crate::memory;
use std::env;
use std::path::Path;

/// Pop '\n' from the end of a string if it is found
pub fn pop_newline(mut string: String) -> String {
    if string.ends_with('\n') {
        string.pop();
    }
    String::from(string)
}
pub fn pop_percent(mut string: String) -> String {
    if string.ends_with('%') {
        string.pop();
    }
    String::from(string)
}
pub fn pop_whitespace(mut string: String) -> String {
    while string.ends_with(' ') {
        string.pop();
    }
    while string.starts_with(' ') {
        string.remove(0);
    }
    String::from(string)
}

/// Return `perc`% of 100%. This is used to determine
/// how many used blocks to display in the memory bar
pub fn percent_of_total(perc: u64) -> u64 {
    let new_perc = (perc as f64 / 100.0) * memory::memtotal() as f64;
    new_perc as u64
}

/// Check if a `String` is a valid integer
pub fn is_int(s: String) -> Result<(), String> {
    let b = s.chars().all(char::is_numeric);
    if b == true {
        return Ok(());
    }
    Err(String::from("this argument only accepts integers."))
}

/// Uppercase first letter of a string of characters
pub fn ucfirst<S: AsRef<str>>(s: S) -> String {
    let mut c = s.as_ref().chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Similar to how GNU's __which__ works.
/// Returns `true` if a program, such as __ps__,
/// exists on the system, and `false` if it doesn't.
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
    match exists {
        Some(_p) => return true,
        None => return false,
    }
}
