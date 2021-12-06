use crate::data;
use clap::App;
use serde::{Deserialize, Serialize};
use std::default::Default;
use structopt::StructOpt;
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

#[derive(StructOpt, Debug, Serialize, Deserialize)]
#[structopt(author = AUTHORS, about = ABOUT)]
#[serde(default, deny_unknown_fields)]
pub struct Opt {
    #[structopt(
        short = "v",
        long = "version",
        help = "Prints version information",
        conflicts_with = "doctor"
    )]
    #[serde(skip_serializing, skip_deserializing)]
    pub version: bool,

    #[structopt(
    short = "o",
    long = "show",
    possible_values = & data::ReadoutKey::variants(),
    case_insensitive = true,
    help = "Displays only the specified readouts",
    min_values = 1,
    )]
    pub show: Option<Vec<data::ReadoutKey>>,

    #[structopt(short = "d", long = "doctor", help = "Checks the system for failures")]
    #[serde(skip_serializing, skip_deserializing)]
    pub doctor: bool,

    #[structopt(short = "U", long = "long-uptime", help = "Lengthens uptime output")]
    pub long_uptime: bool,

    #[structopt(short = "S", long = "long-shell", help = "Lengthens shell output")]
    pub long_shell: bool,

    #[structopt(short = "K", long = "long-kernel", help = "Lengthens kernel output")]
    pub long_kernel: bool,

    #[structopt(
        short = "C",
        long = "physical-cores",
        help = "Toggles between logical and physical cores"
    )]
    pub physical_cores: bool,

    #[structopt(
        short = "s",
        long = "current-shell",
        help = "Toggles between the current shell and the default one"
    )]
    pub current_shell: bool,

    #[structopt(
        short = "t",
        long = "theme",
        case_insensitive = true,
        help = "Specify the name of the theme"
    )]
    pub theme: Option<String>,

    #[structopt(
        long = "export-config",
        short = "e",
        help = "Prints a template configuration file to stdout",
        conflicts_with = "doctor"
    )]
    #[serde(skip_serializing, skip_deserializing)]
    pub export_config: bool,

    #[structopt(
        long = "list-themes",
        short = "l",
        help = "Lists all available themes: built-in and custom"
    )]
    #[serde(skip_serializing, skip_deserializing)]
    pub list_themes: bool,

    #[structopt(
        long = "config",
        short = "c",
        help = "Specify a custom path for the configuration file",
        conflicts_with = "export_config"
    )]
    #[serde(skip_serializing, skip_deserializing)]
    pub config: Option<std::path::PathBuf>,

    #[structopt(
        long = "ascii-artists",
        help = "Lists the original artists of the ASCII art used by macchina"
    )]
    #[serde(skip_serializing, skip_deserializing)]
    pub ascii_artists: bool,

    #[structopt(
        long = "interface",
        short = "i",
        help = "Specify the network interface for the LocalIP readout"
    )]
    pub interface: Option<String>,

    #[structopt(
        long = "verbose",
        short = "V",
        help = "Verbose output"
    )]
    pub verbose: bool,
}

impl Default for Opt {
    fn default() -> Self {
        Opt {
            verbose: false,
            version: false,
            doctor: false,
            export_config: false,
            current_shell: false,
            long_shell: false,
            long_uptime: false,
            long_kernel: true,
            list_themes: false,
            ascii_artists: false,
            physical_cores: false,
            config: None,
            theme: None,
            show: None,
            interface: None,
        }
    }
}

#[allow(dead_code)]
pub fn build_cli() -> App<'static, 'static> {
    Opt::clap()
}
