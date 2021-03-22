use crate::theme::Theme;
use crate::Opt;
use clap::arg_enum;
use libmacchina::traits::ReadoutError;
use libmacchina::{BatteryReadout, GeneralReadout, KernelReadout, MemoryReadout, PackageReadout};
use std::borrow::Cow;
use tui::style::{Color, Style};
use tui::text::{Span, Spans, Text};

arg_enum! {
    /// This enum contains all the possible keys, e.g. _Host_, _Machine_, _Kernel_, etc.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        Uptime,
        Processor,
        Memory,
        Battery,
        LocalIP,
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

fn create_bar<'a>(theme: &Box<dyn Theme>, blocks: usize) -> Spans<'a> {
    let mut span_vector = vec![
        Span::raw(format!("{} ", theme.get_bar_style().symbol_open)),
        Span::raw(""),
        Span::raw(""),
        Span::raw(format!(" {}", theme.get_bar_style().symbol_close)),
    ];

    let glyph = theme.get_bar_style().glyph;
    let glyphs = colored_glyphs(glyph, blocks);

    if blocks == 10 {
        span_vector[1].content = Cow::from(glyphs);
    } else {
        span_vector[1].content = Cow::from(format!("{} ", glyphs));
    }
    span_vector[1].style = Style::default().fg(theme.get_color());

    span_vector[2].content = Cow::from(colored_glyphs(glyph, 10 - blocks).to_string());
    if theme.get_color() == Color::White {
        span_vector[2].content = Cow::from(span_vector[2].content.replace(glyph, " "));
    }

    Spans::from(span_vector)
}

pub fn get_all_readouts<'a>(
    opt: &Opt,
    theme: &Box<dyn Theme>,
    should_display: Vec<ReadoutKey>,
) -> Vec<Readout<'a>> {
    let mut readout_values = Vec::with_capacity(ReadoutKey::variants().len());

    fn battery_readout(vec: &mut Vec<Readout>, theme: &Box<dyn Theme>, use_bar: bool) {
        use crate::format::battery as format_bat;
        use libmacchina::traits::BatteryReadout as _;

        let battery_readout = BatteryReadout::new();
        let key = ReadoutKey::Battery;

        let percentage = battery_readout.percentage();
        let state = battery_readout.status();

        match (percentage, state) {
            (Ok(p), Ok(s)) => {
                if use_bar {
                    let bar = create_bar(theme, crate::bars::battery(p));
                    vec.push(Readout::new(key, bar));
                } else {
                    vec.push(Readout::new(key, format_bat(p, s)));
                }
            }
            (Err(e), _) | (_, Err(e)) => vec.push(Readout::new_err(key, e)),
        }
    }

    fn package_readout(vec: &mut Vec<Readout>) {
        use crate::format::packages as format_pkgs;
        use libmacchina::traits::PackageReadout as _;

        let package_readout = PackageReadout::new();

        let packages = package_readout.count_pkgs();
        match format_pkgs(packages) {
            Ok(s) => vec.push(Readout::new(ReadoutKey::Packages, s)),
            Err(e) => vec.push(Readout::new_err(ReadoutKey::Packages, e)),
        }
    }

    fn kernel_readout(vec: &mut Vec<Readout>) {
        use libmacchina::traits::KernelReadout as _;

        let kernel_readout = KernelReadout::new();

        match kernel_readout.pretty_kernel() {
            Ok(s) => vec.push(Readout::new(ReadoutKey::Kernel, s)),
            Err(e) => vec.push(Readout::new_err(ReadoutKey::Kernel, e)),
        }
    }

    fn memory_readout(vec: &mut Vec<Readout>, theme: &Box<dyn Theme>, use_bar: bool) {
        use crate::format::memory as format_mem;
        use libmacchina::traits::MemoryReadout as _;

        let memory_readout = MemoryReadout::new();
        let total = memory_readout.total();
        let used = memory_readout.used();

        match (total, used, use_bar) {
            (Ok(total), Ok(used), true) => {
                let bar = create_bar(theme, crate::bars::memory(used, total));
                vec.push(Readout::new(ReadoutKey::Memory, bar))
            }
            (Ok(total), Ok(used), false) => {
                vec.push(Readout::new(ReadoutKey::Memory, format_mem(total, used)))
            }
            (Err(e), _, _) | (_, Err(e), _) => vec.push(Readout::new_err(ReadoutKey::Memory, e)),
        }
    }

    fn general_readout(vec: &mut Vec<Readout>, should_display: &[ReadoutKey], opt: &Opt) {
        use crate::format::cpu as format_cpu;
        use crate::format::host as format_host;
        use crate::format::uptime as format_uptime;
        use libmacchina::traits::GeneralReadout as _;

        let general_readout = GeneralReadout::new();

        if should_display.contains(&ReadoutKey::Host) {
            match (general_readout.username(), general_readout.hostname()) {
                (Ok(u), Ok(h)) => vec.push(Readout::new(ReadoutKey::Host, format_host(&u, &h))),
                (Err(e), _) | (_, Err(e)) => vec.push(Readout::new_err(ReadoutKey::LocalIP, e)),
            }
        }

        if should_display.contains(&ReadoutKey::Machine) {
            match general_readout.machine() {
                Ok(s) => vec.push(Readout::new(ReadoutKey::Machine, s)),
                Err(e) => vec.push(Readout::new_err(ReadoutKey::Machine, e)),
            }
        }

        if should_display.contains(&ReadoutKey::OperatingSystem) {
            match general_readout.os_name() {
                Ok(s) => vec.push(Readout::new(ReadoutKey::OperatingSystem, s)),
                Err(e) => vec.push(Readout::new_err(ReadoutKey::OperatingSystem, e)),
            }
        }

        if should_display.contains(&ReadoutKey::Distribution) {
            match general_readout.distribution() {
                Ok(s) => vec.push(Readout::new(ReadoutKey::Distribution, s)),
                Err(e) => vec.push(Readout::new_err(ReadoutKey::Distribution, e)),
            }
        }

        if should_display.contains(&ReadoutKey::Uptime) {
            match general_readout.uptime() {
                Ok(s) => vec.push(Readout::new(
                    ReadoutKey::Uptime,
                    format_uptime(s, opt.short_uptime),
                )),
                Err(e) => vec.push(Readout::new_err(ReadoutKey::Uptime, e)),
            }
        }

        if should_display.contains(&ReadoutKey::LocalIP) {
            match general_readout.local_ip() {
                Ok(s) => vec.push(Readout::new(ReadoutKey::LocalIP, s)),
                Err(e) => vec.push(Readout::new_err(ReadoutKey::LocalIP, e)),
            }
        }

        if should_display.contains(&ReadoutKey::Processor) {
            match general_readout.cpu_model_name() {
                Ok(s) => vec.push(Readout::new(ReadoutKey::Processor, format_cpu(&s))),
                Err(e) => vec.push(Readout::new_err(ReadoutKey::Processor, e)),
            }
        }

        if should_display.contains(&ReadoutKey::Shell) {
            match general_readout.shell(opt.short_shell) {
                Ok(s) => vec.push(Readout::new(ReadoutKey::Shell, s)),
                Err(e) => vec.push(Readout::new_err(ReadoutKey::Shell, e)),
            }
        }

        if should_display.contains(&ReadoutKey::Terminal) {
            match general_readout.terminal() {
                Ok(s) => vec.push(Readout::new(ReadoutKey::Terminal, s)),
                Err(e) => vec.push(Readout::new_err(ReadoutKey::Terminal, e)),
            }
        }

        let window_manager = general_readout.window_manager();
        let desktop_environment = general_readout.desktop_environment();

        // Check if the user is using only a Window Manager.
        match (window_manager, desktop_environment) {
            (Ok(w), Ok(d)) if w.to_uppercase() == d.to_uppercase() => {
                if should_display.contains(&ReadoutKey::WindowManager) {
                    vec.push(Readout::new(ReadoutKey::WindowManager, w));
                }
                vec.push(Readout::new_err(
                    ReadoutKey::DesktopEnvironment,
                    ReadoutError::Warning(String::from(
                        "You appear to be only running a window manager.",
                    )),
                ))
            }
            _ => {
                if should_display.contains(&ReadoutKey::DesktopEnvironment) {
                    match general_readout.desktop_environment() {
                        Ok(s) => vec.push(Readout::new(ReadoutKey::DesktopEnvironment, s)),
                        Err(e) => vec.push(Readout::new_err(ReadoutKey::DesktopEnvironment, e)),
                    }
                }
                if should_display.contains(&ReadoutKey::WindowManager) {
                    match general_readout.window_manager() {
                        Ok(s) => vec.push(Readout::new(ReadoutKey::WindowManager, s)),
                        Err(e) => vec.push(Readout::new_err(ReadoutKey::WindowManager, e)),
                    }
                }
            }
        }
    }

    general_readout(&mut readout_values, &should_display, &opt);

    if should_display.contains(&ReadoutKey::Kernel) {
        kernel_readout(&mut readout_values);
    }

    if should_display.contains(&ReadoutKey::Packages) {
        package_readout(&mut readout_values);
    }

    if should_display.contains(&ReadoutKey::Battery) {
        battery_readout(&mut readout_values, theme, opt.bar);
    }

    if should_display.contains(&ReadoutKey::Memory) {
        memory_readout(&mut readout_values, theme, opt.bar);
    }

    readout_values
}
