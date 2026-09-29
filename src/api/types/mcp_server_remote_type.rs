pub use crate::prelude::*;

/// Transport type
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum McpServerRemoteType {
    StreamableHttp,
    Sse,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for McpServerRemoteType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::StreamableHttp => serializer.serialize_str("streamable-http"),
            Self::Sse => serializer.serialize_str("sse"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for McpServerRemoteType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "streamable-http" => Ok(Self::StreamableHttp),
            "sse" => Ok(Self::Sse),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for McpServerRemoteType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StreamableHttp => write!(f, "streamable-http"),
            Self::Sse => write!(f, "sse"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
