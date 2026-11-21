pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VoiceAgentStateChangedEventOldState {
    Initializing,
    Idle,
    Listening,
    Thinking,
    Speaking,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VoiceAgentStateChangedEventOldState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Initializing => serializer.serialize_str("initializing"),
            Self::Idle => serializer.serialize_str("idle"),
            Self::Listening => serializer.serialize_str("listening"),
            Self::Thinking => serializer.serialize_str("thinking"),
            Self::Speaking => serializer.serialize_str("speaking"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VoiceAgentStateChangedEventOldState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "initializing" => Ok(Self::Initializing),
            "idle" => Ok(Self::Idle),
            "listening" => Ok(Self::Listening),
            "thinking" => Ok(Self::Thinking),
            "speaking" => Ok(Self::Speaking),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VoiceAgentStateChangedEventOldState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Initializing => write!(f, "initializing"),
            Self::Idle => write!(f, "idle"),
            Self::Listening => write!(f, "listening"),
            Self::Thinking => write!(f, "thinking"),
            Self::Speaking => write!(f, "speaking"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
