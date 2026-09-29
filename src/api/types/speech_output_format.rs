pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpeechOutputFormat {
    Pcm,
    Wav,
    Mp3,
    Flac,
    Opus,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SpeechOutputFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pcm => serializer.serialize_str("pcm"),
            Self::Wav => serializer.serialize_str("wav"),
            Self::Mp3 => serializer.serialize_str("mp3"),
            Self::Flac => serializer.serialize_str("flac"),
            Self::Opus => serializer.serialize_str("opus"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SpeechOutputFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pcm" => Ok(Self::Pcm),
            "wav" => Ok(Self::Wav),
            "mp3" => Ok(Self::Mp3),
            "flac" => Ok(Self::Flac),
            "opus" => Ok(Self::Opus),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SpeechOutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pcm => write!(f, "pcm"),
            Self::Wav => write!(f, "wav"),
            Self::Mp3 => write!(f, "mp3"),
            Self::Flac => write!(f, "flac"),
            Self::Opus => write!(f, "opus"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
