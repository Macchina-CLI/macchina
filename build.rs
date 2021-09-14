use std::env;
use vergen::{Config, ShaKind};

fn main() {
    let target = env::var("TARGET").unwrap();

    if !target.contains("netbsd") {
        let mut config = Config::default();
        *config.git_mut().sha_kind_mut() = ShaKind::Short;

        if let Err(e) = vergen::vergen(config) {
            eprintln!("{}", e)
        }
    }
}
