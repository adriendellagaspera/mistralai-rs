pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MessageOutputEntryContent {
    String(String),

    MessageOutputContentChunks(MessageOutputContentChunks),
}

impl MessageOutputEntryContent {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_message_output_content_chunks(&self) -> bool {
        matches!(self, Self::MessageOutputContentChunks(_))
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

    pub fn as_message_output_content_chunks(&self) -> Option<&MessageOutputContentChunks> {
        match self {
            Self::MessageOutputContentChunks(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_message_output_content_chunks(self) -> Option<MessageOutputContentChunks> {
        match self {
            Self::MessageOutputContentChunks(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for MessageOutputEntryContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::MessageOutputContentChunks(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
