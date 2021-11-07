use std::path::{Path, PathBuf};

pub fn config_data_paths() -> [Option<PathBuf>; 3] {
    [
        dirs::config_dir(),
        libmacchina::dirs::localbase_dir(),
        libmacchina::dirs::usr_share_dir(),
    ]
}

// https://stackoverflow.com/questions/54267608/expand-tilde-in-rust-path-idiomatically
pub fn expand_home<P: AsRef<Path>>(path_user_input: P) -> Option<PathBuf> {
    let p = path_user_input.as_ref();
    if !p.starts_with("~") {
        return Some(p.to_path_buf());
    }

    if p.eq(Path::new("~")) {
        return dirs::home_dir();
    }

    dirs::home_dir().map(|mut h| {
        if h == Path::new("/") {
            // Corner case: `h` root directory;
            // don't prepend extra `/`, just drop the tilde.
            p.strip_prefix("~").unwrap().to_path_buf()
        } else {
            h.push(p.strip_prefix("~/").unwrap());
            h
        }
    })
}
