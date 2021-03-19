use crate::Opt;
use clap::arg_enum;
use macchina_read::traits::ReadoutError;
use macchina_read::{BatteryReadout, GeneralReadout, KernelReadout, MemoryReadout, PackageReadout};
use tui::text::Text;

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

pub fn get_all_readouts<'a>(opt: &Opt, should_display: Vec<ReadoutKey>) -> Vec<Readout<'a>> {
    let mut readout_values = Vec::with_capacity(ReadoutKey::variants().len());

    fn battery_readout(vec: &mut Vec<Readout>) {
        use crate::format::battery as format_bat;
        use macchina_read::traits::BatteryReadout as _;

        let battery_readout = BatteryReadout::new();
        let key = ReadoutKey::Battery;

        let percentage = battery_readout.percentage();
        let state = battery_readout.status();

        match (percentage, state) {
            (Ok(p), Ok(s)) => vec.push(Readout::new(key, format_bat(p, s))),
            (Err(e), _) | (_, Err(e)) => vec.push(Readout::new_err(key, e)),
        }
    }

    fn package_readout(vec: &mut Vec<Readout>) {
        use crate::format::packages as format_pkgs;
        use macchina_read::traits::PackageReadout as _;

        let package_readout = PackageReadout::new();

        let packages = package_readout.count_pkgs();
        match format_pkgs(packages) {
            Ok(s) => vec.push(Readout::new(ReadoutKey::Packages, s)),
            Err(e) => vec.push(Readout::new_err(ReadoutKey::Packages, e)),
        }
    }

    fn kernel_readout(vec: &mut Vec<Readout>) {
        use macchina_read::traits::KernelReadout as _;

        let kernel_readout = KernelReadout::new();

        match kernel_readout.pretty_kernel() {
            Ok(s) => vec.push(Readout::new(ReadoutKey::Kernel, s)),
            Err(e) => vec.push(Readout::new_err(ReadoutKey::Kernel, e)),
        }
    }

    fn memory_readout(vec: &mut Vec<Readout>) {
        use crate::format::memory as format_mem;
        use macchina_read::traits::MemoryReadout as _;

        let memory_readout = MemoryReadout::new();
        let total = memory_readout.total();
        let used = memory_readout.used();

        match (total, used) {
            (Ok(total), Ok(used)) => {
                vec.push(Readout::new(ReadoutKey::Memory, format_mem(total, used)))
            }
            (Err(e), _) | (_, Err(e)) => vec.push(Readout::new_err(ReadoutKey::Memory, e)),
        }
    }

    fn general_readout(vec: &mut Vec<Readout>, should_display: &Vec<ReadoutKey>, opt: &Opt) {
        use crate::format::cpu as format_cpu;
        use crate::format::host as format_host;
        use crate::format::uptime as format_uptime;
        use macchina_read::traits::GeneralReadout as _;

        let general_readout = GeneralReadout::new();

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

        if should_display.contains(&ReadoutKey::Host) {
            match (general_readout.username(), general_readout.hostname()) {
                (Ok(u), Ok(h)) => vec.push(Readout::new(ReadoutKey::Host, format_host(&u, &h))),
                (Err(e), _) | (_, Err(e)) => vec.push(Readout::new_err(ReadoutKey::LocalIP, e)),
            }
        }

        if should_display.contains(&ReadoutKey::Processor) {
            match general_readout.cpu_model_name() {
                Ok(s) => vec.push(Readout::new(ReadoutKey::Processor, format_cpu(&s))),
                Err(e) => vec.push(Readout::new_err(ReadoutKey::Processor, e)),
            }
        }

        if should_display.contains(&ReadoutKey::Machine) {
            match general_readout.machine() {
                Ok(s) => vec.push(Readout::new(ReadoutKey::Machine, s)),
                Err(e) => vec.push(Readout::new_err(ReadoutKey::Machine, e)),
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

        if should_display.contains(&ReadoutKey::Distribution) {
            match general_readout.distribution() {
                Ok(s) => vec.push(Readout::new(ReadoutKey::Distribution, s)),
                Err(e) => vec.push(Readout::new_err(ReadoutKey::Distribution, e)),
            }
        }

        let window_manager = general_readout.window_manager();
        let desktop_environment = general_readout.desktop_environment();

        // Check if the user is using only a Window Manager.
        match (window_manager, desktop_environment) {
            (Ok(w), Ok(d)) if w.to_uppercase() == d.to_uppercase() => {
                vec.push(Readout::new(ReadoutKey::WindowManager, w));
                vec.push(Readout::new_err(
                    ReadoutKey::DesktopEnvironment,
                    ReadoutError::Warning(String::from(
                        "You appear to be only running a window manager.",
                    )),
                ))
            }
            _ => {
                match general_readout.window_manager() {
                    Ok(s) => vec.push(Readout::new(ReadoutKey::WindowManager, s)),
                    Err(e) => vec.push(Readout::new_err(ReadoutKey::WindowManager, e)),
                }

                match general_readout.desktop_environment() {
                    Ok(s) => vec.push(Readout::new(ReadoutKey::DesktopEnvironment, s)),
                    Err(e) => vec.push(Readout::new_err(ReadoutKey::DesktopEnvironment, e)),
                }
            }
        }

        match general_readout.os_name() {
            Ok(s) => vec.push(Readout::new(ReadoutKey::OperatingSystem, s)),
            Err(e) => vec.push(Readout::new_err(ReadoutKey::OperatingSystem, e)),
        }
    }

    if should_display.contains(&ReadoutKey::Battery) {
        battery_readout(&mut readout_values);
    }

    if should_display.contains(&ReadoutKey::Packages) {
        package_readout(&mut readout_values);
    }

    if should_display.contains(&ReadoutKey::Kernel) {
        kernel_readout(&mut readout_values);
    }

    if should_display.contains(&ReadoutKey::Memory) {
        memory_readout(&mut readout_values);
    }

    general_readout(&mut readout_values, &should_display, &opt);

    readout_values
}
