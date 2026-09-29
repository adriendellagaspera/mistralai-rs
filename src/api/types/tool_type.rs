pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolType {
    Rag,
    Image,
    Code,
    Event,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ToolType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Rag => serializer.serialize_str("rag"),
            Self::Image => serializer.serialize_str("image"),
            Self::Code => serializer.serialize_str("code"),
            Self::Event => serializer.serialize_str("event"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ToolType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "rag" => Ok(Self::Rag),
            "image" => Ok(Self::Image),
            "code" => Ok(Self::Code),
            "event" => Ok(Self::Event),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ToolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rag => write!(f, "rag"),
            Self::Image => write!(f, "image"),
            Self::Code => write!(f, "code"),
            Self::Event => write!(f, "event"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
