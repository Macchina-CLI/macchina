use std::path::Path;
use std::env;

/// Pop '\n' from the end of a string if it is found
pub fn pop_newline(mut string: String) -> String {
    if string.ends_with('\n') {
        string.pop();
    }
    String::from(string)
}

/// Return `perc`% of 100%. This is used to determine
/// how many used blocks to display in the memory bar
pub fn percent_of_total(perc: u64, total: u64) -> u64 {
    let new_perc = (perc as f64 / 100.0) * total as f64;
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
    return match exists {
        Some(_p) => true,
        None => false,
    }
}
