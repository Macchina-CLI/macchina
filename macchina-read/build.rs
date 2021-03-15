use std::env;

fn build_windows() {
    #[cfg(windows)]
    windows::build!(
        windows::win32::windows_programming::GetUserNameA,
        windows::win32::windows_programming::GetComputerNameExA,
        windows::win32::system_services::GlobalMemoryStatusEx,
        windows::win32::windows_programming::GetTickCount64,
        windows::win32::system_services::GetSystemPowerStatus,
    );
}

fn build_macos() {
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=IOKit");
}

fn main() {
    match env::var("CARGO_CFG_TARGET_OS").as_ref().map(|x| &**x) {
        Ok("macos") => build_macos(),
        Ok("windows") => build_windows(),
        _ => {}
    }
}
