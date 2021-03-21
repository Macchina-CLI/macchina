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
        Span::styled(" .MMMMMMMMMMMMMMMMMMMMMMMMMMMM", *MAGENTA),
        Span::styled("  .MMMMMMMMMMMMMMMMMMMMMMMMMM.", *BLUE),
        Span::styled("    MMMMMMMMMMMMMMMMMMMMMMMM", *MAGENTA),
        Span::styled("     ;MMMMMMMMMMMMMMMMMMMM.", *MAGENTA),
        Span::styled("       .MMMM,.    .MMMM,.", *MAGENTA),
    ];

    vec![Text::from(
        art.iter()
            .map(|f| Spans::from(f.to_owned()))
            .collect::<Vec<Spans>>(),
    )]
}

#[cfg(target_os = "windows")]
pub(crate) fn get_ascii_art() -> Vec<Text<'static>> {
    const ASCII_ARRAY: &[&str] = &[r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW
WWWWWWWWWWWWWW  WWWWWWWWWWWWWW
WWWWWWWWWWWWWW  WWWWWWWWWWWWWW
WWWWWWWWWWWWWW  WWWWWWWWWWWWWW
WWWWWWWWWWWWWW  WWWWWWWWWWWWWW
WWWWWWWWWWWWWW  WWWWWWWWWWWWWW
WWWWWWWWWWWWWW  WWWWWWWWWWWWWW

WWWWWWWWWWWWWW  WWWWWWWWWWWWWW
WWWWWWWWWWWWWW  WWWWWWWWWWWWWW
WWWWWWWWWWWWWW  WWWWWWWWWWWWWW
WWWWWWWWWWWWWW  WWWWWWWWWWWWWW
WWWWWWWWWWWWWW  WWWWWWWWWWWWWW
WWWWWWWWWWWWWW  WWWWWWWWWWWWWW
WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#];

    Box::new(ASCII_ARRAY)
}

#[cfg(target_os = "linux")]
pub(crate) fn get_ascii_art() -> Vec<Text<'static>> {
    let art: Vec<Span> = vec![
        Span::raw("         _nnnn_"),
        Span::raw("        dGGGGMMb"),
        Span::raw("       @p~qp~~qMb"),
        Span::raw("       M|@||@) M|"),
        Span::raw("       @,----.JM|"),
        Span::raw("      JS^\__/  qKL"),
        Span::raw("     dZP        qKRb"),
        Span::raw("    dZP          qKKb"),
        Span::raw("   fZP            SMMb"),
        Span::raw("   HZM            MMMM"),
        Span::raw("   FqM            MMMM"),
        Span::raw(" __| ".        |\dS"qML"),
        Span::raw(" |    `.       | `' \Zq"),
        Span::raw("_)      \.___.,|     .'"),
        Span::raw("\____   )MMMMMP|   .'"),
        Span::raw("     `-'       `--'"),
    ];

    //todo add distribution specific art
    vec![Text::from(
        art.iter()
            .map(|f| Spans::from(f.to_owned()))
            .collect::<Vec<Spans>>(),
    )]
}

#[cfg(target_os = "netbsd")]
pub(crate) fn get_ascii_art() -> Vec<Text<'static>> {
    const ASCII_ARRAY: &[&str] = &[r#"                                 __,gnCCCOObaau
       _._                  __,gnnCCCPF"''
      (N\XCbngg,._____.,gnnndCCCCCCC___,
       \N\XCCCCCCCCCCCCCCCCCCCCCCCCCCCCOOOOPYv
        \N\XCCCCCCCCCCCCCCCCCCCCCCCCCCCCPF"''
         \N\XCCCCCCCCCCCCCCCCCCCCOF"'
          \N\XCCCCCCCCCCCCCCCCF"'
           \N\XCCCCCCCCCCCCCF"'
            \N\"PCOCCCOC"
             \N\
              \N\
               \N\
                \N\
                 \N\
                  \N\
                   \N\
                    \N\
                     \N\"#];

    Box::new(ASCII_ARRAY)
}
