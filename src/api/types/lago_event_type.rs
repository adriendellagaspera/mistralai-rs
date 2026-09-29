pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LagoEventType {
    ApiTokens,
    ApiPages,
    ApiAudioSeconds,
    ApiAudioCharacters,
    ApiConnectors,
    ApiLibrariesTokens,
    ApiLibrariesPages,
    ApiLibrariesAudio,
    DeploymentTokens,
    GpuHour,
    ReservedInstance,
    VibeTokens,
    VibeConnectors,
    VibePages,
    VibeAudioSeconds,
    VibeAudioCharacters,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LagoEventType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ApiTokens => serializer.serialize_str("api_tokens"),
            Self::ApiPages => serializer.serialize_str("api_pages"),
            Self::ApiAudioSeconds => serializer.serialize_str("api_audio_seconds"),
            Self::ApiAudioCharacters => serializer.serialize_str("api_audio_characters"),
            Self::ApiConnectors => serializer.serialize_str("api_connectors"),
            Self::ApiLibrariesTokens => serializer.serialize_str("api_libraries_tokens"),
            Self::ApiLibrariesPages => serializer.serialize_str("api_libraries_pages"),
            Self::ApiLibrariesAudio => serializer.serialize_str("api_libraries_audio"),
            Self::DeploymentTokens => serializer.serialize_str("deployment_tokens"),
            Self::GpuHour => serializer.serialize_str("gpu_hour"),
            Self::ReservedInstance => serializer.serialize_str("reserved_instance"),
            Self::VibeTokens => serializer.serialize_str("vibe_tokens"),
            Self::VibeConnectors => serializer.serialize_str("vibe_connectors"),
            Self::VibePages => serializer.serialize_str("vibe_pages"),
            Self::VibeAudioSeconds => serializer.serialize_str("vibe_audio_seconds"),
            Self::VibeAudioCharacters => serializer.serialize_str("vibe_audio_characters"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LagoEventType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "api_tokens" => Ok(Self::ApiTokens),
            "api_pages" => Ok(Self::ApiPages),
            "api_audio_seconds" => Ok(Self::ApiAudioSeconds),
            "api_audio_characters" => Ok(Self::ApiAudioCharacters),
            "api_connectors" => Ok(Self::ApiConnectors),
            "api_libraries_tokens" => Ok(Self::ApiLibrariesTokens),
            "api_libraries_pages" => Ok(Self::ApiLibrariesPages),
            "api_libraries_audio" => Ok(Self::ApiLibrariesAudio),
            "deployment_tokens" => Ok(Self::DeploymentTokens),
            "gpu_hour" => Ok(Self::GpuHour),
            "reserved_instance" => Ok(Self::ReservedInstance),
            "vibe_tokens" => Ok(Self::VibeTokens),
            "vibe_connectors" => Ok(Self::VibeConnectors),
            "vibe_pages" => Ok(Self::VibePages),
            "vibe_audio_seconds" => Ok(Self::VibeAudioSeconds),
            "vibe_audio_characters" => Ok(Self::VibeAudioCharacters),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LagoEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiTokens => write!(f, "api_tokens"),
            Self::ApiPages => write!(f, "api_pages"),
            Self::ApiAudioSeconds => write!(f, "api_audio_seconds"),
            Self::ApiAudioCharacters => write!(f, "api_audio_characters"),
            Self::ApiConnectors => write!(f, "api_connectors"),
            Self::ApiLibrariesTokens => write!(f, "api_libraries_tokens"),
            Self::ApiLibrariesPages => write!(f, "api_libraries_pages"),
            Self::ApiLibrariesAudio => write!(f, "api_libraries_audio"),
            Self::DeploymentTokens => write!(f, "deployment_tokens"),
            Self::GpuHour => write!(f, "gpu_hour"),
            Self::ReservedInstance => write!(f, "reserved_instance"),
            Self::VibeTokens => write!(f, "vibe_tokens"),
            Self::VibeConnectors => write!(f, "vibe_connectors"),
            Self::VibePages => write!(f, "vibe_pages"),
            Self::VibeAudioSeconds => write!(f, "vibe_audio_seconds"),
            Self::VibeAudioCharacters => write!(f, "vibe_audio_characters"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
