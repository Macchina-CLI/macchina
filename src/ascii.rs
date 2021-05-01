use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use tui::style::{Color, Style};
use tui::text::{Span, Spans, Text};

lazy_static! {
    static ref BLUE: Style = Style::default().fg(Color::Blue);
    static ref RED: Style = Style::default().fg(Color::Red);
    static ref GREEN: Style = Style::default().fg(Color::Green);
    static ref YELLOW: Style = Style::default().fg(Color::Yellow);
    static ref MAGENTA: Style = Style::default().fg(Color::Magenta);
    static ref WHITE: Style = Style::default().fg(Color::White);
    static ref BLACK: Style = Style::default().fg(Color::Black);
}

// TODO: Parse the file given more thorougly and use the custom colours supplied in the file
// instead of some preset
pub fn get_ascii_from_file(
    file_path: &Path,
    override_color: Option<Color>,
) -> Result<Vec<Text<'static>>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    return Ok(vec![Text::from(
        reader
            .lines()
            .map(|line| {
                if let Some(override_color) = override_color {
                    Spans::from(Span::styled(
                        line.unwrap(),
                        Style::default().fg(override_color),
                    ))
                } else {
                    Spans::from(Span::from(line.unwrap()))
                }
            })
            .collect::<Vec<Spans>>(),
    )]);
}

// The following is a slightly modified
// version of neofetch's Apple ASCII art.
#[cfg(target_os = "macos")]
pub(crate) fn get_ascii_art(small: bool) -> Vec<Text<'static>> {
    if !small {
        let art: Vec<Span> = vec![
            Span::styled("                 ,MMMM.", *GREEN),
            Span::styled("               .MMMMMM", *GREEN),
            Span::styled("               MMMMM,", *GREEN),
            Span::styled("     .;MMMMM:' MMMMMMMMMM;.", *YELLOW),
            Span::styled("   MMMMMMMMMMMMNWMMMMMMMMMMM:", *YELLOW),
            Span::styled(" .MMMMMMMMMMMMMMMMMMMMMMMMWM.", *YELLOW),
            Span::styled(" MMMMMMMMMMMMMMMMMMMMMMMMM.", *RED),
            Span::styled(";MMMMMMMMMMMMMMMMMMMMMMMM:", *RED),
            Span::styled(":MMMMMMMMMMMMMMMMMMMMMMMM:", *RED),
            Span::styled(".MMMMMMMMMMMMMMMMMMMMMMMMM.", *MAGENTA),
            Span::styled(" MMMMMMMMMMMMMMMMMMMMMMMMMMM.", *MAGENTA),
            Span::styled("  .MMMMMMMMMMMMMMMMMMMMMMMMMM.", *MAGENTA),
            Span::styled("    MMMMMMMMMMMMMMMMMMMMMMMM", *BLUE),
            Span::styled("     ;MMMMMMMMMMMMMMMMMMMM.", *BLUE),
            Span::styled("       .MMMM,.    .MMMM,.", *BLUE),
        ];

        return vec![Text::from(
            art.iter()
                .map(|f| Spans::from(f.to_owned()))
                .collect::<Vec<Spans>>(),
        )];
    }

    // The following Apple ASCII art was made by Joan Stark (jgs)
    let art: Vec<Span> = vec![
        Span::styled("        .:'", *GREEN),
        Span::styled("    __ :'__", *GREEN),
        Span::styled(" .'`  `-'  ``.", *YELLOW),
        Span::styled(":          .-'", *YELLOW),
        Span::styled(":         :", *RED),
        Span::styled(" :         `-;", *RED),
        Span::styled("  `.__.-.__.'", *MAGENTA),
    ];

    vec![Text::from(
        art.iter()
            .map(|f| Spans::from(f.to_owned()))
            .collect::<Vec<Spans>>(),
    )]
}

