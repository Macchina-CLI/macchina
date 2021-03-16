#![allow(dead_code)]
use colored::{Color, ColoredString, Colorize};
use std::collections::HashMap;
use clap::arg_enum;

/// `Misc` contains several elements that make up a `Theme`, such as the: \
/// - Separator glyph
/// - Separator color
/// - Key color
/// - Amount of padding
/// - Amount of spacing
/// - Longest key of the current `Theme`
pub struct Misc {
    pub separator: &'static str,
    pub separator_color: Color,
    pub color: Color,
    pub padding: usize,
    pub spacing: usize,
    pub longest_key: String,
}

impl Misc {
    fn dash() -> Misc {
        Misc {
            color: Color::Red,
            separator_color: Color::White,
            separator: "—",
            spacing: 2,
            padding: 4,
            longest_key: String::new(),
        }
    }
    fn arrow() -> Misc {
        Misc {
            color: Color::Green,
            separator_color: Color::White,
            separator: "=>",
            spacing: 2,
            padding: 4,
            longest_key: String::new(),
        }
    }
    fn squiggly() -> Misc {
        Misc {
            color: Color::Yellow,
            separator_color: Color::White,
            separator: "~",
            spacing: 2,
            padding: 4,
            longest_key: String::new(),
        }
    }
}

/// `Bar` contains several elements that make up a `Theme`, such as the: \
/// - `Bar` glyph
/// - `Bar` opening symbol
/// - `Bar` closing symbol
pub struct Bar {
    /// This is the glyph/symbol that represents the usage.
    pub glyph: &'static str,
    /// This is used to indicate the beginning of a bar.
    pub symbol_open: char,
    /// This is used to indicate the end of a bar.
    pub symbol_close: char,
}

/// This implements all the different ways a `Bar` can look.
impl Bar {
    fn squared() -> Bar {
        Bar {
            glyph: "■",
            symbol_open: '[',
            symbol_close: ']',
        }
    }
    fn rounded() -> Bar {
        Bar {
            glyph: "●",
            symbol_open: '(',
            symbol_close: ')',
        }
    }
    fn angled() -> Bar {
        Bar {
            glyph: "×",
            symbol_open: '<',
            symbol_close: '>',
        }
    }
}

arg_enum! {
    /// This enum contains all the possible keys, e.g. _Host_, _Machine_, _Kernel_, etc.
    #[derive(Debug, PartialEq)]
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
    }
}

arg_enum! {
    #[derive(Debug, PartialEq)]
    pub enum Themes {
        Hydrogen,
        Helium,
        Lithium
    }
}

impl Themes {
    pub fn create_instance(&self) -> Box<dyn Theme> {
        match self {
            Themes::Hydrogen => HydrogenTheme::new(),
            Themes::Helium => HeliumTheme::new(),
            Themes::Lithium => LithiumTheme::new()
        }
    }
}

/// Defines the different ways a key can be named, let's take the _OperatingSystem variant_ for example: \
/// - `AbbreviationType::Classic` -> OS \
/// - `AbbreviationType::Alternative` -> Ope \
/// - `AbbreviationType::Long` -> Operating System
#[derive(Eq, PartialEq, Hash)]
pub enum AbbreviationType {
    Classic,
    Alternative,
    Long,
}

/// This implements all the different ways a `Key` can be named using \
/// the predefined variants found in the `AbbreviationType` enum.
impl ReadoutKey {
    fn get_common_name(&self) -> HashMap<&AbbreviationType, &'static str> {
        let mut values = HashMap::new();

        match self {
            ReadoutKey::Host => {
                values.insert(&AbbreviationType::Classic, "Host");
                values.insert(&AbbreviationType::Alternative, "Hos");
                values.insert(&AbbreviationType::Long, "Host");
            }
            ReadoutKey::Machine => {
                values.insert(&AbbreviationType::Classic, "Machine");
                values.insert(&AbbreviationType::Alternative, "Mac");
                values.insert(&AbbreviationType::Long, "Machine");
            }
            ReadoutKey::Kernel => {
                values.insert(&AbbreviationType::Classic, "Kernel");
                values.insert(&AbbreviationType::Alternative, "Ker");
                values.insert(&AbbreviationType::Long, "Kernel");
            }
            ReadoutKey::Distribution => {
                values.insert(&AbbreviationType::Classic, "Distro");
                values.insert(&AbbreviationType::Alternative, "Dis");
                values.insert(&AbbreviationType::Long, "Distribution");
            }
            ReadoutKey::OperatingSystem => {
                values.insert(&AbbreviationType::Classic, "OS");
                values.insert(&AbbreviationType::Alternative, "Ope");
                values.insert(&AbbreviationType::Long, "Operating System");
            }
            ReadoutKey::DesktopEnvironment => {
                values.insert(&AbbreviationType::Classic, "DE");
                values.insert(&AbbreviationType::Alternative, "Des");
                values.insert(&AbbreviationType::Long, "Desktop Environment");
            }
            ReadoutKey::WindowManager => {
                values.insert(&AbbreviationType::Classic, "WM");
                values.insert(&AbbreviationType::Alternative, "Win");
                values.insert(&AbbreviationType::Long, "Window Manager");
            }
            ReadoutKey::Packages => {
                values.insert(&AbbreviationType::Classic, "Packages");
                values.insert(&AbbreviationType::Alternative, "Pac");
                values.insert(&AbbreviationType::Long, "Packages");
            }
            ReadoutKey::Shell => {
                values.insert(&AbbreviationType::Classic, "Shell");
                values.insert(&AbbreviationType::Alternative, "She");
                values.insert(&AbbreviationType::Long, "Shell");
            }
            ReadoutKey::Terminal => {
                values.insert(&AbbreviationType::Classic, "Terminal");
                values.insert(&AbbreviationType::Alternative, "Ter");
                values.insert(&AbbreviationType::Long, "Terminal");
            }
            ReadoutKey::Uptime => {
                values.insert(&AbbreviationType::Classic, "Uptime");
                values.insert(&AbbreviationType::Alternative, "Upt");
                values.insert(&AbbreviationType::Long, "Uptime");
            }
            ReadoutKey::Processor => {
                values.insert(&AbbreviationType::Classic, "CPU");
                values.insert(&AbbreviationType::Alternative, "Cpu");
                values.insert(&AbbreviationType::Long, "Processor");
            }
            ReadoutKey::Memory => {
                values.insert(&AbbreviationType::Classic, "Memory");
                values.insert(&AbbreviationType::Alternative, "Mem");
                values.insert(&AbbreviationType::Long, "Memory");
            }
            ReadoutKey::Battery => {
                values.insert(&AbbreviationType::Classic, "Battery");
                values.insert(&AbbreviationType::Alternative, "Bat");
                values.insert(&AbbreviationType::Long, "Battery");
            }
        }

