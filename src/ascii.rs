use crate::Result;
use ansi_to_tui::IntoText;
use colored::Colorize;
use io::Read;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

const BLUE: Style = Style::new().fg(Color::Blue);
const RED: Style = Style::new().fg(Color::Red);
const GREEN: Style = Style::new().fg(Color::Green);
const YELLOW: Style = Style::new().fg(Color::Yellow);
const MAGENTA: Style = Style::new().fg(Color::Magenta);
const WHITE: Style = Style::new().fg(Color::White);
const BLACK: Style = Style::new().fg(Color::Black);

pub enum AsciiSize {
    Big,
    Small,
}

pub fn list_ascii_artists() {
    println!(
        "- FreeBSD ASCII art (small variant) was taken from {}' {}",
        "Dylan Araps".bold(),
        "pfetch".bright_purple()
    );

    println!(
        "- macOS ASCII art (big variant) was taken from {}' {}",
        "Dylan Araps".bold(),
        "Neofetch".bright_purple()
    );

    println!(
        "- macOS ASCII art (small variant) was originally made by {}",
        "Joan Stark".bold(),
    );

    println!(
        "- Linux ASCII art (big variant) was originally made by {}",
        "Joan Stark".bold(),
    );

    println!(
        "- Linux ASCII art (small variant) was taken from {}",
        "Christopher Johnson's ASCII art collection".bold(),
    );
}

pub fn select_ascii(ascii_size: AsciiSize) -> Option<Text<'static>> {
    let ascii_art = get_ascii_art(ascii_size);

    if ascii_art.is_empty() {
        return None;
    }

    Some(ascii_art[0].to_owned())
}

pub fn get_ascii_from_file(file_path: &Path) -> Result<Text<'static>> {
    let file = File::open(file_path)?;
    let mut buffer = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut buffer)?;
    Ok(buffer.into_text().unwrap_or_default())
}

pub fn get_ascii_from_file_override_color(file_path: &Path, color: Color) -> Result<Text<'static>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut buffer: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let text = buffer.into_text().unwrap_or_default();
    let text = text.patch_style(Style::default().fg(color));
    Ok(text)
}

// The following is a slightly modified
// version of neofetch's Apple ASCII art.
#[cfg(target_os = "macos")]
pub(crate) fn get_ascii_art(size: AsciiSize) -> Vec<Text<'static>> {
    match size {
        AsciiSize::Big => {
            let art: Vec<Span> = vec![
                Span::styled("                 ,MMMM.", GREEN),
                Span::styled("               .MMMMMM", GREEN),
                Span::styled("               MMMMM,", GREEN),
                Span::styled("     .;MMMMM:' MMMMMMMMMM;.", YELLOW),
                Span::styled("   MMMMMMMMMMMMNWMMMMMMMMMMM:", YELLOW),
                Span::styled(" .MMMMMMMMMMMMMMMMMMMMMMMMWM.", YELLOW),
                Span::styled(" MMMMMMMMMMMMMMMMMMMMMMMMM.", RED),
                Span::styled(";MMMMMMMMMMMMMMMMMMMMMMMM:", RED),
                Span::styled(":MMMMMMMMMMMMMMMMMMMMMMMM:", RED),
                Span::styled(".MMMMMMMMMMMMMMMMMMMMMMMMM.", MAGENTA),
                Span::styled(" MMMMMMMMMMMMMMMMMMMMMMMMMMM.", MAGENTA),
                Span::styled("  .MMMMMMMMMMMMMMMMMMMMMMMMMM.", MAGENTA),
                Span::styled("    MMMMMMMMMMMMMMMMMMMMMMMM", BLUE),
                Span::styled("     ;MMMMMMMMMMMMMMMMMMMM.", BLUE),
                Span::styled("       .MMMM,.    .MMMM,.", BLUE),
            ];

            vec![Text::from(
                art.iter()
                    .map(|f| Line::from(f.to_owned()))
                    .collect::<Vec<Line>>(),
            )]
        }
        AsciiSize::Small => {
            // The following Apple ASCII art was made by Joan Stark (jgs)
            let art: Vec<Span> = vec![
                Span::styled("        .:'", GREEN),
                Span::styled("    __ :'__", GREEN),
                Span::styled(" .'`  `-'  ``.", YELLOW),
                Span::styled(":          .-'", YELLOW),
                Span::styled(":         :", RED),
                Span::styled(" :         `-;", RED),
                Span::styled("  `.__.-.__.'", MAGENTA),
            ];

            vec![Text::from(
                art.iter()
                    .map(|f| Line::from(f.to_owned()))
                    .collect::<Vec<Line>>(),
            )]
        }
    }
}

