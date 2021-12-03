use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use tui::style::Color;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorTypes {
    Base,
    Hexadecimal,
    Indexed,
}

pub fn make_random_color() -> Color {
    use Color::*;
    let mut random = rand::thread_rng();
    let colors = [
        Red,
        Black,
        Green,
        Yellow,
        Blue,
        Magenta,
        Cyan,
        Gray,
        LightRed,
        LightGreen,
        LightYellow,
        LightBlue,
        LightMagenta,
        LightCyan,
        White,
    ];
    *colors.choose(&mut random).unwrap()
}
