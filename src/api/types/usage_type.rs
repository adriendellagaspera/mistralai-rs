pub use crate::prelude::*;

/// The UsageType maps a recorded usage to a billable metric in Lago.
///
/// For example `UsageType.COMPLETION` maps to `api_tokens` in Lago. A new UsageType is billed only once it is added to
/// `USAGE_TYPE_TO_LAGO_EVENT_TYPE_MAP` in kazekit.usage, which the job reporting the usage relies on. Types left out of
/// it, such as `WORKFLOW_EXECUTIONS`, are recorded but not reported to Lago.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UsageType {
    Usage,
    Codestral,
    DocumentLibrary,
    DocumentLibraryApi,
    LibrariesPages,
    LibrariesTokens,
    LibrariesAudio,
    Chat,
    Rlhf,
    Ocr,
    AudioSeconds,
    AudioCharacters,
    AudioAgents,
    Connectors,
    WorkflowExecutions,
    Vibe,
    VibeConnectors,
    VibeOcr,
    VibeAudioSeconds,
    VibeAudioCharacters,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UsageType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Usage => serializer.serialize_str("usage"),
            Self::Codestral => serializer.serialize_str("codestral"),
            Self::DocumentLibrary => serializer.serialize_str("document_library"),
            Self::DocumentLibraryApi => serializer.serialize_str("document_library_api"),
            Self::LibrariesPages => serializer.serialize_str("libraries_pages"),
            Self::LibrariesTokens => serializer.serialize_str("libraries_tokens"),
            Self::LibrariesAudio => serializer.serialize_str("libraries_audio"),
            Self::Chat => serializer.serialize_str("chat"),
            Self::Rlhf => serializer.serialize_str("rlhf"),
            Self::Ocr => serializer.serialize_str("ocr"),
            Self::AudioSeconds => serializer.serialize_str("audio_seconds"),
            Self::AudioCharacters => serializer.serialize_str("audio_characters"),
            Self::AudioAgents => serializer.serialize_str("audio_agents"),
            Self::Connectors => serializer.serialize_str("connectors"),
            Self::WorkflowExecutions => serializer.serialize_str("workflow_executions"),
            Self::Vibe => serializer.serialize_str("vibe"),
            Self::VibeConnectors => serializer.serialize_str("vibe_connectors"),
            Self::VibeOcr => serializer.serialize_str("vibe_ocr"),
            Self::VibeAudioSeconds => serializer.serialize_str("vibe_audio_seconds"),
            Self::VibeAudioCharacters => serializer.serialize_str("vibe_audio_characters"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UsageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "usage" => Ok(Self::Usage),
            "codestral" => Ok(Self::Codestral),
            "document_library" => Ok(Self::DocumentLibrary),
            "document_library_api" => Ok(Self::DocumentLibraryApi),
            "libraries_pages" => Ok(Self::LibrariesPages),
            "libraries_tokens" => Ok(Self::LibrariesTokens),
            "libraries_audio" => Ok(Self::LibrariesAudio),
            "chat" => Ok(Self::Chat),
            "rlhf" => Ok(Self::Rlhf),
            "ocr" => Ok(Self::Ocr),
            "audio_seconds" => Ok(Self::AudioSeconds),
            "audio_characters" => Ok(Self::AudioCharacters),
            "audio_agents" => Ok(Self::AudioAgents),
            "connectors" => Ok(Self::Connectors),
            "workflow_executions" => Ok(Self::WorkflowExecutions),
            "vibe" => Ok(Self::Vibe),
            "vibe_connectors" => Ok(Self::VibeConnectors),
            "vibe_ocr" => Ok(Self::VibeOcr),
            "vibe_audio_seconds" => Ok(Self::VibeAudioSeconds),
            "vibe_audio_characters" => Ok(Self::VibeAudioCharacters),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UsageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage => write!(f, "usage"),
            Self::Codestral => write!(f, "codestral"),
            Self::DocumentLibrary => write!(f, "document_library"),
            Self::DocumentLibraryApi => write!(f, "document_library_api"),
            Self::LibrariesPages => write!(f, "libraries_pages"),
            Self::LibrariesTokens => write!(f, "libraries_tokens"),
            Self::LibrariesAudio => write!(f, "libraries_audio"),
            Self::Chat => write!(f, "chat"),
            Self::Rlhf => write!(f, "rlhf"),
            Self::Ocr => write!(f, "ocr"),
            Self::AudioSeconds => write!(f, "audio_seconds"),
            Self::AudioCharacters => write!(f, "audio_characters"),
            Self::AudioAgents => write!(f, "audio_agents"),
            Self::Connectors => write!(f, "connectors"),
            Self::WorkflowExecutions => write!(f, "workflow_executions"),
            Self::Vibe => write!(f, "vibe"),
            Self::VibeConnectors => write!(f, "vibe_connectors"),
            Self::VibeOcr => write!(f, "vibe_ocr"),
            Self::VibeAudioSeconds => write!(f, "vibe_audio_seconds"),
            Self::VibeAudioCharacters => write!(f, "vibe_audio_characters"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