#[cfg(target_os = "android")]
pub(crate) fn get_ascii_art(size: AsciiSize) -> Vec<Text<'static>> {
    match size {
        AsciiSize::Big => {
            let art: Vec<Span> = vec![
                Span::styled("    .        .", GREEN),
                Span::styled("     \\      /", GREEN),
                Span::styled("    .oooooooo.", GREEN),
                Span::styled("   .oooooooooo. ", GREEN),
                Span::styled("   ooo  oo  ooo", GREEN),
                Span::styled("   oooooooooooo", GREEN),
                Span::styled("   ____________", GREEN),
                Span::styled("oo oooooooooooo oo", GREEN),
                Span::styled("oo oooooooooooo oo", GREEN),
                Span::styled("oo oooooooooooo oo", GREEN),
                Span::styled("   oooooooooooo", GREEN),
                Span::styled("     ooo   ooo", GREEN),
                Span::styled("     ooo   ooo", GREEN),
            ];

            vec![Text::from(
                art.iter()
                    .map(|f| Line::from(f.to_owned()))
                    .collect::<Vec<Line>>(),
            )]
        }
        AsciiSize::Small => {
            let art: Vec<Span> = vec![
                Span::styled("  .        .   ", GREEN),
                Span::styled("   \\      /   ", GREEN),
                Span::styled("  .oooooooo.   ", GREEN),
                Span::styled(" .oooooooooo.  ", GREEN),
                Span::styled(" ooo  oo  ooo  ", GREEN),
                Span::styled(" oooooooooooo  ", GREEN),
            ];

            vec![Text::from(
                art.iter()
                    .map(|f| Line::from(f.to_owned()))
                    .collect::<Vec<Line>>(),
            )]
        }
    }
}

#[cfg(target_os = "windows")]
pub(crate) fn get_ascii_art(size: AsciiSize) -> Vec<Text<'static>> {
    match size {
        AsciiSize::Big => {
            let art: Vec<Span> = vec![
                Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, BLUE),
                Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, BLUE),
                Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, BLUE),
                Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, BLUE),
                Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, BLUE),
                Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, BLUE),
                Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, BLUE),
                Span::raw(r#" "#),
                Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, BLUE),
                Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, BLUE),
                Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, BLUE),
                Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, BLUE),
                Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, BLUE),
                Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, BLUE),
                Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, BLUE),
                Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, BLUE),
            ];

            vec![Text::from(
                art.iter()
                    .map(|f| Line::from(f.to_owned()))
                    .collect::<Vec<Line>>(),
            )]
        }
        AsciiSize::Small => {
            let art: Vec<Span> = vec![
                Span::styled("wwww  wwww", BLUE),
                Span::styled("wwww  wwww", BLUE),
                Span::styled("wwww  wwww", BLUE),
                Span::raw(r#" "#),
                Span::styled("wwww  wwww", BLUE),
                Span::styled("wwww  wwww", BLUE),
                Span::styled("wwww  wwww", BLUE),
            ];

            vec![Text::from(
                art.iter()
                    .map(|f| Line::from(f.to_owned()))
                    .collect::<Vec<Line>>(),
            )]
        }
    }
}

