use std::process::{Command, Stdio};
use macchina_read::traits::ReadoutError;

#[cfg(target_os = "netbsd")]
/// Extract package count using `pkg_info | wc -l`
pub fn package_count(fail: &mut Fail) -> String {
    if extra::which("pkg_info") {
        let pkg_info = Command::new("pkg_info")
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to spawn \"pkg_info\" process");

        let pkg_out = pkg_info
            .stdout
            .expect("ERROR: failed to open \"pkg_info\" stdout");

        let count = Command::new("wc")
            .arg("-l")
            .stdin(Stdio::from(pkg_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to start \"wc\" process");

        let output = count
            .wait_with_output()
            .expect("ERROR: failed to wait on for \"wc\" process to exit");
        return String::from_utf8(output.stdout)
            .expect("ERROR: \"pkg_info | wc -l\" output was not valid UTF-8")
            .trim()
            .to_string();
    }
    fail.battery.fail_component();
    return String::from("0");
}