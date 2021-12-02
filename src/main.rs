#![warn(clippy::all)]
mod bars;
mod cli;
mod config;
mod error;
mod extra;
mod format;
pub mod theme;

use cli::Opt;
use colored::Colorize;
use std::{array, io};
use structopt::StructOpt;

#[macro_use]
extern crate lazy_static;

mod ascii;
mod data;
mod doctor;
pub mod widgets;

use crate::data::ReadoutKey;
use crate::theme::color::make_random_color;
use crate::theme::Theme;
use crate::widgets::readout::ReadoutList;
use atty::Stream;
use data::Readout;
use error::Result;
use std::io::Stdout;
use std::str::FromStr;
use tui::backend::{Backend, CrosstermBackend};
use tui::buffer::{Buffer, Cell};
use tui::layout::{Margin, Rect};
use tui::text::Text;
use tui::widgets::{Block, Borders, Paragraph, Widget};
use unicode_width::UnicodeWidthStr;

fn create_backend() -> CrosstermBackend<Stdout> {
    CrosstermBackend::new(io::stdout())
}

fn find_widest_cell(buf: &Buffer, last_y: u16) -> u16 {
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

fn draw_readout_data(data: Vec<Readout>, theme: Theme, buf: &mut Buffer, area: Rect) {
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
                    .title(theme.get_block().get_title().unwrap_or_default())
                    .borders(Borders::ALL),
            );
    }

    list.render(area, buf);
}

fn set_theme_properties(theme: &mut Theme) {
    if theme.get_randomization().is_key_color_randomized() {
        theme.set_key_color(make_random_color());
    }

    if theme.get_randomization().is_separator_color_randomized() {
        theme.set_separator_color(make_random_color());
    }

    if theme.get_bar().are_delimiters_hidden() {
        theme.get_bar().to_owned().hide_delimiters();
    }
}

fn create_theme(opt: &Opt) -> Theme {
    let mut theme = Theme::default();
    let mut found = false;
    let locations = array::IntoIter::new(extra::config_data_paths()).flatten();
    if let Some(opt_theme) = &opt.theme {
        for dir in locations {
            if let Ok(custom_theme) = Theme::get_theme(opt_theme, dir) {
                found = true;
                theme = custom_theme;
                set_theme_properties(&mut theme);
            }
        }

        if !found {
            println!(
                "\x1b[31mError\x1b[0m: Could not find \"{}\" in any of the default directories.",
                opt_theme
            );
        }
    }

    theme
}

fn should_display(opt: &Opt) -> Vec<ReadoutKey> {
    if let Some(shown) = opt.show.to_owned() {
        return shown;
    }

    let keys: Vec<ReadoutKey> = ReadoutKey::variants()
        .iter()
        .map(|f| ReadoutKey::from_str(f).unwrap())
        .collect();

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

fn list_themes() {
    let locations = array::IntoIter::new(extra::config_data_paths()).flatten();
    for dir in locations {
        let entries = libmacchina::extra::list_dir_entries(&dir.join("macchina/themes"));
        if entries.is_empty() {
            continue;
        }

        let custom_themes = entries.iter().filter(|&x| {
            if let Some(ext) = libmacchina::extra::path_extension(x) {
                ext == "toml"
            } else {
                false
            }
        });

        if custom_themes.clone().count() == 0 {
            println!(
                "\nNo custom themes were found in {}",
                dir.join("macchina/themes")
                    .to_string_lossy()
                    .bright_yellow()
            )
        }

        custom_themes.for_each(|x| {
            if let Some(theme) = x.file_name() {
                let name = theme.to_string_lossy().replace(".toml", "");
                println!(
                    "- {} ({}/macchina/themes)",
                    name.bright_green(),
                    &dir.to_string_lossy()
                );
            }
        });
    }
}

fn main() -> Result<()> {
    let arg_opt = Opt::from_args();

    if arg_opt.export_config {
        println!("{}", toml::to_string(&arg_opt).unwrap());
        return Ok(());
    }

    let config_opt = match arg_opt.config {
        Some(_) => Opt::read_config(&arg_opt.config.clone().unwrap()),
        None => Opt::get_config(),
    };

    let opt = match config_opt {
        Ok(mut config) => {
            config.patch_args(Opt::from_args());
            config
        }
        Err(e) => {
            match e {
                error::Error::ParsingError(e) => match e.line_col() {
                    Some((l, c)) => {
                        println!(
                            "\x1b[31mError\x1b[0m: At line {} column {}\nCaused by: {}",
                            l, c, e
                        )
                    }
                    None => println!("\x1b[31mError\x1b[0m: {:?}", e),
                },
                error::Error::IOError(e) => {
                    println!("\x1b[31mError\x1b[0m: {:?}", e);
                }
            }
            arg_opt
        }
    };

    if opt.version {
        if let Some(git_sha) = option_env!("VERGEN_GIT_SHA_SHORT") {
            println!("macchina     {} ({})", env!("CARGO_PKG_VERSION"), git_sha);
        } else {
            println!("macchina     {}", env!("CARGO_PKG_VERSION"));
        }
        println!("libmacchina  {}", libmacchina::version());
        return Ok(());
    }

    if opt.list_themes {
        list_themes();
        return Ok(());
    }

    if opt.ascii_artists {
        ascii::list_ascii_artists();
        return Ok(());
    }

    let theme = create_theme(&opt);
    let should_display = should_display(&opt);
    let readout_data = data::get_all_readouts(&opt, &theme, should_display);

    if opt.doctor {
        doctor::print_doctor(&readout_data);
        return Ok(());
    }

    let mut backend = create_backend();
    let mut tmp_buffer = Buffer::empty(Rect::new(0, 0, 500, 50));

    let ascii_area;

    if let Some(ref file_path) = theme.get_custom_ascii().get_path() {
        let file_path = extra::expand_home(file_path).expect("Failed to expand ~ to HOME");
        let ascii_art;

        if let Some(color) = theme.get_custom_ascii().get_color() {
            ascii_art = ascii::get_ascii_from_file_override_color(&file_path, color)?;
        } else {
            ascii_art = ascii::get_ascii_from_file(&file_path)?;
        }

        // if the file is empty just default to disabled
        if ascii_art.width() != 0 && ascii_art.height() < 50 && !theme.is_ascii_hidden() {
            // because tmp_buffer height is 50
            ascii_area = draw_ascii(ascii_art.to_owned(), &mut tmp_buffer);
        } else {
            ascii_area = Rect::new(0, 1, 0, tmp_buffer.area.height - 1);
        }
    } else if readout_data.len() <= 6 || theme.prefers_small_ascii() {
        // prefer smaller ascii if condition is satisfied
        ascii_area = match (theme.is_ascii_hidden(), select_ascii(true)) {
            (false, Some(ascii)) => draw_ascii(ascii.to_owned(), &mut tmp_buffer),
            _ => Rect::new(0, 1, 0, tmp_buffer.area.height - 1),
        };
    } else {
        // prefer bigger ascii
        ascii_area = match (theme.is_ascii_hidden(), select_ascii(false)) {
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
    );

    write_buffer_to_console(&mut backend, &mut tmp_buffer)?;

    backend.flush()?;
    print!("\n\n");

    Ok(())
}

fn write_buffer_to_console(
    backend: &mut CrosstermBackend<Stdout>,
    tmp_buffer: &mut Buffer,
) -> Result<()> {
    let term_size = backend.size().unwrap_or_default();

    let (_, last_y) =
        find_last_buffer_cell_index(tmp_buffer).expect("Error while writing to terminal buffer.");

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
