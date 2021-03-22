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

#[cfg(target_os = "windows")]
pub(crate) fn get_ascii_art() -> Vec<Text<'static>> {
    let art: Vec<Span> = vec![
        Span::raw(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#),
        Span::raw(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#),
        Span::raw(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#),
        Span::raw(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#),
        Span::raw(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#),
        Span::raw(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#),
        Span::raw(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#),
        Span::raw(r#" "#),
        Span::raw(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#),
        Span::raw(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#),
        Span::raw(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#),
        Span::raw(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#),
        Span::raw(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#),
        Span::raw(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#),
        Span::raw(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#),
        Span::raw(r#"WWWWWWWWWWWWWW  WWWWWWWWWWWWWW"#),
    ];

    vec![Text::from(
        art.iter()
            .map(|f| Spans::from(f.to_owned()))
            .collect::<Vec<Spans>>(),
    )]
}

#[cfg(target_os = "linux")]
pub(crate) fn get_ascii_art() -> Vec<Text<'static>> {
    let art: Vec<Span> = vec![
        Span::raw(r#"         _nnnn_"#),
        Span::raw(r#"        dGGGGMMb"#),
        Span::raw(r#"       @p~qp~~qMb"#),
        Span::raw(r#"       M|@||@) Mb"#),
        Span::raw(r#"       @,----.JMb"#),
        Span::raw(r#"      JS^\\__/  qKL"#),
        Span::raw(r#"     dZP        qKRb"#),
        Span::raw(r#"    dZP          qKKb"#),
        Span::raw(r#"   fZP            SMMb"#),
        Span::raw(r#"   HZM            MMMM"#),
        Span::raw(r#"   FqM            MMMM"#),
        Span::raw(r#" __| ".        |\\dS"qML"#),
        Span::raw(r#" |    `.       | `' \\Zq"#),
        Span::raw(r#"_)      \.___.,|     .'"#),
        Span::raw(r#"\____   )MMMMMP|   .'"#),
        Span::raw(r#"     `-'       `--'"#),
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
    let art: Vec<Span> = vec![
        Span::raw(r#"                              __,gnCCaau"#),
        Span::raw(r#"                     __,gnnCCFFFF"#),
        Span::raw(r#"(N\XCbngg,._____.,gnnndCCCCC___,"#),
        Span::raw(r#" \N\XCCCCCCCCCCCCCCCCCCCCCCCCCOOOOPYv"#),
        Span::raw(r#"  \N\XCCCCCCCCCCCCCCCCCCCCCCCCCCPFP''"#),
        Span::raw(r#"   \N\XCCCCCCCCCCCCCCCCCCCFF"'"#),
        Span::raw(r#"    \N\XCCCCCCCCCCCCCCCCF"'"#),
        Span::raw(r#"     \N\XCCCCCCCCCCCCCF"'"#),
        Span::raw(r#"      \N\"PCOCCCOC""#),
        Span::raw(r#"       \N\"#),
        Span::raw(r#"        \N\"#),
        Span::raw(r#"         \N\"#),
        Span::raw(r#"          \N\"#),
        Span::raw(r#"           \N\"#),
        Span::raw(r#"            \N\"#),
        Span::raw(r#"             \N\"#),
        Span::raw(r#"              \N\"#),
        Span::raw(r#"               \N\"#),
    ];

    vec![Text::from(
        art.iter()
            .map(|f| Spans::from(f.to_owned()))
            .collect::<Vec<Spans>>(),
    )]
}
