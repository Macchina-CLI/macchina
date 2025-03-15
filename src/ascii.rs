use crate::Result;
use ansi_to_tui::IntoText;
use colored::Colorize;
use io::Read;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

lazy_static! {
    static ref BLUE: Style = Style::default().fg(Color::Blue);
    static ref RED: Style = Style::default().fg(Color::Red);
    static ref GREEN: Style = Style::default().fg(Color::Green);
    static ref YELLOW: Style = Style::default().fg(Color::Yellow);
    static ref MAGENTA: Style = Style::default().fg(Color::Magenta);
    static ref WHITE: Style = Style::default().fg(Color::White);
    static ref BLACK: Style = Style::default().fg(Color::Black);
}

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
    let mut buffer: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut buffer)?;
    let text = buffer.into_text().unwrap_or_default();
    Ok(text)
}

pub fn get_ascii_from_file_override_color(file_path: &Path, color: Color) -> Result<Text<'static>> {
    let text = get_ascii_from_file(file_path)?;
    let text = text.patch_style(Style::default().fg(color));
    Ok(text)
}

fn convert_to_text(art_orig: Vec<Vec<(&'static str, Style)>>) -> Vec<Text<'static>> {
    let art: Vec<Vec<Span>> = art_orig
        .clone()
        .into_iter()
        .map(|line| {
            let spans: Vec<Span> = line
                .into_iter()
                .map(|(text, style)| Span::styled(text, style))
                .collect();
            spans
        })
        .collect();

    vec![Text::from(
        art.iter()
            .map(|spans| Line::from(spans.clone()))
            .collect::<Vec<Line>>(),
    )]
}

// The following is a slightly modified
// version of neofetch's Apple ASCII art.
#[cfg(target_os = "macos")]
pub(crate) fn get_ascii_art(size: AsciiSize) -> Vec<Text<'static>> {
    let big_art: Vec<Vec<(&'static str, Style)>> = vec![
        vec![("                 ,MMMM.", *GREEN)],
        vec![("               .MMMMMM", *GREEN)],
        vec![("               MMMMM,", *GREEN)],
        vec![("     .;MMMMM:' MMMMMMMMMM;.", *YELLOW)],
        vec![("   MMMMMMMMMMMMNWMMMMMMMMMMM:", *YELLOW)],
        vec![(".MMMMMMMMMMMMMMMMMMMMMMMMWM.", *YELLOW)],
        vec![(" MMMMMMMMMMMMMMMMMMMMMMMMM.", *RED)],
        vec![(";MMMMMMMMMMMMMMMMMMMMMMMM:", *RED)],
        vec![(":MMMMMMMMMMMMMMMMMMMMMMMM:", *RED)],
        vec![(".MMMMMMMMMMMMMMMMMMMMMMMMM.", *MAGENTA)],
        vec![(" MMMMMMMMMMMMMMMMMMMMMMMMMMM.", *MAGENTA)],
        vec![("  .MMMMMMMMMMMMMMMMMMMMMMMMMM.", *MAGENTA)],
        vec![("    MMMMMMMMMMMMMMMMMMMMMMMM", *BLUE)],
        vec![("     ;MMMMMMMMMMMMMMMMMMMM.", *BLUE)],
        vec![("       .MMMM,.    .MMMM,.", *BLUE)],
    ];

    // The following Apple ASCII art was made by Joan Stark (jgs)
    let small_art: Vec<Vec<(&'static str, Style)>> = vec![
        vec![("        .:'", *GREEN)],
        vec![("    __ :'__", *GREEN)],
        vec![(" .'`  `-'  ``.", *YELLOW)],
        vec![(":          .-'", *YELLOW)],
        vec![(":         :", *RED)],
        vec![(" :         `-;", *RED)],
        vec![("  `.__.-.__.'", *MAGENTA)],
    ];

    match size {
        AsciiSize::Big => convert_to_text(big_art),
        AsciiSize::Small => convert_to_text(small_art),
    }
}

#[cfg(target_os = "android")]
pub(crate) fn get_ascii_art(size: AsciiSize) -> Vec<Text<'static>> {
    let big_art: Vec<Vec<(&'static str, Style)>> = vec![
        vec![("    .        .", *GREEN)],
        vec![("     \\      /", *GREEN)],
        vec![("    .oooooooo.", *GREEN)],
        vec![("   .oooooooooo. ", *GREEN)],
        vec![("   ooo  oo  ooo", *GREEN)],
        vec![("   oooooooooooo", *GREEN)],
        vec![("   ____________", *GREEN)],
        vec![("oo oooooooooooo oo", *GREEN)],
        vec![("oo oooooooooooo oo", *GREEN)],
        vec![("oo oooooooooooo oo", *GREEN)],
        vec![("   oooooooooooo", *GREEN)],
        vec![("     ooo   ooo", *GREEN)],
        vec![("     ooo   ooo", *GREEN)],
    ];

    let small_art: Vec<Vec<(&'static str, Style)>> = vec![
        vec![("  .        .   ", *GREEN)],
        vec![("   \\      /   ", *GREEN)],
        vec![("  .oooooooo.   ", *GREEN)],
        vec![(" .oooooooooo.  ", *GREEN)],
        vec![(" ooo  oo  ooo  ", *GREEN)],
        vec![(" oooooooooooo  ", *GREEN)],
    ];

    match size {
        AsciiSize::Big => convert_to_text(big_art),
        AsciiSize::Small => convert_to_text(small_art),
    }
}