#[cfg(target_os = "android")]
pub(crate) fn get_ascii_art(small: bool) -> Vec<Text<'static>> {
    if !small {
        let art: Vec<Span> = vec![
            Span::styled("    .        .", *GREEN),
            Span::styled("     \\      /", *GREEN),
            Span::styled("    .oooooooo.", *GREEN),
            Span::styled("   .oooooooooo. ", *GREEN),
            Span::styled("   ooo  oo  ooo", *GREEN),
            Span::styled("   oooooooooooo", *GREEN),
            Span::styled("   ____________", *GREEN),
            Span::styled("oo oooooooooooo oo", *GREEN),
            Span::styled("oo oooooooooooo oo", *GREEN),
            Span::styled("oo oooooooooooo oo", *GREEN),
            Span::styled("   oooooooooooo", *GREEN),
            Span::styled("     ooo   ooo", *GREEN),
            Span::styled("     ooo   ooo", *GREEN),
        ];

        return vec![Text::from(
            art.iter()
                .map(|f| Spans::from(f.to_owned()))
                .collect::<Vec<Spans>>(),
        )];
    }

    let art: Vec<Span> = vec![
        Span::styled("  .        .   ", *GREEN),
        Span::styled("   \\      /   ", *GREEN),
        Span::styled("  .oooooooo.   ", *GREEN),
        Span::styled(" .oooooooooo.  ", *GREEN),
        Span::styled(" ooo  oo  ooo  ", *GREEN),
        Span::styled(" oooooooooooo  ", *GREEN),
    ];
    vec![Text::from(
        art.iter()
            .map(|f| Spans::from(f.to_owned()))
            .collect::<Vec<Spans>>(),
    )]
}

#[cfg(target_os = "windows")]
pub(crate) fn get_ascii_art(small: bool) -> Vec<Text<'static>> {
    if !small {
        let art: Vec<Span> = vec![
            Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, *BLUE),
            Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, *BLUE),
            Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, *BLUE),
            Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, *BLUE),
            Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, *BLUE),
            Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, *BLUE),
            Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, *BLUE),
            Span::raw(r#" "#),
            Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, *BLUE),
            Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, *BLUE),
            Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, *BLUE),
            Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, *BLUE),
            Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, *BLUE),
            Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, *BLUE),
            Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, *BLUE),
            Span::styled(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#, *BLUE),
        ];

        return vec![Text::from(
            art.iter()
                .map(|f| Spans::from(f.to_owned()))
                .collect::<Vec<Spans>>(),
        )];
    }

    let art: Vec<Span> = vec![
        Span::styled("wwww  wwww", *BLUE),
        Span::styled("wwww  wwww", *BLUE),
        Span::styled("wwww  wwww", *BLUE),
        Span::raw(r#" "#),
        Span::styled("wwww  wwww", *BLUE),
        Span::styled("wwww  wwww", *BLUE),
        Span::styled("wwww  wwww", *BLUE),
    ];

    vec![Text::from(
        art.iter()
            .map(|f| Spans::from(f.to_owned()))
            .collect::<Vec<Spans>>(),
    )]
}

// The following penguin ASCII art was made by Joan Stark (jgs)
#[cfg(target_os = "linux")]
pub(crate) fn get_ascii_art(small: bool) -> Vec<Text<'static>> {
    if !small {
        let art: Vec<Spans> = vec![
            Spans::from(vec![Span::styled("       a8888b.", *WHITE)]),
            Spans::from(vec![Span::styled("      d888888b.", *WHITE)]),
            Spans::from(vec![Span::styled("      8P\"YP\"Y88", *WHITE)]),
            Spans::from(vec![Span::styled("      8|o||o|88", *WHITE)]),
            Spans::from(vec![
                Span::styled("      8", *WHITE),
                Span::styled("'    .", *YELLOW),
                Span::styled("88", *WHITE),
            ]),
            Spans::from(vec![
                Span::styled("      8", *WHITE),
                Span::styled("`._.'", *YELLOW),
                Span::styled(" Y8.", *WHITE),
            ]),
            Spans::from(vec![Span::styled("     d/      `8b.", *WHITE)]),
            Spans::from(vec![Span::styled("    dP        Y8b.", *WHITE)]),
            Spans::from(vec![Span::styled("   d8:       ::88b.", *WHITE)]),
            Spans::from(vec![Span::styled("  d8\"         'Y88b", *WHITE)]),
            Spans::from(vec![Span::styled(" :8P           :888", *WHITE)]),
            Spans::from(vec![Span::styled("  8a.         _a88P", *WHITE)]),
            Spans::from(vec![
                Span::styled("._/\"Y", *YELLOW),
                Span::styled("aa     .", *WHITE),
                Span::styled("|", *YELLOW),
                Span::styled(" 88P", *WHITE),
                Span::styled("|", *YELLOW),
            ]),
            Spans::from(vec![
                Span::styled("\\    Y", *YELLOW),
                Span::styled("P\"    `", *WHITE),
                Span::styled("|     `.", *YELLOW),
            ]),
            Spans::from(vec![
                Span::styled("/     \\", *YELLOW),
                Span::styled(".___.d", *WHITE),
                Span::styled("|    .'", *YELLOW),
            ]),
            Spans::from(vec![
                Span::styled("`--..__)", *YELLOW),
                Span::styled("8888P", *WHITE),
                Span::styled("`._.'", *YELLOW),
            ]),
        ];

        return vec![Text::from(
            art.iter()
                .map(|f| Spans::from(f.to_owned()))
                .collect::<Vec<Spans>>(),
        )];
    }

    // The following penguin ASCII art was found and
    // taken from: https://asciiart.website/index.php
    // Artist attribution missing.
    // Thank you to whoever made it :^)
    let art: Vec<Spans> = vec![
        Spans::from(vec![Span::styled("    .--.", *WHITE)]),
        Spans::from(vec![
            Span::styled("   |o", *WHITE),
            Span::styled("_", *YELLOW),
            Span::styled("o |", *WHITE),
        ]),
        Spans::from(vec![
            Span::styled("   |", *WHITE),
            Span::styled("\\_/", *YELLOW),
            Span::styled(" |", *WHITE),
        ]),
        Spans::from(vec![Span::styled("  //   \\ \\", *WHITE)]),
        Spans::from(vec![Span::styled(" (|     | )", *WHITE)]),
        Spans::from(vec![Span::styled("/'\\_   _/`\\", *WHITE)]),
        Spans::from(vec![Span::styled("\\___)=(___/", *WHITE)]),
    ];

    vec![Text::from(art)]
}

