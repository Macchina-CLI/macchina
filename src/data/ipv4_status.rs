use crate::theme::Theme;
use libmacchina::traits::ReadoutError;
use ratatui::text::Text;

#[cfg(any(target_os = "macos", test))]
use std::net::Ipv4Addr;

#[cfg(target_os = "macos")]
pub(super) fn collect(
    theme: &Theme,
    requested_interface: Option<&str>,
) -> Result<Text<'static>, ReadoutError> {
    use ratatui::style::Style;
    use ratatui::text::{Line, Span};
    use std::process::Command;

    let mut route = Command::new("/sbin/route");
    route.args(["-n", "get", "-inet"]);
    if let Some(interface) = requested_interface {
        route.args(["-ifscope", interface]);
    }
    let output = route.arg("default").output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let route = if output.status.success() {
        parse_route(&stdout)
    } else {
        None
    };
    let interface = requested_interface
        .or_else(|| route.map(|(interface, _)| interface))
        .ok_or(ReadoutError::MetricNotAvailable)?;
    let gateway = match route {
        Some((route_interface, Some(ip))) if route_interface == interface => ip.to_string(),
        Some((route_interface, None)) if route_interface == interface => "no IPv4 gateway".into(),
        _ => "unavailable".into(),
    };
    let (address, netmask) = interface_ipv4(interface)?;

    let style = Style::default().fg(theme.get_key_color());
    let row = |label: &'static str, value: String| {
        Line::from(vec![Span::styled(label, style), Span::raw(value)])
    };
    Ok(Text::from(vec![
        Line::from(interface.to_owned()),
        row("Address: ", address.to_string()),
        row(
            "Netmask: ",
            netmask.map_or_else(|| "unavailable".to_owned(), |mask| mask.to_string()),
        ),
        row("Router:  ", gateway),
    ]))
}

#[cfg(target_os = "macos")]
fn interface_ipv4(interface: &str) -> Result<(Ipv4Addr, Option<Ipv4Addr>), ReadoutError> {
    use std::ffi::CStr;
    use std::{io, ptr, slice};

    struct Interfaces(*mut libc::ifaddrs);
    impl Drop for Interfaces {
        fn drop(&mut self) {
            // SAFETY: This pointer is the list returned by a successful getifaddrs call.
            unsafe { libc::freeifaddrs(self.0) };
        }
    }

    let mut head = ptr::null_mut();
    // SAFETY: head is writable storage for the list pointer.
    if unsafe { libc::getifaddrs(&mut head) } != 0 {
        return Err(io::Error::last_os_error().into());
    }
    let interfaces = Interfaces(head);

    // SAFETY: The list and its fields are owned by interfaces for this traversal.
    unsafe {
        let mut current = interfaces.0;
        while let Some(entry) = current.as_ref() {
            current = entry.ifa_next;
            if CStr::from_ptr(entry.ifa_name).to_bytes() != interface.as_bytes() {
                continue;
            }
            let address = entry.ifa_addr.cast::<u8>();
            if address.is_null() {
                continue;
            }
            if usize::from(*address) < size_of::<libc::sockaddr_in>()
                || i32::from(*address.add(1)) != libc::AF_INET
            {
                continue;
            }
            let address = &*entry.ifa_addr.cast::<libc::sockaddr_in>();
            let ip = Ipv4Addr::from(address.sin_addr.s_addr.to_ne_bytes());
            if ip.is_loopback() {
                continue;
            }

            let netmask = if entry.ifa_netmask.is_null() {
                None
            } else {
                let bytes = entry.ifa_netmask.cast::<u8>();
                // Darwin can omit the family and trailing zero bytes from a netmask.
                let len = usize::from(*bytes);
                decode_netmask(slice::from_raw_parts(bytes, len))
            };
            return Ok((ip, netmask));
        }
    }
    Err(ReadoutError::MetricNotAvailable)
}

#[cfg(any(target_os = "macos", test))]
fn decode_netmask(sockaddr: &[u8]) -> Option<Ipv4Addr> {
    // The IPv4 address starts at byte four in Darwin's sockaddr_in.
    let octets = sockaddr.get(4..sockaddr.len().min(8))?;
    let mut mask = [0; 4];
    mask[..octets.len()].copy_from_slice(octets);
    Some(Ipv4Addr::from(mask))
}

#[cfg(not(target_os = "macos"))]
pub(super) fn collect(_: &Theme, _: Option<&str>) -> Result<Text<'static>, ReadoutError> {
    Err(ReadoutError::NotImplemented)
}

#[cfg(any(target_os = "macos", test))]
fn parse_route(output: &str) -> Option<(&str, Option<Ipv4Addr>)> {
    let mut interface = None;
    let mut gateway = None;
    for line in output.lines() {
        let Some((key, value)) = line.trim().split_once(':') else {
            continue;
        };
        let value = value.trim();
        match key {
            "interface" if !value.is_empty() => interface = Some(value),
            "gateway" => gateway = value.parse().ok(),
            _ => {}
        }
    }
    Some((interface?, gateway))
}

#[cfg(test)]
mod tests {
    use super::{decode_netmask, parse_route, Ipv4Addr};

    #[test]
    fn decodes_compact_darwin_netmask() {
        assert_eq!(
            decode_netmask(&[5, 0, 0, 0, 255]),
            Some(Ipv4Addr::new(255, 0, 0, 0))
        );
    }

    #[test]
    fn parses_ipv4_and_link_layer_gateways() {
        for (gateway, expected) in [
            ("10.13.37.42", Some(Ipv4Addr::new(10, 13, 37, 42))),
            ("link#7", None),
        ] {
            let output = format!("   route to: default\n    gateway: {gateway}\n  interface: en0");
            assert_eq!(parse_route(&output), Some(("en0", expected)), "{output}");
        }
    }
}
