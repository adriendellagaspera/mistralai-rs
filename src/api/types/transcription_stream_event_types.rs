pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TranscriptionStreamEventTypes {
    TranscriptionLanguage,
    TranscriptionSegment,
    TranscriptionTextDelta,
    TranscriptionDone,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TranscriptionStreamEventTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TranscriptionLanguage => serializer.serialize_str("transcription.language"),
            Self::TranscriptionSegment => serializer.serialize_str("transcription.segment"),
            Self::TranscriptionTextDelta => serializer.serialize_str("transcription.text.delta"),
            Self::TranscriptionDone => serializer.serialize_str("transcription.done"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TranscriptionStreamEventTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "transcription.language" => Ok(Self::TranscriptionLanguage),
            "transcription.segment" => Ok(Self::TranscriptionSegment),
            "transcription.text.delta" => Ok(Self::TranscriptionTextDelta),
            "transcription.done" => Ok(Self::TranscriptionDone),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TranscriptionStreamEventTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TranscriptionLanguage => write!(f, "transcription.language"),
            Self::TranscriptionSegment => write!(f, "transcription.segment"),
            Self::TranscriptionTextDelta => write!(f, "transcription.text.delta"),
            Self::TranscriptionDone => write!(f, "transcription.done"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
