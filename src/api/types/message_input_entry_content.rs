pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MessageInputEntryContent {
    String(String),

    MessageInputContentChunks(MessageInputContentChunks),
}

impl MessageInputEntryContent {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_message_input_content_chunks(&self) -> bool {
        matches!(self, Self::MessageInputContentChunks(_))
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

    pub fn as_message_input_content_chunks(&self) -> Option<&MessageInputContentChunks> {
        match self {
            Self::MessageInputContentChunks(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_message_input_content_chunks(self) -> Option<MessageInputContentChunks> {
        match self {
            Self::MessageInputContentChunks(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for MessageInputEntryContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::MessageInputContentChunks(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
