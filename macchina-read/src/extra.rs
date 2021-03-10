use std::path::Path;

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

/// Similar to how GNU's __which__ works.
/// Returns `true` if a program, such as __ps__,
/// exists on the system, and `false` if it doesn't.
pub fn which(program_name: &str) -> bool {
    if Path::new(&String::from("/bin/".to_owned() + &program_name)).exists() {
        true
    } else if Path::new(&String::from("/usr/bin/".to_owned() + &program_name)).exists() {
        true
    } else if Path::new(&String::from("/usr/sbin/".to_owned() + &program_name)).exists() {
        true
    } else if Path::new(&String::from("/usr/pkg/bin/".to_owned() + &program_name)).exists() {
        true
    } else if Path::new(&String::from("/usr/local/bin/".to_owned() + &program_name)).exists() {
        true
    } else if Path::new(&String::from("/usr/local/sbin/".to_owned() + &program_name)).exists() {
        true
    } else {
        false
    }
}
