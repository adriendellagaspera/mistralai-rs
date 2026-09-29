pub use crate::prelude::*;

/// How a connector's OAuth server metadata was obtained.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OAuthMetadataSource {
    Autodiscovery,
    Provided,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OAuthMetadataSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Autodiscovery => serializer.serialize_str("autodiscovery"),
            Self::Provided => serializer.serialize_str("provided"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OAuthMetadataSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "autodiscovery" => Ok(Self::Autodiscovery),
            "provided" => Ok(Self::Provided),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OAuthMetadataSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Autodiscovery => write!(f, "autodiscovery"),
            Self::Provided => write!(f, "provided"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
