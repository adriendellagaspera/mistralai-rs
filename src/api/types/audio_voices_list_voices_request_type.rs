pub use crate::prelude::*;

/// Filter the voices between customs and presets
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListVoicesRequestType {
    All,
    Custom,
    Preset,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListVoicesRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::All => serializer.serialize_str("all"),
            Self::Custom => serializer.serialize_str("custom"),
            Self::Preset => serializer.serialize_str("preset"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListVoicesRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "all" => Ok(Self::All),
            "custom" => Ok(Self::Custom),
            "preset" => Ok(Self::Preset),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListVoicesRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Custom => write!(f, "custom"),
            Self::Preset => write!(f, "preset"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