        values
    }
}

/// This trait provides the necessary functions for creating _themes_.
pub trait Theme {
    fn new() -> Box<dyn Theme>
    where
        Self: Sized;

    fn bar(&self) -> &Bar;
    fn misc(&self) -> &Misc;
    fn bar_mut(&mut self) -> &mut Bar;
    fn misc_mut(&mut self) -> &mut Misc;

    fn default_abbreviation(&self) -> &AbbreviationType;

    fn key(&self, readout_key: ReadoutKey, abbreviation: &AbbreviationType) -> &'static str {
        let abbreviated_names = readout_key.get_common_name();
        let name_entry = abbreviated_names.get(&abbreviation);

        if let Some(name) = name_entry {
            name
        } else {
            abbreviated_names.values().next().unwrap()
        }
    }

    fn get_colored_separator(&self) -> ColoredString {
        ColoredString::from(self.misc().separator).color(self.misc().separator_color)
    }

    fn key_to_colored_string(&self, readout_key: ReadoutKey) -> ColoredString {
        let key_name = self.key(readout_key, self.default_abbreviation());
        ColoredString::from(key_name)
            .color(self.misc().color)
            .bold()
    }

    fn padding(&self) -> String {
        " ".repeat(self.misc().padding)
    }

    fn spacing(&self) -> String {
        " ".repeat(self.misc().spacing)
    }
}

/// This structure's implementation utilizes the following:
/// - _Rounded_ bar through `Bar::rounded()`
/// - _Dash_ style through `Misc::dash()`
/// - _Classic_ abbreviation type through `AbbreviationType::Classic`
pub struct HydrogenTheme {
    bar: Bar,
    misc: Misc,
}

impl Theme for HydrogenTheme {
    fn new() -> Box<dyn Theme> {
        Box::new(HydrogenTheme {
            bar: Bar::rounded(),
            misc: Misc::dash(),
        })
    }

    fn bar(&self) -> &Bar {
        &self.bar
    }

    fn misc(&self) -> &Misc {
        &self.misc
    }

    fn bar_mut(&mut self) -> &mut Bar {
        &mut self.bar
    }

    fn misc_mut(&mut self) -> &mut Misc {
        &mut self.misc
    }

    fn default_abbreviation(&self) -> &AbbreviationType {
        &AbbreviationType::Classic
    }
}
/// This structure's implementation utilizes the following:
/// - _Squared_ bar through `Bar::squared()`
/// - _Arrow_ style through `Misc::arrow()`
/// - _Alternative_ abbreviation type through `AbbreviationType::Alternative`
pub struct HeliumTheme {
    bar: Bar,
    misc: Misc,
}

impl Theme for HeliumTheme {
    fn new() -> Box<dyn Theme> {
        Box::new(HeliumTheme {
            bar: Bar::squared(),
            misc: Misc::arrow(),
        })
    }

    fn bar(&self) -> &Bar {
        &self.bar
    }

    fn misc(&self) -> &Misc {
        &self.misc
    }

    fn bar_mut(&mut self) -> &mut Bar {
        &mut self.bar
    }

    fn misc_mut(&mut self) -> &mut Misc {
        &mut self.misc
    }

    fn default_abbreviation(&self) -> &AbbreviationType {
        &AbbreviationType::Alternative
    }
}

/// This structure's implementation utilizes the following:
/// - _Angled_ bar through `Bar::angled()`
/// - _Squiggly_ style through `Misc::squiggly()`
/// - _Long_ abbreviation type through `AbbreviationType::Long`
pub struct LithiumTheme {
    bar: Bar,
    misc: Misc,
}

impl Theme for LithiumTheme {
    fn new() -> Box<dyn Theme> {
        Box::new(LithiumTheme {
            bar: Bar::angled(),
            misc: Misc::squiggly(),
        })
    }

    fn bar(&self) -> &Bar {
        &self.bar
    }

    fn misc(&self) -> &Misc {
        &self.misc
    }

    fn bar_mut(&mut self) -> &mut Bar {
        &mut self.bar
    }

    fn misc_mut(&mut self) -> &mut Misc {
        &mut self.misc
    }

    fn default_abbreviation(&self) -> &AbbreviationType {
        &AbbreviationType::Long
    }
}
