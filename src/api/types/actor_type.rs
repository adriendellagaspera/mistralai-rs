pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActorType {
    Human,
    ApiKey,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ActorType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Human => serializer.serialize_str("HUMAN"),
            Self::ApiKey => serializer.serialize_str("API_KEY"),
            Self::Other => serializer.serialize_str("OTHER"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ActorType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "HUMAN" => Ok(Self::Human),
            "API_KEY" => Ok(Self::ApiKey),
            "OTHER" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ActorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Human => write!(f, "HUMAN"),
            Self::ApiKey => write!(f, "API_KEY"),
            Self::Other => write!(f, "OTHER"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
