mod bars;
mod format;
mod theme;

use clap::arg_enum;
use clap::crate_authors;
use macchina_read::Readouts;
use std::io;
use structopt::StructOpt;

mod data;
pub mod widgets;

#[macro_use]
extern crate lazy_static;

use crate::data::ReadoutKey;
use crate::theme::LithiumTheme;
use crate::theme::Themes::{EmojiTheme, Lithium};
use data::Readout;
use macchina_read::traits::*;
use std::ops::Deref;
use std::thread::current;
use tui::backend::{Backend, CrosstermBackend};
use tui::buffer::{Buffer, Cell};
use tui::layout::{Margin, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::Text;
use tui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget};
use tui::Terminal;

pub const AUTHORS: &str = crate_authors!();
pub const ABOUT: &str = "System information fetcher";

lazy_static! {
    pub(crate) static ref READOUTS: Readouts = Readouts {
        battery: macchina_read::BatteryReadout::new(),
        kernel: macchina_read::KernelReadout::new(),
        memory: macchina_read::MemoryReadout::new(),
        general: macchina_read::GeneralReadout::new(),
        product: macchina_read::ProductReadout::new(),
        packages: macchina_read::PackageReadout::new()
    };
}

arg_enum! {
    #[derive(Debug)]
    pub enum MacchinaColor {
        Red,
        Green,
        Blue,
        Yellow,
        Cyan,
        Magenta,
        Black,
        White
    }
}

impl MacchinaColor {
    /// Convert arguments passed to `--color` to their respective color.
    fn get_color(&self) -> Color {
        match self {
            MacchinaColor::Red => Color::Red,
            MacchinaColor::Green => Color::Green,
            MacchinaColor::Blue => Color::Blue,
            MacchinaColor::Yellow => Color::Yellow,
            MacchinaColor::Cyan => Color::Cyan,
            MacchinaColor::Magenta => Color::Magenta,
            MacchinaColor::Black => Color::Black,
            MacchinaColor::White => Color::White,
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(author = AUTHORS, about = ABOUT)]
pub struct Opt {
    #[structopt(short = "p", long = "palette", help = "Displays color palette")]
    palette: bool,

    #[structopt(
        short = "P",
        long = "padding",
        default_value = "4",
        help = "Specifies the amount of left padding to use"
    )]
    padding: usize,

    #[structopt(
        short = "s",
        long = "spacing",
        help = "Specifies the amount of spacing to use"
    )]
    spacing: Option<usize>,

    #[structopt(short = "n", long = "no-color", help = "Disables color")]
    no_color: bool,

    #[structopt(
    short = "c",
    long = "color",
    possible_values = & MacchinaColor::variants(),
    case_insensitive = true,
    default_value = "Blue",
    help = "Specifies the key color"
    )]
    color: MacchinaColor,

    #[structopt(
        short = "b",
        long = "bar",
        help = "Displays bars instead of numerical values"
    )]
    bar: bool,

    #[structopt(
    short = "C",
    long = "separator-color",
    possible_values = & MacchinaColor::variants(),
    case_insensitive = true,
    default_value = "White",
    help = "Specifies the separator color"
    )]
    separator_color: MacchinaColor,

    #[structopt(
        short = "r",
        long = "random-color",
        help = "Picks a random key color for you"
    )]
    random_color: bool,

    #[structopt(
        short = "R",
        long = "random-sep-color",
        help = "Picks a random separator color for you"
    )]
    random_sep_color: bool,

    #[structopt(
    short = "H",
    long = "hide",
    possible_values = & data::ReadoutKey::variants(),
    case_insensitive = true,
    help = "Hides the specified elements",
    min_values = 1
    )]
    hide: Option<Vec<data::ReadoutKey>>,

    #[structopt(
    short = "X",
    long = "show-only",
    possible_values = & data::ReadoutKey::variants(),
    case_insensitive = true,
    help = " Displays only the specified elements",
    min_values = 1
    )]
    show_only: Option<Vec<data::ReadoutKey>>,

    #[structopt(short = "d", long = "debug", help = "Prints debug information")]
    debug: bool,

    #[structopt(short = "U", long = "short-uptime", help = "Shortens uptime output")]
    short_uptime: bool,

    #[structopt(short = "S", long = "short-shell", help = "Shortens shell output")]
    short_shell: bool,

    #[structopt(
    short = "t",
    long = "theme",
    default_value = "Hydrogen",
    possible_values = & theme::Themes::variants(),
    help = "Specifies the theme to use"
    )]
    theme: theme::Themes,
}

