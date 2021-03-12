use std::env;

fn main() {
    match env::var("CARGO_CFG_TARGET_OS").as_ref().map(|x| &**x) {
        Ok("macos") | Ok("ios") => {
            println!("cargo:rustc-link-lib=framework=Foundation");
            println!("cargo:rustc-link-lib=framework=IOKit");
        },
        Ok("windows") => {
            windows::build!(
                windows::win32::windows_programming::GetUserNameA,
                windows::win32::windows_programming::GetComputerNameExA
            );
        }
        _ => {}
    }
}