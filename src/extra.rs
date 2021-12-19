use std::path::{Path, PathBuf};
use std::ffi::OsStr;

/// Expands `~` to its appropriate value.
///
/// Thanks to Andrey Tyukin
/// https://stackoverflow.com/questions/54267608/expand-tilde-in-rust-path-idiomatically
pub fn expand_home<P: AsRef<Path>>(initial_path: P) -> Option<PathBuf> {
    let p = initial_path.as_ref();

    if !p.starts_with("~") {
        return Some(p.to_path_buf());
    }

    if p.eq(Path::new("~")) {
        return dirs::home_dir();
    }

    dirs::home_dir().map(|mut h| {
        if h == Path::new("/") {
            p.strip_prefix("~").unwrap().to_path_buf()
        } else {
            h.push(p.strip_prefix("~/").unwrap());
            h
        }
    })
}

/// Simply returns `$HOME/.config`
pub fn config_dir() -> Option<PathBuf> {
    if let Ok(home) = std::env::var("HOME") {
        Some(PathBuf::from(home).join(".config"))
    } else {
        None
    }
}

/// Simply returns `/usr/share`
pub fn usr_share_dir() -> Option<PathBuf> {
    Some(PathBuf::from("/usr/share"))
}

/// Returns the entries of a given path.
pub fn list_entries(path: &Path) -> Vec<PathBuf> {
    let mut directory_entries: Vec<PathBuf> = Vec::new();
    let directory = std::fs::read_dir(path);

    if let Ok(dir) = directory {
        for entry in dir.flatten() {
            directory_entries.push(entry.path())
        }
    }

    directory_entries
}

/// Returns the extension of a give path.
pub fn path_extension(path: &Path) -> Option<&str> {
    path.extension().and_then(OsStr::to_str)
}
