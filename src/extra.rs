use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// https://stackoverflow.com/questions/54267608/expand-tilde-in-rust-path-idiomatically
pub fn expand_home<P: AsRef<Path>>(path_user_input: P) -> Option<PathBuf> {
    let p = path_user_input.as_ref();
    if !p.starts_with("~") {
        return Some(p.to_path_buf());
    }
    if p == Path::new("~") {
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

pub fn localbase_dir() -> Option<PathBuf> {
    if cfg!(netbsd) {
        if let Ok(lines) = read_lines("/etc/mk.conf") {
            for line in lines {
                if let Ok(var) = line {
                    if var.starts_with("LOCALBASE") {
                        let localbase =
                            PathBuf::from(var.split("=").nth(1).unwrap().trim().to_string());
                        if localbase.is_dir() {
                            return Some(localbase);
                        }
                    }

                    continue;
                }
            }
        }

        return Some(PathBuf::from("/usr/pkg"));
    }

    None
}
