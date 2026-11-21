pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResponsesPromptCacheBreakpointTtl {
    ThirtyM,
    TwelveH,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResponsesPromptCacheBreakpointTtl {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ThirtyM => serializer.serialize_str("30m"),
            Self::TwelveH => serializer.serialize_str("12h"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResponsesPromptCacheBreakpointTtl {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "30m" => Ok(Self::ThirtyM),
            "12h" => Ok(Self::TwelveH),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResponsesPromptCacheBreakpointTtl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ThirtyM => write!(f, "30m"),
            Self::TwelveH => write!(f, "12h"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