fn create_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

fn find_last_buffer_cell_index(buf: &Buffer) -> Option<(u16, u16)> {
    let empty_cell = Cell::default();

    if let Some((idx, _)) = buf
        .content
        .iter()
        .enumerate()
        .filter(|p| !(*(p.1)).eq(&empty_cell))
        .last()
    {
        return Some(buf.pos_of(idx));
    }

    None
}

const ASCII: &'static str = r#"        .n.                     |
       /___\          _.---.  \ _ /
       [|||]         (_._ ) )--;_) =-
       [___]           '---'.__,' \
       }-=-{                    |
       |-" |
       |.-"|                p
~^=~^~-|_.-|~^-~^~ ~^~ -^~^~|\ ~^-~^~-
^   .=.| _.|__  ^       ~  /| \
 ~ /:. \" _|_/\    ~      /_|__\  ^
.-/::.  |   |""|-._    ^   ~~~~
  `===-'-----'""`  '-.              ~
                 __.-'      ^"#;

fn main() -> Result<(), io::Error> {
    let opt = Opt::from_args();
    let mut terminal = create_terminal()?;

    let readout_data = vec![
        Readout::new(ReadoutKey::DesktopEnvironment, "Apple Windows"),
        Readout::new(ReadoutKey::Uptime, "10h 5m 3s"),
        Readout::new(ReadoutKey::Processor, "Intel Core\nProcessor"),
        Readout::new(ReadoutKey::OperatingSystem, "Apple Windows 123"),
        Readout::new(ReadoutKey::Terminal, "iTerm2"),
    ];

    use crate::theme::{EmojiTheme, Theme};
    use crate::widgets::readout::ReadoutList;

    let list = ReadoutList::new(readout_data, EmojiTheme::new())
        .block_inner_margin(Margin {
            horizontal: 1,
            vertical: 1,
        })
        .block(
            Block::default()
                .border_type(BorderType::Rounded)
                .title("🍻 Yeet")
                .borders(Borders::ALL),
        );

    let mut tmp_buffer = Buffer::empty(terminal.current_buffer_mut().area);

    let paragraph = Text::raw(ASCII);
    let ascii_rect = Rect {
        x: 0,
        y: 0,
        width: paragraph.width() as u16,
        height: paragraph.height() as u16,
    };

    Paragraph::new(paragraph).render(ascii_rect, &mut tmp_buffer);

    list.render(
        Rect {
            x: ascii_rect.x + ascii_rect.width + 4,
            y: ascii_rect.y,
            width: tmp_buffer.area.width - ascii_rect.width - 4,
            height: ascii_rect.height,
        },
        &mut tmp_buffer,
    );

    let (last_x, last_y) = find_last_buffer_cell_index(&mut tmp_buffer).expect(
        "Error while writing to terminal buffer\
        .",
    );

    print!("{}", "\n".repeat(last_y as usize));

    let mut current_buffer = terminal.current_buffer_mut();
    let buffer_start_index = current_buffer.index_of(0, current_buffer.area.height - last_y - 1);
    let tmp_buffer_index = tmp_buffer.index_of(last_x, last_y);
    let tmp_buffer_slice = &tmp_buffer.content[..=tmp_buffer_index];

    current_buffer.content[buffer_start_index..(buffer_start_index + tmp_buffer_slice.len())]
        .clone_from_slice(tmp_buffer_slice);

    terminal.flush();

    println!();

    Ok(())
}
