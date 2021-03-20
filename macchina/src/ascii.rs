use tui::style::{Color, Style};
use tui::text::{Span, Spans, Text};

#[cfg(target_os = "macos")]
pub(crate) fn get_ascii_art() -> Vec<Text<'static>> {
    let color = Style::default().fg(Color::Red);

    let art: Vec<Span> = vec![
        Span::styled("                 ,MMMM.", color),
        Span::raw("               .MMMMMM"),
        Span::styled("               MMMMM,", color),
        Span::raw("     .;MMMMM:' MMMMMMMMMM;."),
        Span::styled("   MMMMMMMMMMMMNWMMMMMMMMMMM:", color),
        Span::raw(" .MMMMMMMMMMMMMMMMMMMMMMMMWM."),
        Span::styled(" MMMMMMMMMMMMMMMMMMMMMMMMM.", color),
        Span::raw(";MMMMMMMMMMMMMMMMMMMMMMMM:"),
        Span::styled(":MMMMMMMMMMMMMMMMMMMMMMMM:", color),
        Span::raw(".MMMMMMMMMMMMMMMMMMMMMMMMM."),
        Span::styled(" MMMMMMMMMMMMMMMMMMMMMMMMMMM.", color),
        Span::raw(" .MMMMMMMMMMMMMMMMMMMMMMMMMMMM"),
        Span::styled("  .MMMMMMMMMMMMMMMMMMMMMMMMMM.", color),
        Span::raw("    MMMMMMMMMMMMMMMMMMMMMMMM"),
        Span::styled("     ;MMMMMMMMMMMMMMMMMMMM.", color),
        Span::raw("       .MMMM,.    .MMMM,."),
    ];

    vec![Text::from(
        art.iter()
            .map(|f| Spans::from(f.to_owned()))
            .collect::<Vec<Spans>>(),
    )]
}

#[cfg(target_os = "windows")]
pub(crate) fn get_ascii_art() -> Box<&'static [&'static str]> {
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
pub(crate) fn get_ascii_art() -> Box<&'static [&'static str]> {
    const ASCII_ARRAY: &[&str] = &[r#"         _nnnn_
        dGGGGMMb
       @p~qp~~qMb
       M|@||@) M|
       @,----.JM|
      JS^\__/  qKL
     dZP        qKRb
    dZP          qKKb
   fZP            SMMb
   HZM            MMMM
   FqM            MMMM
 __| ".        |\dS"qML
 |    `.       | `' \Zq
_)      \.___.,|     .'
\____   )MMMMMP|   .'
     `-'       `--'"#];

    //todo add distribution specific art
    Box::new(ASCII_ARRAY)
}

#[cfg(target_os = "netbsd")]
pub(crate) fn get_ascii_art() -> Box<&'static [&'static str]> {
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
