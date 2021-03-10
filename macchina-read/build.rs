use std::env;

fn main() {
    match env::var("CARGO_CFG_TARGET_OS").as_ref().map(|x| &**x) {
        Ok("macos") => {
            println!("cargo:rustc-link-lib=framework=IOKit");
        }
        _ => {}
    }
}