#[cfg(target_os = "windows")]
pub(crate) fn get_ascii_art(size: AsciiSize) -> Vec<Text<'static>> {
    let big_art: Vec<Vec<(&'static str, Style)>> = vec![
        vec![("WWWWWWWWWWWWWW  WWWWWWWWWWWWWW", *BLUE)],
        vec![("WWWWWWWWWWWWWW  WWWWWWWWWWWWWW", *BLUE)],
        vec![("WWWWWWWWWWWWWW  WWWWWWWWWWWWWW", *BLUE)],
        vec![("WWWWWWWWWWWWWW  WWWWWWWWWWWWWW", *BLUE)],
        vec![("WWWWWWWWWWWWWW  WWWWWWWWWWWWWW", *BLUE)],
        vec![("WWWWWWWWWWWWWW  WWWWWWWWWWWWWW", *BLUE)],
        vec![("WWWWWWWWWWWWWW  WWWWWWWWWWWWWW", *BLUE)],
        vec![(" ", Style::default())],
        vec![("WWWWWWWWWWWWWW  WWWWWWWWWWWWWW", *BLUE)],
        vec![("WWWWWWWWWWWWWW  WWWWWWWWWWWWWW", *BLUE)],
        vec![("WWWWWWWWWWWWWW  WWWWWWWWWWWWWW", *BLUE)],
        vec![("WWWWWWWWWWWWWW  WWWWWWWWWWWWWW", *BLUE)],
        vec![("WWWWWWWWWWWWWW  WWWWWWWWWWWWWW", *BLUE)],
        vec![("WWWWWWWWWWWWWW  WWWWWWWWWWWWWW", *BLUE)],
        vec![("WWWWWWWWWWWWWW  WWWWWWWWWWWWWW", *BLUE)],
        vec![("WWWWWWWWWWWWWW  WWWWWWWWWWWWWW", *BLUE)],
    ];

    let small_art: Vec<Vec<(&'static str, Style)>> = vec![
        vec![("wwww  wwww", *BLUE)],
        vec![("wwww  wwww", *BLUE)],
        vec![("wwww  wwww", *BLUE)],
        vec![(" ", Style::default())],
        vec![("wwww  wwww", *BLUE)],
        vec![("wwww  wwww", *BLUE)],
        vec![("wwww  wwww", *BLUE)],
    ];

    match size {
        AsciiSize::Big => convert_to_text(big_art),
        AsciiSize::Small => convert_to_text(small_art),
    }
}

