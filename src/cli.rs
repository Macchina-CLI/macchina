use crate::config;
use crate::data;
use crate::error;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::default::Default;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Parser, Debug, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Opt {
    #[clap(
        short = 'v',
        long = "version",
        help = "Prints version information",
        conflicts_with = "doctor"
    )]
    #[serde(skip_serializing, skip_deserializing)]
    pub version: bool,

    #[clap(
        short = 'o',
        long = "show",
        help = "Displays only the specified readouts",
        hide_possible_values = true,
        use_value_delimiter = true,
        value_delimiter = ','
    )]
    pub show: Option<Vec<data::ReadoutKey>>,

    #[clap(short = 'd', long = "doctor", help = "Checks the system for failures")]
    #[serde(skip_serializing, skip_deserializing)]
    pub doctor: bool,

    #[clap(short = 'U', long = "long-uptime", help = "Lengthens uptime output")]
    pub long_uptime: bool,

    #[clap(short = 'S', long = "long-shell", help = "Lengthens shell output")]
    pub long_shell: bool,

    #[clap(short = 'K', long = "long-kernel", help = "Lengthens kernel output")]
    pub long_kernel: bool,

    #[clap(
        short = 'G',
        long = "shorter-cpu",
        help = "Shorten the CPU's model name"
    )]
    pub shorter_cpu: bool,

    #[clap(
        short = 'm',
        long = "memory-percentage",
        help = "Show memory usage in percentage"
    )]
    pub memory_percentage: bool,

    #[clap(
        short = 'p',
        long = "disk-space-percentage",
        help = "Show disk space usage in percentage"
    )]
    pub disk_space_percentage: bool,

    #[clap(
        short = 'D',
        long = "disks",
        use_value_delimiter = true,
        value_delimiter = ',',
        help = "Comma separated list of disk(s) to show disk space readout for, e.g.
'/,/home/user/'"
    )]
    pub disks: Option<Vec<String>>,

    #[clap(
        short = 'C',
        long = "physical-cores",
        help = "Toggles between logical and physical cores"
    )]
    pub physical_cores: bool,

    #[clap(
        short = 's',
        long = "current-shell",
        help = "Toggles between the current shell and the default one"
    )]
    pub current_shell: bool,

    #[clap(short = 't', long = "theme", help = "Specify the name of the theme")]
    pub theme: Option<String>,

    #[clap(
        long = "list-themes",
        short = 'l',
        help = "Lists all available themes: built-in and custom"
    )]
    #[serde(skip_serializing, skip_deserializing)]
    pub list_themes: bool,

    #[clap(
        long = "config",
        short = 'c',
        help = "Specify a custom path for the configuration file"
    )]
    #[serde(skip_serializing, skip_deserializing)]
    pub config: Option<std::path::PathBuf>,

    #[clap(
        long = "ascii-artists",
        help = "Lists the original artists of the ASCII art used by macchina"
    )]
    #[serde(skip_serializing, skip_deserializing)]
    pub ascii_artists: bool,

    #[clap(
        long = "interface",
        short = 'i',
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

        if args.memory_percentage {
            self.memory_percentage = args.memory_percentage;
        }

        if args.disk_space_percentage {
            self.disk_space_percentage = args.disk_space_percentage;
        }

        if args.physical_cores {
            self.physical_cores = true;
        }

        if args.ascii_artists {
            self.ascii_artists = true;
        }

        if args.shorter_cpu {
            self.shorter_cpu = args.shorter_cpu;
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

        if args.disks.is_some() {
            self.disks = args.disks
        }
    }

    pub fn get_options() -> Opt {
        let args = Opt::parse();
        let config_opt = match args.config {
            Some(_) => config::read_config(&args.config.clone().unwrap()),
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
