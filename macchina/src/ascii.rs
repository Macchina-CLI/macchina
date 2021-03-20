
#[cfg(target_os = "macos")]
pub(crate) fn get_ascii_art() -> Box<&[&str]> {
    const ASCII_ARRAY: &[&str] = &[
        r#""#
    ];

    Box::new(ASCII_ARRAY)
}

#[cfg(target_os = "windows")]
pub(crate) fn get_ascii_art() -> Vec<&str> {

}

#[cfg(target_os = "linux")]
pub(crate) fn get_ascii_art() -> Vec<&str> {
    Vec::new()
}

#[cfg(target_os = "netbsd")]
pub(crate) fn get_ascii_art() -> Vec<&str> {
    Vec::new()
}