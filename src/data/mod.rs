use crate::cli::Opt;
use crate::theme::Theme;
use clap::{Parser, ValueEnum};
use libmacchina::traits::GeneralReadout as _;
use libmacchina::traits::{ReadoutError, ShellFormat, ShellKind};
use libmacchina::{BatteryReadout, GeneralReadout, KernelReadout, MemoryReadout, PackageReadout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Display;

/// This enum contains all the possible keys, e.g. _Host_, _Machine_, _Kernel_, etc.
#[allow(clippy::upper_case_acronyms)]
#[derive(Parser, ValueEnum, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[clap(rename_all = "kebab-case")]
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
    GPU,
    DiskSpace,
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
            Self::GPU => write!(f, "GPU"),
            Self::DiskSpace => write!(f, "DiskSpace"),
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

fn create_bar<'a>(theme: &Theme, blocks: usize) -> Line<'a> {
    if theme.get_bar().are_delimiters_hidden() {
        let mut span_vector = vec![Span::raw(""), Span::raw("")];

        let glyph = theme.get_bar().get_glyph();
        let glyphs = colored_glyphs(glyph, blocks);

        if blocks == 10 {
            span_vector[0].content = Cow::from(glyphs);
        } else {
            span_vector[0].content = Cow::from(format!("{glyphs} "));
        }

        span_vector[0].style = Style::default().fg(theme.get_key_color());
        span_vector[1].content = Cow::from(colored_glyphs(glyph, 10 - blocks));

        if theme.get_key_color() == Color::White {
            span_vector[1].content = Cow::from(span_vector[1].content.replace(glyph, " "));
        }
        return Line::from(span_vector);
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
        span_vector[1].content = Cow::from(format!("{glyphs} "));
    }
    span_vector[1].style = Style::default().fg(theme.get_key_color());

    span_vector[2].content = Cow::from(colored_glyphs(glyph, 10 - blocks));
    if theme.get_key_color() == Color::White {
        span_vector[2].content = Cow::from(span_vector[2].content.replace(glyph, " "));
    }
    Line::from(span_vector)
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
    let mut readout_values = Vec::with_capacity(ReadoutKey::value_variants().len());
    let general_readout: GeneralReadout = GeneralReadout::new();

    for readout_key in should_display {
        match readout_key {
            ReadoutKey::Host => handle_readout_host(&mut readout_values, &general_readout),
            ReadoutKey::Machine => handle_readout_machine(&mut readout_values, &general_readout),
            ReadoutKey::Kernel => handle_readout_kernel(&mut readout_values, opt),
            ReadoutKey::OperatingSystem => {
                handle_readout_operating_system(&mut readout_values, &general_readout)
            }
            ReadoutKey::Distribution => {
                handle_readout_distribution(&mut readout_values, &general_readout)
            }
            ReadoutKey::Packages => handle_readout_packages(&mut readout_values),
            ReadoutKey::LocalIP => handle_readout_local_ip(&mut readout_values, opt),
            ReadoutKey::Terminal => handle_readout_terminal(&mut readout_values, &general_readout),
            ReadoutKey::Shell => handle_readout_shell(&mut readout_values, &general_readout, opt),
            ReadoutKey::Uptime => handle_readout_uptime(&mut readout_values, &general_readout, opt),
            ReadoutKey::Resolution => {
                handle_readout_resolution(&mut readout_values, &general_readout)
            }
            ReadoutKey::Backlight => {
                handle_readout_backlight(&mut readout_values, &general_readout, theme)
            }
            ReadoutKey::Processor => {
                handle_readout_processor(&mut readout_values, &general_readout, opt)
            }
            ReadoutKey::ProcessorLoad => {
                handle_readout_processor_load(&mut readout_values, &general_readout, theme)
            }
            ReadoutKey::Memory => handle_readout_memory(&mut readout_values, theme, opt),
            ReadoutKey::Battery => handle_readout_battery(&mut readout_values, theme),
            ReadoutKey::DesktopEnvironment => {
                handle_readout_desktop_environment(&mut readout_values, &general_readout)
            }
            ReadoutKey::WindowManager => {
                handle_readout_window_manager(&mut readout_values, &general_readout)
            }
            ReadoutKey::GPU => handle_readout_gpu(&mut readout_values, &general_readout),
            ReadoutKey::DiskSpace => {
                handle_readout_disk_space(&mut readout_values, &general_readout, theme, opt)
            }
        };
    }

    readout_values
}

// READOUT HANDLERS
fn handle_readout_host(readout_values: &mut Vec<Readout>, general_readout: &GeneralReadout) {
    use crate::format::host as format_host;

    match (general_readout.username(), general_readout.hostname()) {
        (Ok(u), Ok(h)) => readout_values.push(Readout::new(ReadoutKey::Host, format_host(&u, &h))),
        (Err(e), _) | (_, Err(e)) => readout_values.push(Readout::new_err(ReadoutKey::Host, e)),
    }
}

fn handle_readout_machine(readout_values: &mut Vec<Readout>, general_readout: &GeneralReadout) {
    match general_readout.machine() {
        Ok(s) => readout_values.push(Readout::new(ReadoutKey::Machine, s)),
        Err(e) => readout_values.push(Readout::new_err(ReadoutKey::Machine, e)),
    }
}

fn handle_readout_kernel(readout_values: &mut Vec<Readout>, opt: &Opt) {
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

fn handle_readout_operating_system(
    readout_values: &mut Vec<Readout>,
    general_readout: &GeneralReadout,
) {
    match general_readout.os_name() {
        Ok(s) => readout_values.push(Readout::new(ReadoutKey::OperatingSystem, s)),
        Err(e) => readout_values.push(Readout::new_err(ReadoutKey::OperatingSystem, e)),
    }
}
fn handle_readout_distribution(
    readout_values: &mut Vec<Readout>,
    general_readout: &GeneralReadout,
) {
    match general_readout.distribution() {
        Ok(s) => readout_values.push(Readout::new(ReadoutKey::Distribution, s)),
        Err(e) => readout_values.push(Readout::new_err(ReadoutKey::Distribution, e)),
    }
}

fn handle_readout_packages(readout_values: &mut Vec<Readout>) {
    use crate::format::packages as format_pkgs;
    use libmacchina::traits::PackageReadout as _;

    let package_readout = PackageReadout::new();

    let packages = package_readout.count_pkgs();
    match format_pkgs(packages) {
        Ok(s) => readout_values.push(Readout::new(ReadoutKey::Packages, s)),
        Err(e) => readout_values.push(Readout::new_err(ReadoutKey::Packages, e)),
    }
}

fn handle_readout_local_ip(readout_values: &mut Vec<Readout>, opt: &Opt) {
    use libmacchina::traits::NetworkReadout as _;
    use libmacchina::NetworkReadout;

    let network_readout = NetworkReadout::new();
    match network_readout.logical_address(opt.interface.as_deref()) {
        Ok(s) => readout_values.push(Readout::new(ReadoutKey::LocalIP, s)),
        Err(e) => readout_values.push(Readout::new_err(ReadoutKey::LocalIP, e)),
    }
}
fn handle_readout_terminal(readout_values: &mut Vec<Readout>, general_readout: &GeneralReadout) {
    match general_readout.terminal() {
        Ok(s) => readout_values.push(Readout::new(ReadoutKey::Terminal, s)),
        Err(e) => readout_values.push(Readout::new_err(ReadoutKey::Terminal, e)),
    }
}

fn handle_readout_shell(
    readout_values: &mut Vec<Readout>,
    general_readout: &GeneralReadout,
    opt: &Opt,
) {
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
fn handle_readout_uptime(
    readout_values: &mut Vec<Readout>,
    general_readout: &GeneralReadout,
    opt: &Opt,
) {
    use crate::format::uptime as format_uptime;
    match general_readout.uptime() {
        Ok(s) => readout_values.push(Readout::new(
            ReadoutKey::Uptime,
            format_uptime(s, opt.long_uptime),
        )),
        Err(e) => readout_values.push(Readout::new_err(ReadoutKey::Uptime, e)),
    }
}

fn handle_readout_resolution(readout_values: &mut Vec<Readout>, general_readout: &GeneralReadout) {
    match general_readout.resolution() {
        Ok(r) => readout_values.push(Readout::new(ReadoutKey::Resolution, r)),
        Err(e) => readout_values.push(Readout::new_err(ReadoutKey::Resolution, e)),
    }
}

fn handle_readout_backlight(
    readout_values: &mut Vec<Readout>,
    general_readout: &GeneralReadout,
    theme: &Theme,
) {
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

fn handle_readout_processor(
    readout_values: &mut Vec<Readout>,
    general_readout: &GeneralReadout,
    opt: &Opt,
) {
    use crate::format::cpu as format_cpu;
    use crate::format::cpu_only as format_cpu_only;

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
        (Ok(m), _) => readout_values.push(Readout::new(ReadoutKey::Processor, format_cpu_only(&m))),
        (Err(e), _) => readout_values.push(Readout::new_err(ReadoutKey::Processor, e)),
    }
}

fn handle_readout_processor_load(
    readout_values: &mut Vec<Readout>,
    general_readout: &GeneralReadout,
    theme: &Theme,
) {
    use crate::format::cpu_usage as format_cpu_usage;
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

fn handle_readout_memory(readout_values: &mut Vec<Readout>, theme: &Theme, opt: &Opt) {
    use crate::format::memory as format_mem;
    use libmacchina::traits::MemoryReadout as _;

    let memory_readout = MemoryReadout::new();
    let total = memory_readout.total();
    let used = memory_readout.used();

    match (total, used) {
        (Ok(total), Ok(used)) => {
            if theme.get_bar().is_visible() {
                let bar = create_bar(theme, crate::bars::usage(used, total));
                readout_values.push(Readout::new(ReadoutKey::Memory, bar))
            } else {
                readout_values.push(Readout::new(
                    ReadoutKey::Memory,
                    format_mem(total, used, opt.memory_percentage),
                ))
            }
        }
        (Err(e), _) | (_, Err(e)) => readout_values.push(Readout::new_err(ReadoutKey::Memory, e)),
    }
}

fn handle_readout_battery(readout_values: &mut Vec<Readout>, theme: &Theme) {
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

fn handle_readout_desktop_environment(
    readout_values: &mut Vec<Readout>,
    general_readout: &GeneralReadout,
) {
    let desktop_environment = general_readout.desktop_environment();
    let window_manager = general_readout.window_manager();

    match &desktop_environment {
        Ok(d) => match &window_manager {
            // check if the user is using a window manager only.
            Ok(w) if w.eq_ignore_ascii_case(d) => {
                readout_values.push(Readout::new_err(
                    ReadoutKey::DesktopEnvironment,
                    ReadoutError::Warning(String::from(
                        "You appear to be only running a window manager.",
                    )),
                ));
            }
            _ => {
                readout_values.push(Readout::new(ReadoutKey::DesktopEnvironment, d.to_owned()));
            }
        },
        Err(e) => readout_values.push(Readout::new_err(
            ReadoutKey::DesktopEnvironment,
            e.to_owned(),
        )),
    }
}

fn handle_readout_window_manager(
    readout_values: &mut Vec<Readout>,
    general_readout: &GeneralReadout,
) {
    let window_manager = general_readout.window_manager();
    let session = general_readout.session();

    match window_manager {
        Ok(w) => match session {
            Ok(s) => {
                readout_values.push(Readout::new(
                    ReadoutKey::WindowManager,
                    format!("{} ({})", w, s),
                ));
            }
            _ => readout_values.push(Readout::new(ReadoutKey::WindowManager, w)),
        },
        Err(e) => readout_values.push(Readout::new_err(ReadoutKey::WindowManager, e)),
    }
}

fn handle_readout_gpu(readout_values: &mut Vec<Readout>, general_readout: &GeneralReadout) {
    match general_readout.gpus() {
        Ok(gpus) => {
            for gpu in gpus {
                readout_values.push(Readout::new(ReadoutKey::GPU, gpu));
            }
        }

        Err(e) => readout_values.push(Readout::new_err(ReadoutKey::GPU, e)),
    };
}

fn handle_readout_disk_space(
    readout_values: &mut Vec<Readout>,
    general_readout: &GeneralReadout,
    theme: &Theme,
    opt: &Opt,
) {
    use crate::format::disk_space as format_disk_space;

    match general_readout.disk_space() {
        Ok((used, total)) => {
            if theme.get_bar().is_visible() {
                let bar = create_bar(theme, crate::bars::usage(used, total));
                readout_values.push(Readout::new(ReadoutKey::DiskSpace, bar))
            } else {
                readout_values.push(Readout::new(
                    ReadoutKey::DiskSpace,
                    format_disk_space(used, total, opt.disk_space_percentage),
                ))
            }
        }
        Err(e) => readout_values.push(Readout::new_err(ReadoutKey::DiskSpace, e)),
    }
}
