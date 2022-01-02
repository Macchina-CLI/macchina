use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

/// Simply returns `$HOME/.config`
pub fn config_dir() -> Option<PathBuf> {
    match env::var("HOME") {
        Ok(home) => Some(PathBuf::from(home).join(".config")),
        _ => None,
    }
}

/// Simply returns `/usr/share`
pub fn usr_share_dir() -> Option<PathBuf> {
    Some(PathBuf::from("/usr/share"))
}

/// Returns the entries of a given path.
pub fn get_entries(path: &Path) -> Option<Vec<PathBuf>> {
    match fs::read_dir(path) {
        Ok(dir) => {
            let mut entries: Vec<PathBuf> = Vec::new();
            dir.flatten().for_each(|x| entries.push(x.path()));
            Some(entries)
        }
        _ => None,
    }
}

/// Returns the extension of a given path.
pub fn path_extension(path: &Path) -> Option<&str> {
    path.extension().and_then(OsStr::to_str)
}
