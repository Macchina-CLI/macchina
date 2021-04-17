mod bars;
mod cli;
mod format;
mod theme;

use cli::{MacchinaColor, Opt};
use std::io;
use structopt::StructOpt;
#[macro_use]
extern crate lazy_static;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod ascii;
mod data;
mod doctor;
pub mod widgets;

use crate::data::ReadoutKey;
use crate::theme::Theme;
use crate::widgets::readout::ReadoutList;
use atty::Stream;
use data::Readout;
use rand::Rng;
use std::io::Stdout;
use std::str::FromStr;
use tui::backend::{Backend, CrosstermBackend};
use tui::buffer::{Buffer, Cell};
use tui::layout::{Margin, Rect};
use tui::style::Color;
use tui::text::Text;
use tui::widgets::{Block, BorderType, Borders, Paragraph, Widget};
use unicode_width::UnicodeWidthStr;

fn create_backend() -> CrosstermBackend<Stdout> {
    CrosstermBackend::new(io::stdout())
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

fn draw_ascii(ascii: Text<'static>, tmp_buffer: &mut Buffer) -> Rect {
    let ascii_rect = Rect {
        x: 1,
        y: 1,
        width: ascii.width() as u16,
        height: ascii.height() as u16,
    };

    Paragraph::new(ascii).render(ascii_rect, tmp_buffer);
    ascii_rect
}

fn draw_readout_data(
    data: Vec<Readout>,
    theme: Box<dyn Theme>,
    buf: &mut Buffer,
    area: Rect,
    show_box: bool,
    palette: bool,
) {
    let mut list = ReadoutList::new(data, &theme).palette(palette);

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

fn create_theme(opt: &Opt) -> Box<dyn Theme> {
    let mut theme = opt.theme.create_instance();
    let color_variants = MacchinaColor::variants();
    let make_random_color = || {
        let mut random = rand::thread_rng();
        MacchinaColor::from_str(color_variants[random.gen_range(0..color_variants.len())])
            .unwrap()
            .get_color()
    };

    if let Some(padding) = opt.padding {
        theme.set_padding(padding);
    }

    if let Some(spacing) = opt.spacing {
        theme.set_spacing(spacing);
    }

    if let Some(color) = &opt.color {
        theme.set_color(color.get_color());
    }

    if let Some(separator_color) = &opt.separator_color {
        theme.set_separator_color(separator_color.get_color());
    }

    if let Some(box_title) = &opt.box_title {
        theme.set_block_title(&box_title[..]);
    }

    if opt.no_title {
        theme.set_block_title("");
    }

    if opt.no_separator {
        theme.set_separator("");
    }

    if opt.no_bar_delimiter {
        let new_bar = theme.get_bar_style().hide_delimiters();
        theme.set_bar_style(new_bar);
    }

    if opt.random_color {
        theme.set_color(make_random_color());
    }

    if opt.random_sep_color {
        theme.set_separator_color(make_random_color());
    }

    if opt.no_color {
        theme.set_separator_color(Color::White);
        theme.set_color(Color::White);
    }

    theme
}

fn should_display(opt: &Opt) -> Vec<ReadoutKey> {
    if let Some(show_only) = opt.show_only.to_owned() {
        return show_only;
    }

    let mut keys: Vec<ReadoutKey> = ReadoutKey::variants()
        .iter()
        .map(|f| ReadoutKey::from_str(f).unwrap())
        .collect();
    if let Some(hide) = opt.hide.to_owned() {
        keys.retain(|f| !hide.contains(f));
    }

    keys
}

fn select_ascii(small: bool) -> Option<Text<'static>> {
    let ascii_art = ascii::get_ascii_art(small);

    if !ascii_art.is_empty() {
        Some(ascii_art[0].to_owned())
    } else {
        None
    }
}

fn main() -> Result<(), io::Error> {
    let opt = Opt::from_args();
    let should_display = should_display(&opt);
    let theme = create_theme(&opt);
    let readout_data = data::get_all_readouts(&opt, &theme, should_display);

    if opt.doctor {
        doctor::print_doctor(&readout_data);
        return Ok(());
    }

    let mut backend = create_backend();
    let mut tmp_buffer = Buffer::empty(Rect::new(0, 0, 500, 50));

    let ascii_area;

    if readout_data.len() <= 6 {
        ascii_area = match (opt.no_ascii, select_ascii(true)) {
            (false, Some(ascii)) => draw_ascii(ascii.to_owned(), &mut tmp_buffer),
            _ => Rect::new(0, 1, 0, tmp_buffer.area.height - 1),
        };
    } else {
        ascii_area = match (opt.no_ascii, select_ascii(false)) {
            (false, Some(ascii)) => draw_ascii(ascii.to_owned(), &mut tmp_buffer),
            _ => Rect::new(0, 1, 0, tmp_buffer.area.height - 1),
        };
    }

    let tmp_buffer_area = tmp_buffer.area;

    draw_readout_data(
        readout_data,
        theme,
        &mut tmp_buffer,
        Rect::new(
            ascii_area.x + ascii_area.width + 2,
            ascii_area.y,
            tmp_buffer_area.width - ascii_area.width - 4,
            ascii_area.height,
        ),
        !opt.no_box,
        opt.palette,
    );

    write_buffer_to_console(&mut backend, &mut tmp_buffer)?;

    backend.flush()?;
    print!("\n\n");

    Ok(())
}

fn write_buffer_to_console(
    backend: &mut CrosstermBackend<Stdout>,
    tmp_buffer: &mut Buffer,
) -> Result<(), io::Error> {
    let (_, last_y) =
        find_last_buffer_cell_index(tmp_buffer).expect("Error while writing to terminal buffer.");

    print!("{}", "\n".repeat(last_y as usize + 1));

    let mut cursor_y: u16 = 0;
    if atty::is(Stream::Stdout) {
        cursor_y = backend.get_cursor().unwrap_or((0, 0)).1;
    }

    let term_size = backend.size().unwrap_or_default();
    // We need a checked subtraction here, because (cursor_y - last_y - 1) might underflow if the
    // cursor_y is smaller than (last_y - 1).
    let starting_pos = cursor_y.saturating_sub(last_y).saturating_sub(1);
    let mut skip_n = 0;

    let iter = tmp_buffer
        .content
        .iter()
        .enumerate()
        .filter(|(_previous, cell)| {
            let curr_width = cell.symbol.width();
            if curr_width == 0 {
                return false;
            }

            let old_skip = skip_n;
            skip_n = curr_width.saturating_sub(1);
            old_skip == 0
        })
        .map(|(idx, cell)| {
            let (x, y) = tmp_buffer.pos_of(idx);
            (x, y, cell)
        })
        .filter(|(x, y, _)| *x < term_size.width && *y <= last_y)
        .map(|(x, y, cell)| (x, y + starting_pos, cell));

    backend.draw(iter)?;
    Ok(())
}
