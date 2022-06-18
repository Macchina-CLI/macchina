use crate::data::Readout;
use crate::theme::Theme;
use crate::widgets::readout::ReadoutList;
use atty::Stream;
use std::io;
use std::io::Stdout;
use tui::backend::{Backend, CrosstermBackend};
use tui::buffer::{Buffer, Cell};
use tui::layout::{Margin, Rect};
use tui::text::Text;
use tui::widgets::{Block, Borders, Paragraph, Widget};
use unicode_width::UnicodeWidthStr;

pub fn create_backend() -> CrosstermBackend<Stdout> {
    CrosstermBackend::new(io::stdout())
}

pub fn find_widest_cell(buf: &Buffer, last_y: u16) -> u16 {
    let area = &buf.area;
    let mut widest: u16 = 0;
    let empty_cell = Cell::default();

    for y in 0..last_y {
        for x in (0..area.width).rev() {
            let current_cell = buf.get(x, y);
            if current_cell.ne(&empty_cell) && x > widest {
                widest = x;
                break;
            }
        }
    }

    widest + 1
}

pub fn find_last_buffer_cell_index(buf: &Buffer) -> Option<(u16, u16)> {
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

pub enum Side {
    Bottom,
    Right,
    Left,
    Top,
}

pub fn draw_ascii(side: Side, ascii: Text<'static>, tmp_buffer: &mut Buffer) -> Rect {
    let mut x = 0;
    let mut y = &x;

    match side {
        Side::Bottom => x = 1,
        Side::Top => x = 1,
        Side::Left => x = 1,
        Side::Right => x = 1,
    }

    let ascii_rect = Rect {
        x: x,
        y: *y,
        width: ascii.width() as u16,
        height: ascii.height() as u16,
    };

    Paragraph::new(ascii).render(ascii_rect, tmp_buffer);
    ascii_rect
}

pub fn draw_readout_data(data: Vec<Readout>, theme: Theme, buf: &mut Buffer, area: Rect) {
    let mut list = ReadoutList::new(data, &theme);

    if theme.get_block().is_visible() {
        list = list
            .block_inner_margin(Margin {
                horizontal: theme.get_block().get_horizontal_margin(),
                vertical: theme.get_block().get_vertical_margin(),
            })
            .block(
                Block::default()
                    .border_type(theme.get_block().get_border_type())
                    .title(theme.get_block().get_title())
                    .borders(Borders::ALL),
            );
    }

    list.render(area, buf);
}

pub fn write_buffer_to_console(
    backend: &mut CrosstermBackend<Stdout>,
    tmp_buffer: &mut Buffer,
) -> Result<(), io::Error> {
    let term_size = backend.size().unwrap_or_default();

    let (_, last_y) = find_last_buffer_cell_index(tmp_buffer)
        .expect("An error occurred while writing to the terminal buffer.");

    let last_x = find_widest_cell(tmp_buffer, last_y);

    print!("{}", "\n".repeat(last_y as usize + 1));

    let mut cursor_y: u16 = 0;

    if atty::is(Stream::Stdout) {
        cursor_y = backend.get_cursor().unwrap_or((0, 0)).1;
    }

    // we need a checked subtraction here, because (cursor_y - last_y - 1) might underflow if the
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
        .filter(|(x, y, _)| *x < last_x && *x < term_size.width && *y <= last_y)
        .map(|(x, y, cell)| (x, y + starting_pos, cell));

    backend.draw(iter)?;
    Ok(())
}
