pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateResponseResponsePromptCacheRetention {
    InMemory,
    TwentyFourH,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateResponseResponsePromptCacheRetention {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::InMemory => serializer.serialize_str("in_memory"),
            Self::TwentyFourH => serializer.serialize_str("24h"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateResponseResponsePromptCacheRetention {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "in_memory" => Ok(Self::InMemory),
            "24h" => Ok(Self::TwentyFourH),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateResponseResponsePromptCacheRetention {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InMemory => write!(f, "in_memory"),
            Self::TwentyFourH => write!(f, "24h"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
