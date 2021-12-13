use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum Border {
    Thick,
    Plain,
    Rounded,
    Double,
}

impl<'de> Deserialize<'de> for Border {
    fn deserialize<D>(deserializer: D) -> Result<Border, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match &s.as_str().to_lowercase()[..] {
            "thick" => Ok(Self::Thick),
            "plain" => Ok(Self::Plain),
            "double" => Ok(Self::Double),
            _ => Ok(Self::Rounded),
        }
    }
}

impl Serialize for Border {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_some(&self)
    }
}
