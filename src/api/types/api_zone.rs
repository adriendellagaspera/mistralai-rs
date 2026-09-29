pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApiZone {
    Global,
    Us,
    Eu,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ApiZone {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Global => serializer.serialize_str("global"),
            Self::Us => serializer.serialize_str("us"),
            Self::Eu => serializer.serialize_str("eu"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ApiZone {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "global" => Ok(Self::Global),
            "us" => Ok(Self::Us),
            "eu" => Ok(Self::Eu),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ApiZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Global => write!(f, "global"),
            Self::Us => write!(f, "us"),
            Self::Eu => write!(f, "eu"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
