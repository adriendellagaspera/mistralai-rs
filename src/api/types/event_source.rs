pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventSource {
    Database,
    Live,
    Hybrid,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EventSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Database => serializer.serialize_str("DATABASE"),
            Self::Live => serializer.serialize_str("LIVE"),
            Self::Hybrid => serializer.serialize_str("HYBRID"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EventSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "DATABASE" => Ok(Self::Database),
            "LIVE" => Ok(Self::Live),
            "HYBRID" => Ok(Self::Hybrid),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EventSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database => write!(f, "DATABASE"),
            Self::Live => write!(f, "LIVE"),
            Self::Hybrid => write!(f, "HYBRID"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
