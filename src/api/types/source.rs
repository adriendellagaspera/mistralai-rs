pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Source {
    Upload,
    Repository,
    Mistral,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for Source {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Upload => serializer.serialize_str("upload"),
            Self::Repository => serializer.serialize_str("repository"),
            Self::Mistral => serializer.serialize_str("mistral"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for Source {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "upload" => Ok(Self::Upload),
            "repository" => Ok(Self::Repository),
            "mistral" => Ok(Self::Mistral),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Upload => write!(f, "upload"),
            Self::Repository => write!(f, "repository"),
            Self::Mistral => write!(f, "mistral"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
