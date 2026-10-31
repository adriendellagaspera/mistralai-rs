pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VoiceGender {
    Female,
    Male,
    Neutral,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VoiceGender {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Female => serializer.serialize_str("female"),
            Self::Male => serializer.serialize_str("male"),
            Self::Neutral => serializer.serialize_str("neutral"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VoiceGender {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "female" => Ok(Self::Female),
            "male" => Ok(Self::Male),
            "neutral" => Ok(Self::Neutral),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VoiceGender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Female => write!(f, "female"),
            Self::Male => write!(f, "male"),
            Self::Neutral => write!(f, "neutral"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
