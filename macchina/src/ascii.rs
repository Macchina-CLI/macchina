use tui::style::{Color, Style};
use tui::text::{Span, Spans, Text};

pub enum Colors {
    Red,
    Green,
    Blue,
    Magenta,
    Yellow,
    Gray,
    White,
    Cyan,
}

impl Colors {
    pub fn set(c: Color) -> Style {
        match c {
            Blue => Style::default().fg(Color::Blue),
            Red => Style::default().fg(Color::Red),
            Green => Style::default().fg(Color::Green),
            Yellow => Style::default().fg(Color::Yellow),
            Magenta => Style::default().fg(Color::Magenta),
            Gray => Style::default().fg(Color::Gray),
            White => Style::default().fg(Color::White),
        }
    }
}

#[cfg(target_os = "macos")]
pub(crate) fn get_ascii_art() -> Vec<Text<'static>> {
    let art: Vec<Span> = vec![
        Span::styled("                 ,MMMM.", Colors::set(Color::Green)),
        Span::styled("               .MMMMMM", Colors::set(Color::Green)),
        Span::styled("               MMMMM,", Colors::set(Color::Green)),
        Span::styled("     .;MMMMM:' MMMMMMMMMM;.", Colors::set(Color::Yellow)),
        Span::styled("   MMMMMMMMMMMMNWMMMMMMMMMMM:", Colors::set(Color::Yellow)),
        Span::styled(" .MMMMMMMMMMMMMMMMMMMMMMMMWM.", Colors::set(Color::Yellow)),
        Span::styled(" MMMMMMMMMMMMMMMMMMMMMMMMM.", Colors::set(Color::Red)),
        Span::styled(";MMMMMMMMMMMMMMMMMMMMMMMM:", Colors::set(Color::Red)),
        Span::styled(":MMMMMMMMMMMMMMMMMMMMMMMM:", Colors::set(Color::Red)),
        Span::styled(".MMMMMMMMMMMMMMMMMMMMMMMMM.", Colors::set(Color::Magenta)),
        Span::styled(" MMMMMMMMMMMMMMMMMMMMMMMMMMM.", Colors::set(Color::Magenta)),
        Span::styled(
            " .MMMMMMMMMMMMMMMMMMMMMMMMMMMM",
            Colors::set(Color::Magenta),
        ),
        Span::styled("  .MMMMMMMMMMMMMMMMMMMMMMMMMM.", Colors::set(Color::Blue)),
        Span::styled("    MMMMMMMMMMMMMMMMMMMMMMMM", Colors::set(Color::Magenta)),
        Span::styled("     ;MMMMMMMMMMMMMMMMMMMM.", Colors::set(Color::Magenta)),
        Span::styled("       .MMMM,.    .MMMM,.", Colors::set(Color::Magenta)),
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
