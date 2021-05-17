mod bars;
mod cli;
mod config;
mod format;
mod theme;

use cli::{MacchinaColor, Opt};
use colored::Colorize;
use std::io;
use structopt::StructOpt;
use theme::Themes;

#[macro_use]
extern crate lazy_static;

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
use std::path::PathBuf;
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

fn draw_readout_data(data: Vec<Readout>, theme: Theme, buf: &mut Buffer, area: Rect, config: &Opt) {
    let mut list = ReadoutList::new(data, &theme).palette(config.palette);

    if !config.no_box {
        list = list
            .block_inner_margin(Margin {
                horizontal: config.box_inner_margin_x,
                vertical: config.box_inner_margin_y,
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

fn create_theme(opt: &Opt) -> Theme {
    let mut theme;
    if let Some(opt_theme) = &opt.theme {
        if let Ok(ts) = theme::Themes::from_str(opt_theme) {
            theme = Theme::new(ts);
        } else if let Ok(custom_theme) = theme::CustomTheme::get_theme(opt_theme) {
            theme = Theme::from(custom_theme);
        } else {
            println!(
                "\x1b[33mWarning:\x1b[0m Invalid theme {}, falling back to default",
                opt_theme
            );
            theme = Theme::default();
        }
    } else {
        theme = Theme::default();
    }
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

fn list_themes() {
    let themes = Themes::variants();
    themes
        .iter()
        .for_each(|x| println!("• {} (Built-in)", x.bright_green()));

    if let Some(dir) = dirs::data_local_dir() {
        let entries = libmacchina::extra::list_dir_entries(&dir.join("macchina/themes"));
        if !entries.is_empty() {
            let custom_themes = entries.iter().filter(|&x| {
                if let Some(ext) = libmacchina::extra::path_extension(&x) {
                    ext == "json"
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
                    let name = theme.to_string_lossy().replace(".json", "");
                    println!("• {}", name.bright_blue());
                }
            });
        }
    }
}

fn main() -> Result<(), io::Error> {
    let mut opt: Opt;
    let config_opt = Opt::from_config();
    let arg_opt = Opt::from_args();

    if arg_opt.export_config {
        println!("{}", toml::to_string(&arg_opt).unwrap());
        return Ok(());
    }

    if let Ok(mut config_opt) = config_opt {
        config_opt.patch_args(Opt::from_args());
        opt = config_opt;
        let conflicts = opt.check_conflicts();
        if !conflicts.is_empty() {
            println!("\x1b[33mWarning:\x1b[0m Conflicting keys in config file:");
            for conflict in conflicts {
                println!("• {}", conflict);
            }
            opt = arg_opt;
        }
    } else {
        println!("\x1b[33mWarning:\x1b[0m Invalid config file");
        opt = arg_opt;
    }

    let should_display = should_display(&opt);
    let theme = create_theme(&opt);
    let readout_data = data::get_all_readouts(&opt, &theme, should_display);

    if opt.doctor {
        doctor::print_doctor(&readout_data);
        return Ok(());
    }

    if opt.list_themes {
        list_themes();
        return Ok(());
    }

    let mut backend = create_backend();
    let mut tmp_buffer = Buffer::empty(Rect::new(0, 0, 500, 50));

    let ascii_area;

    if let Some(ref file_path) = opt.custom_ascii {
        let file_path = PathBuf::from(file_path);
        let ascii_art;
        match opt.custom_ascii_color {
            Some(ref color) => {
                ascii_art = ascii::get_ascii_from_file_override_color(
                    &file_path,
                    color.get_color().to_owned(),
                )?;
            }

            None => {
                ascii_art = ascii::get_ascii_from_file(&file_path)?;
            }
        };

        // If the file is empty just default to disabled
        if ascii_art.width() != 0 && ascii_art.height() < 50 && !opt.small_ascii {
            // because tmp_buffer height is 50
            ascii_area = draw_ascii(ascii_art.to_owned(), &mut tmp_buffer);
        } else {
            ascii_area = Rect::new(0, 1, 0, tmp_buffer.area.height - 1);
        }
    } else if readout_data.len() <= 6 || opt.small_ascii {
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
        &opt,
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
