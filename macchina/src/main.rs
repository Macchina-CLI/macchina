mod bars;
mod format;
mod theme;

use clap::arg_enum;
use clap::crate_authors;
use std::io;
use structopt::StructOpt;

mod data;
mod doctor;
pub mod widgets;

use crate::theme::Theme;
use crate::widgets::readout::ReadoutList;
use data::Readout;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::buffer::{Buffer, Cell};
use tui::layout::{Margin, Rect};
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::{Block, BorderType, Borders, Paragraph, Widget};
use tui::Terminal;

pub const AUTHORS: &str = crate_authors!();
pub const ABOUT: &str = "System information fetcher";

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

    #[structopt(short = "d", long = "doctor", help = "Checks the system for failures.")]
    doctor: bool,

    #[structopt(short = "U", long = "short-uptime", help = "Shortens uptime output")]
    short_uptime: bool,

    #[structopt(short = "S", long = "short-shell", help = "Shortens shell output")]
    short_shell: bool,

    #[structopt(
    short = "t",
    long = "theme",
    default_value = "Hydrogen",
    possible_values = & theme::Themes::variants(),
    case_insensitive = true,
    help = "Specifies the theme to use"
    )]
    theme: theme::Themes,

    #[structopt(long = "no-ascii", help = "Removes the ascii art.")]
    no_ascii: bool,

    #[structopt(long = "no-box", help = "Removes the system information borders.")]
    no_box: bool,
}

fn create_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
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

const ASCII: &str = r#"         _nnnn_
        dGGGGMMb
       @p~qp~~qMb
       M|@||@) M|
       @,----.JM|
      JS^\__/  qKL
     dZP        qKRb
    dZP          qKKb
   fZP            SMMb
   HZM            MMMM
   FqM            MMMM
 __| ".        |\dS"qML
 |    `.       | `' \Zq
_)      \.___.,|     .'
\____   )MMMMMP|   .'
     `-'       `--' hjm"#;

fn draw_ascii(ascii: &str, tmp_buffer: &mut Buffer) -> Rect {
    let paragraph = Text::styled(ascii, Style::default().fg(Color::LightBlue));
    let ascii_rect = Rect {
        x: 0,
        y: 0,
        width: paragraph.width() as u16,
        height: paragraph.height() as u16,
    };

    Paragraph::new(paragraph).render(ascii_rect, tmp_buffer);
    ascii_rect
}

fn draw_readout_data(
    data: Vec<Readout>,
    theme: Box<dyn Theme>,
    buf: &mut Buffer,
    area: Rect,
    show_box: bool,
) {
    let mut list = ReadoutList::new(data, &theme);

    if show_box {
        list = list
            .block_inner_margin(Margin {
                horizontal: 1,
                vertical: 1,
            })
            .block(
                Block::default()
                    .border_type(BorderType::Rounded)
                    .title(theme.get_block_title())
                    .borders(Borders::ALL),
            );
    }

    list.render(area, buf);
}

fn main() -> Result<(), io::Error> {
    let opt = Opt::from_args();
    let readout_data = data::get_all_readouts(&opt);

    if opt.doctor {
        doctor::print_doctor(&readout_data);
        return Ok(());
    }

    let mut terminal = create_terminal()?;
    let mut tmp_buffer = Buffer::empty(Rect::new(0, 0, 300, 50));

    let ascii_area = if !opt.no_ascii {
        draw_ascii(ASCII, &mut tmp_buffer)
    } else {
        Rect::new(0, 0, 0, tmp_buffer.area.height)
    };

    let tmp_buffer_area = tmp_buffer.area;

    let theme = opt.theme.create_instance();
    let theme_padding = theme.get_padding() as u16;

    draw_readout_data(
        readout_data,
        theme,
        &mut tmp_buffer,
        Rect::new(
            ascii_area.x + ascii_area.width + theme_padding,
            ascii_area.y,
            tmp_buffer_area.width - ascii_area.width - 4,
            ascii_area.height,
        ),
        !opt.no_box,
    );

    write_buffer_to_console(&mut terminal, &mut tmp_buffer);

    terminal.flush()?;
    println!();

    Ok(())
}

fn write_buffer_to_console(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    tmp_buffer: &mut Buffer,
) {
    let (_, last_y) =
        find_last_buffer_cell_index(tmp_buffer).expect("Error while writing to terminal buffer.");

    print!("{}", "\n".repeat(last_y as usize + 1));

    let (_, cursor_y) = terminal.get_cursor().unwrap();
    let terminal_buf = terminal.current_buffer_mut();
    let term_width = terminal_buf.area.width;
    let tmp_width = tmp_buffer.area.width;

    let mut y_tmp = 0;

    // We need a checked subtraction here, because (cursor_y - last_y - 1) might underflow if the
    // cursor_y is smaller than (last_y - 1).
    let starting_pos = cursor_y
        .checked_sub(last_y)
        .unwrap_or(0)
        .checked_sub(1)
        .unwrap_or(0);

    for y in starting_pos..cursor_y {
        let start_index_term = (y * term_width) as usize;
        let end_index_term = start_index_term + term_width as usize;

        let start_index_tmp = (y_tmp * tmp_width) as usize;
        let end_index_tmp = start_index_tmp + term_width as usize;

        terminal_buf.content[start_index_term..end_index_term]
            .clone_from_slice(&tmp_buffer.content[start_index_tmp..end_index_tmp]);

        y_tmp += 1;
    }
}
