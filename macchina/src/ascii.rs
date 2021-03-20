#[cfg(target_os = "macos")]
pub(crate) fn get_ascii_art() -> Box<&'static [&'static str]> {
    const ASCII_ARRAY: &[&str] = &[r#"                 ,MMMM.
               .MMMMMM
               MMMMM,
     .;MMMMM:' MMMMMMMMMM;.
   MMMMMMMMMMMMNWMMMMMMMMMMM:
 .MMMMMMMMMMMMMMMMMMMMMMMMWM.
 MMMMMMMMMMMMMMMMMMMMMMMMM.
;MMMMMMMMMMMMMMMMMMMMMMMM:
:MMMMMMMMMMMMMMMMMMMMMMMM:
.MMMMMMMMMMMMMMMMMMMMMMMMM.
 MMMMMMMMMMMMMMMMMMMMMMMMMMM.
 .MMMMMMMMMMMMMMMMMMMMMMMMMMMM
  .MMMMMMMMMMMMMMMMMMMMMMMMMM.
    MMMMMMMMMMMMMMMMMMMMMMMM
     ;MMMMMMMMMMMMMMMMMMMM.
       .MMMM,.    .MMMM,."#];

    Box::new(ASCII_ARRAY)
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
pub(crate) fn get_ascii_art() -> Box<&[&'static str]> {
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
pub(crate) fn get_ascii_art() -> Box<&[&'static str]> {
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
