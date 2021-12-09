use std::path::{Path, PathBuf};

// Thanks to Andrey Tyukin
// https://stackoverflow.com/questions/54267608/expand-tilde-in-rust-path-idiomatically
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
