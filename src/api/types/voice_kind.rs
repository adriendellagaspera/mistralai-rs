pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VoiceKind {
    Ivc,
    Pvc,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VoiceKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ivc => serializer.serialize_str("ivc"),
            Self::Pvc => serializer.serialize_str("pvc"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VoiceKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ivc" => Ok(Self::Ivc),
            "pvc" => Ok(Self::Pvc),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VoiceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ivc => write!(f, "ivc"),
            Self::Pvc => write!(f, "pvc"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
