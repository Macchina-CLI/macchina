use tui::style::{Color, Style};
use tui::text::{Span, Spans, Text};

lazy_static! {
    static ref BLUE: Style = Style::default().fg(Color::Blue);
    static ref RED: Style = Style::default().fg(Color::Red);
    static ref GREEN: Style = Style::default().fg(Color::Green);
    static ref YELLOW: Style = Style::default().fg(Color::Yellow);
    static ref MAGENTA: Style = Style::default().fg(Color::Magenta);
    static ref GRAY: Style = Style::default().fg(Color::Gray);
    static ref WHITE: Style = Style::default().fg(Color::White);
    static ref CYAN: Style = Style::default().fg(Color::Cyan);
    static ref BLACK: Style = Style::default().fg(Color::Black);
}

#[cfg(target_os = "macos")]
pub(crate) fn get_ascii_art() -> Vec<Text<'static>> {
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

    vec![Text::from(
        art.iter()
            .map(|f| Spans::from(f.to_owned()))
            .collect::<Vec<Spans>>(),
    )]
}
#[cfg(target_os = "android")]
pub(crate) fn get_ascii_art() -> Vec<Text<'static>> {
    let art: Vec<Span> = vec![
        Span::styled("                                           ", *GREEN),
        Span::styled("              A             A              ", *GREEN),
        Span::styled("              AA           AA              ", *GREEN),
        Span::styled("               AAAAAAAAAAAAA               ", *GREEN),
        Span::styled("             AAAAAAAAAAAAAAAAA             ", *GREEN),
        Span::styled("           AAAAa aAAAAAAAa aAAAA           ", *GREEN),
        Span::styled("          AAAAAa aAAAAAAAa aAAAAA          ", *GREEN),
        Span::styled("         AAAAAAAAAAAAAAAAAAAAAAAAA         ", *GREEN),
        Span::styled("         AAAAAAAAAAAAAAAAAAAAAAAAA         ", *GREEN),
        Span::styled("                                           ", *GREEN),
        Span::styled(" AAAAA  AAAAAAAAAAAAAAAAAAAAAAAAAA  AAAAA  ", *GREEN),
        Span::styled(" AAAAA  AAAAAAAAAAAAAAAAAAAAAAAAAA  AAAAA  ", *GREEN),
        Span::styled(" AAAAA  AAAAAAAAAAAAAAAAAAAAAAAAAA  AAAAA  ", *GREEN),
        Span::styled(" AAAAA  AAAAAAAAAAAAAAAAAAAAAAAAAA  AAAAA  ", *GREEN),
        Span::styled(" AAAAA  AAAAAAAAAAAAAAAAAAAAAAAAAA  AAAAA  ", *GREEN),
        Span::styled(" AAAAA  AAAAAAAAAAAAAAAAAAAAAAAAAA  AAAAA  ", *GREEN),
        Span::styled(" AAAAA  AAAAAAAAAAAAAAAAAAAAAAAAAA  AAAAA  ", *GREEN),
        Span::styled(" AAAAA  AAAAAAAAAAAAAAAAAAAAAAAAAA  AAAAA  ", *GREEN),
        Span::styled(" AAAAA  AAAAAAAAAAAAAAAAAAAAAAAAAA  AAAAA  ", *GREEN),
        Span::styled("  AAA   AAAAAAAAAAAAAAAAAAAAAAAAAA   AAA   ", *GREEN),
        Span::styled("        AAAAAAAAAAAAAAAAAAAAAAAAAA         ", *GREEN),
        Span::styled("        AAAAAAAAAAAAAAAAAAAAAAAAAA         ", *GREEN),
        Span::styled("        AAAAAAAAAAAAAAAAAAAAAAAAAA         ", *GREEN),
        Span::styled("              AAAAAA    AAAAA              ", *GREEN),
        Span::styled("              AAAAAA    AAAAA              ", *GREEN),
        Span::styled("              AAAAAA    AAAAA              ", *GREEN),
        Span::styled("              AAAAAA    AAAAA              ", *GREEN),
        Span::styled("              AAAAAA    AAAAA              ", *GREEN),
        Span::styled("                                           ", *GREEN),
    ];

    vec![Text::from(
        art.iter()
            .map(|f| Spans::from(f.to_owned()))
            .collect::<Vec<Spans>>(),
    )]
}

#[cfg(target_os = "windows")]
pub(crate) fn get_ascii_art() -> Vec<Text<'static>> {
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

    vec![Text::from(
        art.iter()
            .map(|f| Spans::from(f.to_owned()))
            .collect::<Vec<Spans>>(),
    )]
}

#[cfg(target_os = "linux")]
pub(crate) fn get_ascii_art() -> Vec<Text<'static>> {
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

    //todo add distribution specific art
    vec![Text::from(art)]
}

#[cfg(target_os = "netbsd")]
pub(crate) fn get_ascii_art() -> Vec<Text<'static>> {
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
            Span::styled("(N\\", *WHITE),
            Span::styled("XCbngg,._____.,gnnndCCCCC___,", *YELLOW),
        ]),
        Spans::from(vec![
            Span::styled(" \\N\\", *WHITE),
            Span::styled("XCCCCCCCCCCCCCCCCCCCCCCCCCOOOOPYv", *YELLOW),
        ]),
        Spans::from(vec![
            Span::styled("  \\N\\", *WHITE),
            Span::styled("XCCCCCCCCCCCCCCCCCCCCCCCCCCPFP''", *YELLOW),
        ]),
        Spans::from(vec![
            Span::styled("   \\N\\", *WHITE),
            Span::styled("XCCCCCCCCCCCCCCCCCCCFF\"'", *YELLOW),
        ]),
        Spans::from(vec![
            Span::styled("    \\N\\", *WHITE),
            Span::styled("XCCCCCCCCCCCCCCCCF\"'", *YELLOW),
        ]),
        Spans::from(vec![
            Span::styled("     \\N\\", *WHITE),
            Span::styled("XCCCCCCCCCCCCCF\"'", *YELLOW),
        ]),
        Spans::from(vec![
            Span::styled("      \\N\\", *WHITE),
            Span::styled("\"PCOCCCOC\"", *YELLOW),
        ]),
        Spans::from(vec![Span::styled("       \\N\\", *WHITE)]),
        Spans::from(vec![Span::styled("        \\N\\", *WHITE)]),
        Spans::from(vec![Span::styled("         \\N\\", *WHITE)]),
        Spans::from(vec![Span::styled("          \\N\\", *WHITE)]),
        Spans::from(vec![Span::styled("           \\N\\", *WHITE)]),
        Spans::from(vec![Span::styled("            \\N\\", *WHITE)]),
        Spans::from(vec![Span::styled("             \\N\\", *WHITE)]),
        Spans::from(vec![Span::styled("              \\N\\", *WHITE)]),
        Spans::from(vec![Span::styled("               \\N\\", *WHITE)]),
    ];

    vec![Text::from(art)]
}
