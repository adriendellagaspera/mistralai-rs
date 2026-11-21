pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VoiceAgentErrorEventSource {
    Stt,
    Llm,
    Tts,
    RealtimeModel,
    InterruptionDetection,
    Agent,
    Worker,
    Transport,
    AudioInterface,
    Rest,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VoiceAgentErrorEventSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Stt => serializer.serialize_str("stt"),
            Self::Llm => serializer.serialize_str("llm"),
            Self::Tts => serializer.serialize_str("tts"),
            Self::RealtimeModel => serializer.serialize_str("realtime_model"),
            Self::InterruptionDetection => serializer.serialize_str("interruption_detection"),
            Self::Agent => serializer.serialize_str("agent"),
            Self::Worker => serializer.serialize_str("worker"),
            Self::Transport => serializer.serialize_str("transport"),
            Self::AudioInterface => serializer.serialize_str("audio_interface"),
            Self::Rest => serializer.serialize_str("rest"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VoiceAgentErrorEventSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "stt" => Ok(Self::Stt),
            "llm" => Ok(Self::Llm),
            "tts" => Ok(Self::Tts),
            "realtime_model" => Ok(Self::RealtimeModel),
            "interruption_detection" => Ok(Self::InterruptionDetection),
            "agent" => Ok(Self::Agent),
            "worker" => Ok(Self::Worker),
            "transport" => Ok(Self::Transport),
            "audio_interface" => Ok(Self::AudioInterface),
            "rest" => Ok(Self::Rest),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VoiceAgentErrorEventSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stt => write!(f, "stt"),
            Self::Llm => write!(f, "llm"),
            Self::Tts => write!(f, "tts"),
            Self::RealtimeModel => write!(f, "realtime_model"),
            Self::InterruptionDetection => write!(f, "interruption_detection"),
            Self::Agent => write!(f, "agent"),
            Self::Worker => write!(f, "worker"),
            Self::Transport => write!(f, "transport"),
            Self::AudioInterface => write!(f, "audio_interface"),
            Self::Rest => write!(f, "rest"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