// The following penguin ASCII art was made by Joan Stark (jgs)
#[cfg(target_os = "linux")]
pub(crate) fn get_ascii_art(size: AsciiSize) -> Vec<Text<'static>> {
    let big_art: Vec<Vec<(&'static str, Style)>> = vec![
        vec![("       a8888b.", *WHITE)],
        vec![("      d888888b.", *WHITE)],
        vec![("      8P\"YP\"Y88", *WHITE)],
        vec![("      8|o||o|88", *WHITE)],
        vec![("      8", *WHITE), ("'    .", *YELLOW), ("88", *WHITE)],
        vec![("      8", *WHITE), ("`._.'", *YELLOW), (" Y8.", *WHITE)],
        vec![("     d/      `8b.", *WHITE)],
        vec![("    dP        Y8b.", *WHITE)],
        vec![("   d8:       ::88b.", *WHITE)],
        vec![("  d8\"         'Y88b", *WHITE)],
        vec![(" :8P           :888", *WHITE)],
        vec![("  8a.         _a88P", *WHITE)],
        vec![
            ("._/\"Y", *YELLOW),
            ("aa     .", *WHITE),
            ("|", *YELLOW),
            (" 88P", *WHITE),
            ("|", *YELLOW),
        ],
        vec![
            ("\\    Y", *YELLOW),
            ("P\"    `", *WHITE),
            ("|     `.", *YELLOW),
        ],
        vec![
            ("/     \\", *YELLOW),
            (".___.d", *WHITE),
            ("|    .'", *YELLOW),
        ],
        vec![("`--..__)     `._.'", *YELLOW)],
    ];

    let small_art: Vec<Vec<(&'static str, Style)>> = vec![
        vec![("    .--.", *WHITE)],
        vec![("   |o", *WHITE), ("_", *YELLOW), ("o |", *WHITE)],
        vec![("   |", *WHITE), ("\\_/", *YELLOW), (" |", *WHITE)],
        vec![("  //   \\ \\", *WHITE)],
        vec![(" (|     | )", *WHITE)],
        vec![("/'\\_   _/`\\", *WHITE)],
        vec![("\\___)=(___/", *WHITE)],
    ];

    match size {
        AsciiSize::Big => convert_to_text(big_art),
        AsciiSize::Small => convert_to_text(small_art),
    }
}

#[cfg(target_os = "freebsd")]
pub(crate) fn get_ascii_art(size: AsciiSize) -> Vec<Text<'static>> {
    // The following ASCII art was made by Dylan Araps
    // and taken from https://github.com/dylanaraps/pfetch
    let art: Vec<Vec<(&'static str, Style)>> = vec![
        vec![("/\\,-'''''-,/\\", *RED)],
        vec![("\\_)       (_//", *RED)],
        vec![("|           |", *RED)],
        vec![("|           |", *RED)],
        vec![(";         ;", *RED)],
        vec![("  '-_____-'", *RED)],
    ];
    convert_to_text(art)
}

#[cfg(target_os = "netbsd")]
pub(crate) fn get_ascii_art(size: AsciiSize) -> Vec<Text<'static>> {
    let big_art: Vec<Vec<(&'static str, Style)>> = vec![
        vec![("\\\\", *WHITE), ("`-______,----__", *YELLOW)],
        vec![(" \\\\", *WHITE), ("        __,---`.", *YELLOW)],
        vec![("  \\\\", *WHITE), ("       `.____", *YELLOW)],
        vec![("   \\\\", *WHITE), ("-______,----`.", *YELLOW)],
        vec![("    \\\\", *WHITE)],
        vec![("     \\\\", *WHITE)],
        vec![("      \\\\", *WHITE)],
        vec![("       \\\\", *WHITE)],
        vec![("        \\\\", *WHITE)],
        vec![("         \\\\", *WHITE)],
        vec![("          \\\\", *WHITE)],
    ];

    let small_art: Vec<Vec<(&'static str, Style)>> = vec![
        vec![("()", *BLACK), ("ncncncncncnc", *YELLOW)],
        vec![(" \\\\", *BLACK), ("ncncncnc", *YELLOW)],
        vec![("  \\\\", *BLACK), ("ncncncncncn", *YELLOW)],
        vec![("   \\\\", *BLACK)],
        vec![("    \\\\", *BLACK)],
        vec![("     \\\\", *BLACK)],
    ];

    match size {
        AsciiSize::Big => convert_to_text(big_art),
        AsciiSize::Small => convert_to_text(small_art),
    }
}
