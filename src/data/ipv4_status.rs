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
    use if_addrs::IfAddr;
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
    let gateway = route
        .filter(|(route_interface, _)| *route_interface == interface)
        .and_then(|(_, gateway)| gateway);

    // Keep LocalIP's address selection so the mask comes from the same record.
    let address = if_addrs::get_if_addrs()?
        .into_iter()
        .filter(|entry| entry.name == interface && !entry.addr.is_loopback())
        .find_map(|entry| match entry.addr {
            IfAddr::V4(address) => Some(address),
            IfAddr::V6(_) => None,
        })
        .ok_or(ReadoutError::MetricNotAvailable)?;

    let style = Style::default().fg(theme.get_key_color());
    let row = |label: &'static str, value: String| {
        Line::from(vec![Span::styled(label, style), Span::raw(value)])
    };
    Ok(Text::from(vec![
        Line::from(interface.to_owned()),
        row("Address: ", address.ip.to_string()),
        row("Netmask: ", address.netmask.to_string()),
        row(
            "Router:  ",
            gateway.map_or_else(|| "unavailable".to_owned(), |ip| ip.to_string()),
        ),
    ]))
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
    use super::{parse_route, Ipv4Addr};

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
