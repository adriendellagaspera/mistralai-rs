pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApiEndpoint {
    V1ChatCompletions,
    V1Embeddings,
    V1FimCompletions,
    V1Moderations,
    V1ChatModerations,
    V1Ocr,
    V1Classifications,
    V1ChatClassifications,
    V1Conversations,
    V1AudioTranscriptions,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ApiEndpoint {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::V1ChatCompletions => serializer.serialize_str("/v1/chat/completions"),
            Self::V1Embeddings => serializer.serialize_str("/v1/embeddings"),
            Self::V1FimCompletions => serializer.serialize_str("/v1/fim/completions"),
            Self::V1Moderations => serializer.serialize_str("/v1/moderations"),
            Self::V1ChatModerations => serializer.serialize_str("/v1/chat/moderations"),
            Self::V1Ocr => serializer.serialize_str("/v1/ocr"),
            Self::V1Classifications => serializer.serialize_str("/v1/classifications"),
            Self::V1ChatClassifications => serializer.serialize_str("/v1/chat/classifications"),
            Self::V1Conversations => serializer.serialize_str("/v1/conversations"),
            Self::V1AudioTranscriptions => serializer.serialize_str("/v1/audio/transcriptions"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ApiEndpoint {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "/v1/chat/completions" => Ok(Self::V1ChatCompletions),
            "/v1/embeddings" => Ok(Self::V1Embeddings),
            "/v1/fim/completions" => Ok(Self::V1FimCompletions),
            "/v1/moderations" => Ok(Self::V1Moderations),
            "/v1/chat/moderations" => Ok(Self::V1ChatModerations),
            "/v1/ocr" => Ok(Self::V1Ocr),
            "/v1/classifications" => Ok(Self::V1Classifications),
            "/v1/chat/classifications" => Ok(Self::V1ChatClassifications),
            "/v1/conversations" => Ok(Self::V1Conversations),
            "/v1/audio/transcriptions" => Ok(Self::V1AudioTranscriptions),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ApiEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V1ChatCompletions => write!(f, "/v1/chat/completions"),
            Self::V1Embeddings => write!(f, "/v1/embeddings"),
            Self::V1FimCompletions => write!(f, "/v1/fim/completions"),
            Self::V1Moderations => write!(f, "/v1/moderations"),
            Self::V1ChatModerations => write!(f, "/v1/chat/moderations"),
            Self::V1Ocr => write!(f, "/v1/ocr"),
            Self::V1Classifications => write!(f, "/v1/classifications"),
            Self::V1ChatClassifications => write!(f, "/v1/chat/classifications"),
            Self::V1Conversations => write!(f, "/v1/conversations"),
            Self::V1AudioTranscriptions => write!(f, "/v1/audio/transcriptions"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
