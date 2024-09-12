#![warn(clippy::all)]

mod ascii;
mod bars;
mod buffer;
mod cli;
mod config;
mod data;
mod doctor;
mod error;
mod extra;
mod format;
pub mod theme;
pub mod widgets;

use cli::{Opt, PKG_NAME};
use error::Result;
use ratatui::{backend::Backend, buffer::Buffer, layout::Rect};

fn main() -> Result<()> {
    let opt = Opt::get_options();

    if opt.version {
        get_version();
        return Ok(());
    }

    if opt.ascii_artists {
        ascii::list_ascii_artists();
        return Ok(());
    }

    if opt.list_themes {
        theme::list_themes(&opt);
        return Ok(());
    }

    let theme = theme::create_theme(&opt);
    let should_display = data::should_display(&opt);
    let readout_data = data::get_all_readouts(&opt, &theme, &should_display);

    if opt.doctor {
        doctor::print_doctor(&readout_data);
        return Ok(());
    }

    const MAX_ASCII_HEIGHT: usize = 50;
    const MINIMUM_READOUTS_TO_PREFER_SMALL_ASCII: usize = 8;
    let mut backend = buffer::create_backend();
    let mut tmp_buffer = Buffer::empty(Rect::new(0, 0, 500, 50));
    let mut ascii_area = Rect::new(0, 1, 0, tmp_buffer.area.height - 1);
    let prefers_small_ascii =
        readout_data.len() < MINIMUM_READOUTS_TO_PREFER_SMALL_ASCII || theme.prefers_small_ascii();

    if theme.is_ascii_visible() {
        if let Some(path) = theme.get_custom_ascii().get_path() {
            let expanded = shellexpand::tilde(&path.to_string_lossy()).to_string();
            let file_path = std::path::PathBuf::from(expanded);
            let ascii_art = if let Some(color) = theme.get_custom_ascii().get_color() {
                ascii::get_ascii_from_file_override_color(&file_path, color)?
            } else {
                ascii::get_ascii_from_file(&file_path)?
            };

            if ascii_art.width() != 0 && ascii_art.height() < MAX_ASCII_HEIGHT {
                ascii_area = buffer::draw_ascii(ascii_art, &mut tmp_buffer);
            }
        } else if prefers_small_ascii {
            // prefer smaller ascii in this case
            if let Some(ascii) = ascii::select_ascii(ascii::AsciiSize::Small) {
                ascii_area = buffer::draw_ascii(ascii, &mut tmp_buffer);
            }
        } else {
            // prefer bigger ascii otherwise
            if let Some(ascii) = ascii::select_ascii(ascii::AsciiSize::Big) {
                ascii_area = buffer::draw_ascii(ascii, &mut tmp_buffer);
            }
        }
    }

    let tmp_buffer_area = tmp_buffer.area;

    buffer::draw_readout_data(
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

    buffer::write_buffer_to_console(&mut backend, &mut tmp_buffer)?;

    backend.flush()?;
    print!("\n\n");

    Ok(())
}

fn get_version() {
    if let Some(git_sha) = option_env!("VERGEN_GIT_SHA_SHORT") {
        println!(
            "{}     {} ({})",
            PKG_NAME,
            env!("CARGO_PKG_VERSION"),
            git_sha
        );
    } else {
        println!("{}     {}", PKG_NAME, env!("CARGO_PKG_VERSION"));
    }

    println!("libmacchina  {}", libmacchina::version());
}