// The following penguin ASCII art was made by Joan Stark (jgs)
#[cfg(target_os = "linux")]
pub(crate) fn get_ascii_art(size: AsciiSize) -> Vec<Text<'static>> {
    match size {
        AsciiSize::Big => {
            let art: Vec<Line> = vec![
                Line::from(vec![Span::styled("       a8888b.", WHITE)]),
                Line::from(vec![Span::styled("      d888888b.", WHITE)]),
                Line::from(vec![Span::styled("      8P\"YP\"Y88", WHITE)]),
                Line::from(vec![Span::styled("      8|o||o|88", WHITE)]),
                Line::from(vec![
                    Span::styled("      8", WHITE),
                    Span::styled("'    .", YELLOW),
                    Span::styled("88", WHITE),
                ]),
                Line::from(vec![
                    Span::styled("      8", WHITE),
                    Span::styled("`._.'", YELLOW),
                    Span::styled(" Y8.", WHITE),
                ]),
                Line::from(vec![Span::styled("     d/      `8b.", WHITE)]),
                Line::from(vec![Span::styled("    dP        Y8b.", WHITE)]),
                Line::from(vec![Span::styled("   d8:       ::88b.", WHITE)]),
                Line::from(vec![Span::styled("  d8\"         'Y88b", WHITE)]),
                Line::from(vec![Span::styled(" :8P           :888", WHITE)]),
                Line::from(vec![Span::styled("  8a.         _a88P", WHITE)]),
                Line::from(vec![
                    Span::styled("._/\"Y", YELLOW),
                    Span::styled("aa     .", WHITE),
                    Span::styled("|", YELLOW),
                    Span::styled(" 88P", WHITE),
                    Span::styled("|", YELLOW),
                ]),
                Line::from(vec![
                    Span::styled("\\    Y", YELLOW),
                    Span::styled("P\"    `", WHITE),
                    Span::styled("|     `.", YELLOW),
                ]),
                Line::from(vec![
                    Span::styled("/     \\", YELLOW),
                    Span::styled(".___.d", WHITE),
                    Span::styled("|    .'", YELLOW),
                ]),
                Line::from(vec![Span::styled("`--..__)     `._.'", YELLOW)]),
            ];

            vec![Text::from(
                art.iter().map(|f| f.to_owned()).collect::<Vec<Line>>(),
            )]
        }
        AsciiSize::Small => {
            // The following penguin ASCII art was found and
            // taken from: https://asciiart.website/index.php
            // Artist attribution missing.
            // Thank you to whoever made it :^)
            let art: Vec<Line> = vec![
                Line::from(vec![Span::styled("    .--.", WHITE)]),
                Line::from(vec![
                    Span::styled("   |o", WHITE),
                    Span::styled("_", YELLOW),
                    Span::styled("o |", WHITE),
                ]),
                Line::from(vec![
                    Span::styled("   |", WHITE),
                    Span::styled("\\_/", YELLOW),
                    Span::styled(" |", WHITE),
                ]),
                Line::from(vec![Span::styled("  //   \\ \\", WHITE)]),
                Line::from(vec![Span::styled(" (|     | )", WHITE)]),
                Line::from(vec![Span::styled("/'\\_   _/`\\", WHITE)]),
                Line::from(vec![Span::styled("\\___)=(___/", WHITE)]),
            ];

            vec![Text::from(art)]
        }
    }
}

#[cfg(target_os = "freebsd")]
pub(crate) fn get_ascii_art(size: AsciiSize) -> Vec<Text<'static>> {
    // The following ASCII art was made by Dylan Araps
    // and taken from https://github.com/dylanaraps/pfetch

    let art: Vec<Line> = vec![
        Line::from(vec![Span::styled("/\\,-'''''-,/\\", RED)]),
        Line::from(vec![Span::styled("\\_)       (_//", RED)]),
        Line::from(vec![Span::styled(" |           |", RED)]),
        Line::from(vec![Span::styled(" |           |", RED)]),
        Line::from(vec![Span::styled("  ;         ;", RED)]),
        Line::from(vec![Span::styled("   '-_____-'", RED)]),
    ];

    vec![Text::from(art)]
}

#[cfg(target_os = "netbsd")]
pub(crate) fn get_ascii_art(size: AsciiSize) -> Vec<Text<'static>> {
    match size {
        AsciiSize::Big => {
            let art: Vec<Line> = vec![
                Line::from(vec![
                    Span::styled("\\\\", WHITE),
                    Span::styled("`-______,----__", YELLOW),
                ]),
                Line::from(vec![
                    Span::styled(" \\\\", WHITE),
                    Span::styled("        __,---`.", YELLOW),
                ]),
                Line::from(vec![
                    Span::styled("  \\\\", WHITE),
                    Span::styled("       `.____", YELLOW),
                ]),
                Line::from(vec![
                    Span::styled("   \\\\", WHITE),
                    Span::styled("-______,----`.", YELLOW),
                ]),
                Line::from(vec![Span::styled("    \\\\", WHITE)]),
                Line::from(vec![Span::styled("     \\\\", WHITE)]),
                Line::from(vec![Span::styled("      \\\\", WHITE)]),
                Line::from(vec![Span::styled("       \\\\", WHITE)]),
                Line::from(vec![Span::styled("        \\\\", WHITE)]),
                Line::from(vec![Span::styled("         \\\\", WHITE)]),
                Line::from(vec![Span::styled("          \\\\", WHITE)]),
            ];

            vec![Text::from(
                art.iter().map(|f| f.to_owned()).collect::<Vec<Line>>(),
            )]
        }
        AsciiSize::Small => {
            let art: Vec<Line> = vec![
                Line::from(vec![
                    Span::styled("()", BLACK),
                    Span::styled("ncncncncncnc", YELLOW),
                ]),
                Line::from(vec![
                    Span::styled(" \\\\", BLACK),
                    Span::styled("ncncncnc", YELLOW),
                ]),
                Line::from(vec![
                    Span::styled("  \\\\", BLACK),
                    Span::styled("ncncncncncn", YELLOW),
                ]),
                Line::from(vec![Span::styled("   \\\\", BLACK)]),
                Line::from(vec![Span::styled("    \\\\", BLACK)]),
                Line::from(vec![Span::styled("     \\\\", BLACK)]),
            ];

            vec![Text::from(art)]
        }
    }
}
