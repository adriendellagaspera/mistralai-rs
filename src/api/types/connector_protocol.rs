pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectorProtocol {
    Mcp,
    Http,
    Turbine,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConnectorProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Mcp => serializer.serialize_str("mcp"),
            Self::Http => serializer.serialize_str("http"),
            Self::Turbine => serializer.serialize_str("turbine"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConnectorProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mcp" => Ok(Self::Mcp),
            "http" => Ok(Self::Http),
            "turbine" => Ok(Self::Turbine),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConnectorProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mcp => write!(f, "mcp"),
            Self::Http => write!(f, "http"),
            Self::Turbine => write!(f, "turbine"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
