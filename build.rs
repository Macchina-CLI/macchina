#![allow(dead_code)]
use vergen::{ShaKind,Config};
fn main() {
    let outdir = match std::env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };

    let stamp_path = std::path::Path::new(&outdir).join("macchina-stamp");
    if let Err(err) = std::fs::File::create(&stamp_path) {
        panic!("failed to write {}: {}", stamp_path.display(), err);
    }

    let mut config = Config::default();
    *config.git_mut().sha_kind_mut() = ShaKind::Short;
    // This will break builds if the git folder is removed
    vergen::vergen(config).unwrap(); 
}

