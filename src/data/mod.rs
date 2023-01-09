use crate::cli::Opt;
use crate::theme::Theme;
use clap::{Parser, ValueEnum};
use libmacchina::traits::ShellFormat;
use libmacchina::traits::{ReadoutError, ShellKind};
use libmacchina::{BatteryReadout, GeneralReadout, KernelReadout, MemoryReadout, PackageReadout};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Display;
use tui::style::{Color, Style};
use tui::text::{Span, Spans, Text};

/// This enum contains all the possible keys, e.g. _Host_, _Machine_, _Kernel_, etc.
#[allow(clippy::upper_case_acronyms)]
#[derive(Parser, ValueEnum, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReadoutKey {
    Host,
    Machine,
    Kernel,
    Distribution,
    OperatingSystem,
    DesktopEnvironment,
    WindowManager,
    Packages,
    Shell,
    Terminal,
    LocalIP,
    Backlight,
    Resolution,
    Uptime,
    Processor,
    ProcessorLoad,
    Memory,
    Battery,
}

impl Display for ReadoutKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Host => write!(f, "Host"),
            Self::Machine => write!(f, "Machine"),
            Self::Kernel => write!(f, "Kernel"),
            Self::Distribution => write!(f, "Distribution"),
            Self::OperatingSystem => write!(f, "OperatingSystem"),
            Self::DesktopEnvironment => write!(f, "DesktopEnvironment"),
            Self::WindowManager => write!(f, "WindowManager"),
            Self::Packages => write!(f, "Packages"),
            Self::Shell => write!(f, "Shell"),
            Self::Terminal => write!(f, "Terminal"),
            Self::LocalIP => write!(f, "LocalIP"),
            Self::Backlight => write!(f, "Backlight"),
            Self::Resolution => write!(f, "Resolution"),
            Self::Uptime => write!(f, "Uptime"),
            Self::Processor => write!(f, "Processor"),
            Self::ProcessorLoad => write!(f, "ProcessorLoad"),
            Self::Memory => write!(f, "Memory"),
            Self::Battery => write!(f, "Battery"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Readout<'a>(pub ReadoutKey, pub Result<Text<'a>, ReadoutError>);

impl<'a> Readout<'a> {
    pub fn new_err(readout_key: ReadoutKey, err: ReadoutError) -> Readout<'a> {
        Readout(readout_key, Err(err))
    }

    pub fn new<T>(readout_key: ReadoutKey, text: T) -> Readout<'a>
    where
        T: Into<Text<'a>>,
    {
        Readout(readout_key, Ok(text.into()))
    }
}

