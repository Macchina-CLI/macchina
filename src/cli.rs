use crate::config;
use crate::data;
use crate::error;
use serde::{Deserialize, Serialize};
use std::default::Default;
use structopt::StructOpt;

pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(StructOpt, Debug, Default, Serialize, Deserialize)]
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
        long = "export-config",
        short = "e",
        help = "Prints a template configuration file to stdout",
        conflicts_with = "doctor"
    )]
    #[serde(skip_serializing, skip_deserializing)]
    pub export_config: bool,

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
}

impl Opt {
    pub fn parse_args(&mut self, args: Opt) {
        if args.version {
            self.version = true;
        }

        if args.doctor {
            self.doctor = true;
        }

        if args.export_config {
            self.export_config = true;
        }

        if args.current_shell {
            self.current_shell = true;
        }

        if args.long_shell {
            self.long_shell = true;
        }

        if args.long_uptime {
            self.long_uptime = true;
        }

        if args.list_themes {
            self.list_themes = true;
        }

        if args.long_kernel {
            self.long_shell = true;
        }

        if args.physical_cores {
            self.physical_cores = true;
        }

        if args.ascii_artists {
            self.ascii_artists = true;
        }

        if args.config.is_some() {
            self.config = args.config;
        }

        if args.theme.is_some() {
            self.theme = args.theme;
        }

        if args.show.is_some() {
            self.show = args.show;
        }

        if args.interface.is_some() {
            self.interface = args.interface;
        }
    }

    pub fn get_options() -> Opt {
        let args = Opt::from_args();
        let config_opt = match args.config {
            Some(ref path) => config::read_config_file(path),
            None => config::get_config(),
        };

        match config_opt {
            Ok(mut config) => {
                config.parse_args(args);
                config
            }
            Err(e) => {
                error::print_errors(e);
                args
            }
        }
    }
}
