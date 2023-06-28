use rand::seq::SliceRandom;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub enum ColorTypes {
    Base,
    Hexadecimal,
    Indexed,
}

impl<'de> Deserialize<'de> for ColorTypes {
    fn deserialize<D>(deserializer: D) -> Result<ColorTypes, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match &s.as_str().to_lowercase()[..] {
            "hexadecimal" => Ok(Self::Hexadecimal),
            "indexed" => Ok(Self::Indexed),
            _ => Ok(Self::Base),
        }
    }
}

impl Serialize for ColorTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_some(&self)
    }
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