fn colored_glyphs(glyph: &str, blocks: usize) -> String {
    glyph
        .repeat(blocks)
        .chars()
        .collect::<Vec<char>>()
        .chunks(1)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

fn create_bar<'a>(theme: &Theme, blocks: usize) -> Spans<'a> {
    if theme.get_bar().are_delimiters_hidden() {
        let mut span_vector = vec![Span::raw(""), Span::raw("")];

        let glyph = theme.get_bar().get_glyph();
        let glyphs = colored_glyphs(glyph, blocks);

        if blocks == 10 {
            span_vector[0].content = Cow::from(glyphs);
        } else {
            span_vector[0].content = Cow::from(format!("{} ", glyphs));
        }

        span_vector[0].style = Style::default().fg(theme.get_key_color());
        span_vector[1].content = Cow::from(colored_glyphs(glyph, 10 - blocks));

        if theme.get_key_color() == Color::White {
            span_vector[1].content = Cow::from(span_vector[1].content.replace(&glyph, " "));
        }
        return Spans::from(span_vector);
    }

    let mut span_vector = vec![
        Span::raw(format!("{} ", theme.get_bar().get_symbol_open())),
        Span::raw(""),
        Span::raw(""),
        Span::raw(format!(" {}", theme.get_bar().get_symbol_close())),
    ];

    let glyph = theme.get_bar().get_glyph();
    let glyphs = colored_glyphs(glyph, blocks);

    if blocks == 10 {
        span_vector[1].content = Cow::from(glyphs);
    } else {
        span_vector[1].content = Cow::from(format!("{} ", glyphs));
    }
    span_vector[1].style = Style::default().fg(theme.get_key_color());

    span_vector[2].content = Cow::from(colored_glyphs(glyph, 10 - blocks));
    if theme.get_key_color() == Color::White {
        span_vector[2].content = Cow::from(span_vector[2].content.replace(&glyph, " "));
    }
    Spans::from(span_vector)
}

pub fn should_display(opt: &Opt) -> Vec<ReadoutKey> {
    if let Some(shown) = opt.show.to_owned() {
        return shown;
    }

    let keys: Vec<ReadoutKey> = ReadoutKey::value_variants()
        .iter()
        .map(|f| ReadoutKey::from_str(&f.to_string(), true).unwrap_or(*f))
        .collect();

    keys
}

pub fn get_all_readouts<'a>(
    opt: &Opt,
    theme: &Theme,
    should_display: &[ReadoutKey],
) -> Vec<Readout<'a>> {
    use crate::format::cpu as format_cpu;
    use crate::format::cpu_only as format_cpu_only;
    use crate::format::cpu_usage as format_cpu_usage;
    use crate::format::host as format_host;
    use crate::format::uptime as format_uptime;
    use libmacchina::traits::GeneralReadout as _;
    let mut readout_values = Vec::with_capacity(ReadoutKey::value_variants().len());
    let general_readout = GeneralReadout::new();

    if should_display.contains(&ReadoutKey::Host) {
        match (general_readout.username(), general_readout.hostname()) {
            (Ok(u), Ok(h)) => {
                readout_values.push(Readout::new(ReadoutKey::Host, format_host(&u, &h)))
            }
            (Err(e), _) | (_, Err(e)) => readout_values.push(Readout::new_err(ReadoutKey::Host, e)),
        }
    }

    if should_display.contains(&ReadoutKey::Machine) {
        match general_readout.machine() {
            Ok(s) => readout_values.push(Readout::new(ReadoutKey::Machine, s)),
            Err(e) => readout_values.push(Readout::new_err(ReadoutKey::Machine, e)),
        }
    }

    if should_display.contains(&ReadoutKey::Kernel) {
        use libmacchina::traits::KernelReadout as _;

        let kernel_readout = KernelReadout::new();

        if opt.long_kernel {
            match kernel_readout.pretty_kernel() {
                Ok(s) => readout_values.push(Readout::new(ReadoutKey::Kernel, s)),
                Err(e) => readout_values.push(Readout::new_err(ReadoutKey::Kernel, e)),
            }
        } else {
            match kernel_readout.os_release() {
                Ok(s) => readout_values.push(Readout::new(ReadoutKey::Kernel, s)),
                Err(e) => readout_values.push(Readout::new_err(ReadoutKey::Kernel, e)),
            }
        }
    }

    if should_display.contains(&ReadoutKey::OperatingSystem) {
        match general_readout.os_name() {
            Ok(s) => readout_values.push(Readout::new(ReadoutKey::OperatingSystem, s)),
            Err(e) => readout_values.push(Readout::new_err(ReadoutKey::OperatingSystem, e)),
        }
    }

    if should_display.contains(&ReadoutKey::Distribution) {
        match general_readout.distribution() {
            Ok(s) => readout_values.push(Readout::new(ReadoutKey::Distribution, s)),
            Err(e) => readout_values.push(Readout::new_err(ReadoutKey::Distribution, e)),
        }
    }

    let desktop_environment = general_readout.desktop_environment();
    let window_manager = general_readout.window_manager();
    let session = general_readout.session();

    match (&window_manager, &desktop_environment) {
        // check if the user is using a window manager only.
        (Ok(w), Ok(d)) if w.eq_ignore_ascii_case(d) => {
            if should_display.contains(&ReadoutKey::WindowManager) {
                match session {
                    Ok(s) => readout_values.push(Readout::new(
                        ReadoutKey::WindowManager,
                        format!("{} ({})", w, s),
                    )),
                    _ => readout_values.push(Readout::new(ReadoutKey::WindowManager, w.to_owned())),
                }
            }

            readout_values.push(Readout::new_err(
                ReadoutKey::DesktopEnvironment,
                ReadoutError::Warning(String::from(
                    "You appear to be only running a window manager.",
                )),
            ))
        }
        _ => {
            if should_display.contains(&ReadoutKey::DesktopEnvironment) {
                match desktop_environment {
                    Ok(s) => readout_values.push(Readout::new(ReadoutKey::DesktopEnvironment, s)),
                    Err(e) => {
                        readout_values.push(Readout::new_err(ReadoutKey::DesktopEnvironment, e))
                    }
                }
            }

            if should_display.contains(&ReadoutKey::WindowManager) {
                match window_manager {
                    Ok(w) => match session {
                        Ok(s) => readout_values.push(Readout::new(
                            ReadoutKey::WindowManager,
                            format!("{} ({})", w, s),
                        )),
                        _ => readout_values.push(Readout::new(ReadoutKey::WindowManager, w)),
                    },
                    Err(e) => readout_values.push(Readout::new_err(ReadoutKey::WindowManager, e)),
                }
            }
        }
    }

    if should_display.contains(&ReadoutKey::Packages) {
        use crate::format::packages as format_pkgs;
        use libmacchina::traits::PackageReadout as _;

        let package_readout = PackageReadout::new();

        let packages = package_readout.count_pkgs();
        match format_pkgs(packages) {
            Ok(s) => readout_values.push(Readout::new(ReadoutKey::Packages, s)),
            Err(e) => readout_values.push(Readout::new_err(ReadoutKey::Packages, e)),
        }
    }

    if should_display.contains(&ReadoutKey::LocalIP) {
        use libmacchina::traits::NetworkReadout as _;
        use libmacchina::NetworkReadout;

        let network_readout = NetworkReadout::new();
        match network_readout.logical_address(opt.interface.as_deref()) {
            Ok(s) => readout_values.push(Readout::new(ReadoutKey::LocalIP, s)),
            Err(e) => readout_values.push(Readout::new_err(ReadoutKey::LocalIP, e)),
        }
    }

    if should_display.contains(&ReadoutKey::Terminal) {
        match general_readout.terminal() {
            Ok(s) => readout_values.push(Readout::new(ReadoutKey::Terminal, s)),
            Err(e) => readout_values.push(Readout::new_err(ReadoutKey::Terminal, e)),
        }
    }

    if should_display.contains(&ReadoutKey::Shell) {
        let (ls, cs) = (
            if opt.long_shell {
                ShellFormat::Absolute
            } else {
                ShellFormat::Relative
            },
            if opt.current_shell {
                ShellKind::Current
            } else {
                ShellKind::Default
            },
        );

        match general_readout.shell(ls, cs) {
            Ok(s) => readout_values.push(Readout::new(ReadoutKey::Shell, s)),
            Err(e) => readout_values.push(Readout::new_err(ReadoutKey::Shell, e)),
        };
    }

    if should_display.contains(&ReadoutKey::Uptime) {
        match general_readout.uptime() {
            Ok(s) => readout_values.push(Readout::new(
                ReadoutKey::Uptime,
                format_uptime(s, opt.long_uptime),
            )),
            Err(e) => readout_values.push(Readout::new_err(ReadoutKey::Uptime, e)),
        }
    }

    if should_display.contains(&ReadoutKey::Processor) {
        let cores = {
            if opt.physical_cores {
                general_readout.cpu_physical_cores()
            } else {
                general_readout.cpu_cores()
            }
        };

        match (general_readout.cpu_model_name(), cores) {
            (Ok(m), Ok(c)) => {
                readout_values.push(Readout::new(ReadoutKey::Processor, format_cpu(&m, c)))
            }
            (Ok(m), _) => {
                readout_values.push(Readout::new(ReadoutKey::Processor, format_cpu_only(&m)))
            }
            (Err(e), _) => readout_values.push(Readout::new_err(ReadoutKey::Processor, e)),
        }
    }

    if should_display.contains(&ReadoutKey::Resolution) {
        match general_readout.resolution() {
            Ok(r) => readout_values.push(Readout::new(ReadoutKey::Resolution, r)),
            Err(e) => readout_values.push(Readout::new_err(ReadoutKey::Resolution, e)),
        }
    }

    if should_display.contains(&ReadoutKey::Backlight) {
        match (general_readout.backlight(), theme.get_bar().is_visible()) {
            (Ok(b), false) => {
                readout_values.push(Readout::new(ReadoutKey::Backlight, format!("{}%", b)))
            }
            (Ok(b), true) => readout_values.push(Readout::new(
                ReadoutKey::Backlight,
                create_bar(theme, crate::bars::num_to_blocks(b as u8)),
            )),
            (Err(e), _) => readout_values.push(Readout::new_err(ReadoutKey::Backlight, e)),
        }
    }

    if should_display.contains(&ReadoutKey::ProcessorLoad) {
        match (general_readout.cpu_usage(), theme.get_bar().is_visible()) {
            (Ok(u), true) => {
                if u > 100 {
                    readout_values.push(Readout::new(
                        ReadoutKey::ProcessorLoad,
                        create_bar(theme, crate::bars::num_to_blocks(100_u8)),
                    ))
                }
                readout_values.push(Readout::new(
                    ReadoutKey::ProcessorLoad,
                    create_bar(theme, crate::bars::num_to_blocks(u as u8)),
                ))
            }
            (Ok(u), _) => {
                readout_values.push(Readout::new(ReadoutKey::ProcessorLoad, format_cpu_usage(u)))
            }
            (Err(e), _) => readout_values.push(Readout::new_err(ReadoutKey::ProcessorLoad, e)),
        }
    }

    if should_display.contains(&ReadoutKey::Memory) {
        use crate::format::memory as format_mem;
        use libmacchina::traits::MemoryReadout as _;

        let memory_readout = MemoryReadout::new();
        let total = memory_readout.total();
        let used = memory_readout.used();

        match (total, used) {
            (Ok(total), Ok(used)) => {
                if theme.get_bar().is_visible() {
                    let bar = create_bar(theme, crate::bars::memory(used, total));
                    readout_values.push(Readout::new(ReadoutKey::Memory, bar))
                } else {
                    readout_values.push(Readout::new(ReadoutKey::Memory, format_mem(total, used)))
                }
            }
            (Err(e), _) | (_, Err(e)) => {
                readout_values.push(Readout::new_err(ReadoutKey::Memory, e))
            }
        }
    }

    if should_display.contains(&ReadoutKey::Battery) {
        use crate::format::battery as format_bat;
        use libmacchina::traits::BatteryReadout as _;

        let battery_readout = BatteryReadout::new();
        let key = ReadoutKey::Battery;

        let percentage = battery_readout.percentage();
        let state = battery_readout.status();

        match (percentage, state) {
            (Ok(p), Ok(s)) => {
                if theme.get_bar().is_visible() {
                    let bar = create_bar(theme, crate::bars::num_to_blocks(p));
                    readout_values.push(Readout::new(key, bar));
                } else {
                    readout_values.push(Readout::new(key, format_bat(p, s)));
                }
            }
            (Err(e), _) | (_, Err(e)) => readout_values.push(Readout::new_err(key, e)),
        }
    }

    readout_values
}
