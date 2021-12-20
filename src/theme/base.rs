use crate::cli::Opt;
use crate::config;
use crate::data::ReadoutKey;
use crate::theme::components::*;
use crate::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tui::style::Color;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Theme {
    custom_ascii: ASCII,
    bar: Bar,
    r#box: Block,
    separator: String,
    randomize: Randomize,
    spacing: usize,
    padding: usize,
    palette: Palette,
    hide_ascii: bool,
    prefer_small_ascii: bool,
    keys: Keys,
    #[serde(with = "color_to_tui")]
    key_color: Color,
    #[serde(with = "color_to_tui")]
    separator_color: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            key_color: Color::Blue,
            separator_color: Color::Yellow,
            separator: String::from("-"),
            hide_ascii: false,
            prefer_small_ascii: false,
            palette: Palette::default(),
            spacing: 2,
            padding: 2,
            randomize: Randomize::default(),
            custom_ascii: ASCII::default(),
            bar: Bar::default(),
            r#box: Block::default(),
            keys: Keys::default(),
        }
    }
}

impl Theme {
    pub fn new(custom: Theme) -> Self {
        Theme {
            bar: custom.bar,
            key_color: custom.key_color,
            separator: custom.separator,
            separator_color: custom.separator_color,
            spacing: custom.spacing,
            padding: custom.padding,
            palette: custom.palette,
            hide_ascii: custom.hide_ascii,
            prefer_small_ascii: custom.prefer_small_ascii,
            r#box: custom.r#box,
            custom_ascii: custom.custom_ascii,
            randomize: custom.randomize,
            keys: custom.keys,
        }
    }

    pub fn get_bar(&self) -> &Bar {
        &self.bar
    }

    pub fn get_keys(&self) -> &Keys {
        &self.keys
    }

    pub fn get_randomization(&self) -> &Randomize {
        &self.randomize
    }

    pub fn get_custom_ascii(&self) -> &ASCII {
        &self.custom_ascii
    }

    pub fn get_palette(&self) -> &Palette {
        &self.palette
    }

    pub fn get_block(&self) -> &Block {
        &self.r#box
    }

    pub fn get_separator(&self) -> &str {
        &self.separator
    }

    pub fn get_key_color(&self) -> Color {
        self.key_color
    }

    pub fn get_separator_color(&self) -> Color {
        self.separator_color
    }

    pub fn prefers_small_ascii(&self) -> bool {
        self.prefer_small_ascii
    }

    pub fn is_ascii_visible(&self) -> bool {
        !self.hide_ascii
    }

    pub fn get_padding(&self) -> usize {
        self.padding
    }

    pub fn get_spacing(&self) -> usize {
        self.spacing
    }

    pub fn set_separator_color(&mut self, color: Color) {
        self.separator_color = color;
    }

    pub fn set_padding(&mut self, size: usize) {
        self.padding = size
    }

    pub fn set_spacing(&mut self, spacing: usize) {
        self.spacing = spacing;
    }

    pub fn set_key_color(&mut self, color: Color) {
        self.key_color = color;
    }

    pub fn set_separator(&mut self, separator: impl ToString) {
        self.separator = separator.to_string()
    }

    pub fn set_randomization(&mut self) {
        if self.randomize.rand_key() {
            self.key_color = self.randomize.generate();
        }

        if self.randomize.rand_sep() {
            self.separator_color = self.randomize.generate();
        }
    }

    pub fn key(&self, readout_key: &ReadoutKey) -> &str {
        match *readout_key {
            ReadoutKey::Host => self.get_keys().get_host(),
            ReadoutKey::Kernel => self.get_keys().get_kernel(),
            ReadoutKey::OperatingSystem => self.get_keys().get_os(),
            ReadoutKey::Machine => self.get_keys().get_machine(),
            ReadoutKey::Distribution => self.get_keys().get_distro(),
            ReadoutKey::LocalIP => self.get_keys().get_local_ip(),
            ReadoutKey::Resolution => self.get_keys().get_resolution(),
            ReadoutKey::Shell => self.get_keys().get_shell(),
            ReadoutKey::Terminal => self.get_keys().get_terminal(),
            ReadoutKey::WindowManager => self.get_keys().get_wm(),
            ReadoutKey::DesktopEnvironment => self.get_keys().get_de(),
            ReadoutKey::Packages => self.get_keys().get_packages(),
            ReadoutKey::Processor => self.get_keys().get_cpu(),
            ReadoutKey::ProcessorLoad => self.get_keys().get_cpu_load(),
            ReadoutKey::Battery => self.get_keys().get_battery(),
            ReadoutKey::Backlight => self.get_keys().get_backlight(),
            ReadoutKey::Uptime => self.get_keys().get_uptime(),
            ReadoutKey::Memory => self.get_keys().get_memory(),
        }
    }
}

/// Searches for and returns a theme from a given directory.
pub fn get_theme(path: &Path) -> Result<Theme> {
    let buffer = std::fs::read(path)?;
    Ok(toml::from_slice(&buffer)?)
}

pub fn create_theme(opt: &Opt) -> Theme {
    let mut theme = Theme::default();
    let locations = config::locations();
    if let Some(th) = &opt.theme {
        let t = locations.iter().find(|&d| {
            let theme_path = d.join(&format!("macchina/themes/{}.toml", th));
            match get_theme(&theme_path) {
                Ok(t) => {
                    theme = t;
                    theme.set_randomization();
                    true
                }
                _ => false,
            }
        });

        if t.is_none() {
            println!(
                "{}: \"{}\" could not be found, verify it exists with --list-themes.",
                "Error".red(),
                th
            )
        }
    }

    theme
}

pub fn list_themes(opt: &Opt) {
    let locations = config::locations();
    for dir in locations {
        let entries = libmacchina::extra::list_dir_entries(&dir.join("macchina/themes"));
        let custom_themes = entries.iter().filter(|&x| {
            if let Some(ext) = libmacchina::extra::path_extension(x) {
                ext == "toml"
            } else {
                false
            }
        });

        let n_themes = custom_themes.clone().count();
        if n_themes == 0 {
            continue;
        }

        println!("{}/macchina/themes:", dir.to_string_lossy());

        custom_themes.for_each(|x| {
            if let Some(theme) = x.file_name() {
                let name = theme.to_string_lossy().replace(".toml", "");
                if let Some(active_theme) = &opt.theme {
                    if active_theme == &name {
                        println!(
                            "- {} {}",
                            name.bright_green().italic(),
                            "[active]".bright_cyan()
                        );
                    } else {
                        println!("- {}", name.bright_green().italic());
                    }
                } else {
                    println!("- {}", name.bright_green().italic());
                }
            }
        });
    }
}
