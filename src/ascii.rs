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

    // ASCII ART BY JGS
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
            Span::styled("        A         A       ", *GREEN),
            Span::styled("         A       A        ", *GREEN),
            Span::styled("         AAAAAAAA         ", *GREEN),
            Span::styled("       AAAAAAAAAAAA       ", *GREEN),
            Span::styled("      AAAA AAAA AAAA      ", *GREEN),
            Span::styled("      AAAAAAAAAAAAAA      ", *GREEN),
            Span::styled("                            ", *GREEN),
            Span::styled("AAAA  AAAAAAAAAAAAAA  AAAA", *GREEN),
            Span::styled("AAAA  AAAAAAAAAAAAAA  AAAA", *GREEN),
            Span::styled("AAAA  AAAAAAAAAAAAAA  AAAA", *GREEN),
            Span::styled(" AA   AAAAAAAAAAAAAA   AA ", *GREEN),
            Span::styled("      AAAAAAAAAAAAAA      ", *GREEN),
            Span::styled("        AAAA  AAAA        ", *GREEN),
            Span::styled("        AAAA  AAAA        ", *GREEN),
        ];
        return vec![Text::from(
            art.iter()
                .map(|f| Spans::from(f.to_owned()))
                .collect::<Vec<Spans>>(),
        )];
    }

    let art: Vec<Span> = vec![
        Span::styled(" Place  ", *RED),
        Span::styled(" Holder ", *MAGENTA),
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

#[cfg(target_os = "linux")]
pub(crate) fn get_ascii_art(small: bool) -> Vec<Text<'static>> {
    if !small {
        let art: Vec<Spans> = vec![
            Spans::from(vec![Span::styled("         _nnnn_", *WHITE)]),
            Spans::from(vec![Span::styled("        dbMGGMbb", *WHITE)]),
            Spans::from(vec![Span::styled("       dbbbbbbbdb", *WHITE)]),
            Spans::from(vec![
                Span::styled("       d ", *WHITE),
                Span::styled("(@)(@)", *WHITE),
                Span::styled(" Mb", *WHITE),
            ]),
            Spans::from(vec![
                Span::styled("       d  ", *WHITE),
                Span::styled("____", *YELLOW),
                Span::styled("  db", *WHITE),
            ]),
            Spans::from(vec![
                Span::styled("      dp ", *WHITE),
                Span::styled("\\'__'/", *YELLOW),
                Span::styled(" qzb", *WHITE),
            ]),
            Spans::from(vec![Span::styled("     dzp        qzzb", *WHITE)]),
            Spans::from(vec![Span::styled("    dzp          qzzb", *WHITE)]),
            Spans::from(vec![Span::styled("   dzp            qzzb", *WHITE)]),
            Spans::from(vec![Span::styled("   dzp            qzzb", *WHITE)]),
            Spans::from(vec![Span::styled("   dzp            qzzb", *WHITE)]),
            Spans::from(vec![
                Span::styled(" __| '.", *YELLOW),
                Span::styled("        |\\ ", *YELLOW),
                Span::styled("qzzzb", *WHITE),
            ]),
            Spans::from(vec![
                Span::styled(" |    `.       | `' \\\\", *YELLOW),
                Span::styled("b", *WHITE),
            ]),
            Spans::from(vec![Span::styled("_)      '      |     .'", *YELLOW)]),
            Spans::from(vec![
                Span::styled("\\____   |", *YELLOW),
                Span::styled("______", *WHITE),
                Span::styled("|   .'", *YELLOW),
            ]),
            Spans::from(vec![Span::styled("     `-/       `--'", *YELLOW)]),
        ];

        return vec![Text::from(
            art.iter()
                .map(|f| Spans::from(f.to_owned()))
                .collect::<Vec<Spans>>(),
        )];
    }

    let art: Vec<Spans> = vec![
        Spans::from(vec![Span::styled("    .--.", *WHITE)]),
        Spans::from(vec![Span::styled("   |o o |", *WHITE)]),
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
