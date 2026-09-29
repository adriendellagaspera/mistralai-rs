pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MessageOutputEventContent {
    String(String),

    OutputContentChunks(OutputContentChunks),
}

impl MessageOutputEventContent {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_output_content_chunks(&self) -> bool {
        matches!(self, Self::OutputContentChunks(_))
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_output_content_chunks(&self) -> Option<&OutputContentChunks> {
        match self {
            Self::OutputContentChunks(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_output_content_chunks(self) -> Option<OutputContentChunks> {
        match self {
            Self::OutputContentChunks(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for MessageOutputEventContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::OutputContentChunks(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
