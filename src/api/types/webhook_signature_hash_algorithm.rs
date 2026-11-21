pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WebhookSignatureHashAlgorithm {
    Sha256,
    Sha1,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WebhookSignatureHashAlgorithm {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Sha256 => serializer.serialize_str("sha256"),
            Self::Sha1 => serializer.serialize_str("sha1"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WebhookSignatureHashAlgorithm {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sha256" => Ok(Self::Sha256),
            "sha1" => Ok(Self::Sha1),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WebhookSignatureHashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sha256 => write!(f, "sha256"),
            Self::Sha1 => write!(f, "sha1"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