// The following NetBSD ASCII art was found and taken
// from: http://www.netbsd.org/gallery/mobile-logos.html
#[cfg(target_os = "netbsd")]
pub(crate) fn get_ascii_art(small: bool) -> Vec<Text<'static>> {
    if !small {
        let art: Vec<Spans> = vec![
            Spans::from(vec![Span::styled(
                "                              __,gnCCaau",
                *YELLOW,
            )]),
            Spans::from(vec![Span::styled(
                "                     __,gnnCCFFFF",
                *YELLOW,
            )]),
            Spans::from(vec![
                Span::styled("(N\\", *BLACK),
                Span::styled("XCbngg,._____.,gnnndCCCCC___,", *YELLOW),
            ]),
            Spans::from(vec![
                Span::styled(" \\N\\", *BLACK),
                Span::styled("XCCCCCCCCCCCCCCCCCCCCCCCCCOOOOPYv", *YELLOW),
            ]),
            Spans::from(vec![
                Span::styled("  \\N\\", *BLACK),
                Span::styled("XCCCCCCCCCCCCCCCCCCCCCCCCCCPFP''", *YELLOW),
            ]),
            Spans::from(vec![
                Span::styled("   \\N\\", *BLACK),
                Span::styled("XCCCCCCCCCCCCCCCCCCCFF\"'", *YELLOW),
            ]),
            Spans::from(vec![
                Span::styled("    \\N\\", *BLACK),
                Span::styled("XCCCCCCCCCCCCCCCCF\"'", *YELLOW),
            ]),
            Spans::from(vec![
                Span::styled("     \\N\\", *BLACK),
                Span::styled("XCCCCCCCCCCCCCF\"'", *YELLOW),
            ]),
            Spans::from(vec![
                Span::styled("      \\N\\", *BLACK),
                Span::styled("\"PCOCCCOC\"", *YELLOW),
            ]),
            Spans::from(vec![Span::styled("       \\N\\", *BLACK)]),
            Spans::from(vec![Span::styled("        \\N\\", *BLACK)]),
            Spans::from(vec![Span::styled("         \\N\\", *BLACK)]),
            Spans::from(vec![Span::styled("          \\N\\", *BLACK)]),
            Spans::from(vec![Span::styled("           \\N\\", *BLACK)]),
            Spans::from(vec![Span::styled("            \\N\\", *BLACK)]),
            Spans::from(vec![Span::styled("             \\N\\", *BLACK)]),
            Spans::from(vec![Span::styled("              \\N\\", *BLACK)]),
            Spans::from(vec![Span::styled("               \\N\\", *BLACK)]),
        ];

        return vec![Text::from(
            art.iter()
                .map(|f| Spans::from(f.to_owned()))
                .collect::<Vec<Spans>>(),
        )];
    }

    let art: Vec<Spans> = vec![
        Spans::from(vec![
            Span::styled("()", *BLACK),
            Span::styled("ncncncncncnc", *YELLOW),
        ]),
        Spans::from(vec![
            Span::styled(" \\\\", *BLACK),
            Span::styled("ncncncnc", *YELLOW),
        ]),
        Spans::from(vec![
            Span::styled("  \\\\", *BLACK),
            Span::styled("ncncncncncn", *YELLOW),
        ]),
        Spans::from(vec![Span::styled("   \\\\", *BLACK)]),
        Spans::from(vec![Span::styled("    \\\\", *BLACK)]),
        Spans::from(vec![Span::styled("     \\\\", *BLACK)]),
    ];

    vec![Text::from(art)]
}
