use clap::arg_enum;
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
pub struct Readout<'a>(pub ReadoutKey, pub Text<'a>);

impl<'a> Readout<'a> {
    pub fn new<T>(readout_key: ReadoutKey, text: T) -> Readout<'a> where T: Into<Text<'a>> {
        Readout(readout_key, text.into())
    }
}