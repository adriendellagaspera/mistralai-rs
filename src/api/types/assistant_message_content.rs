pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AssistantMessageContent {
    String(String),

    ContentChunkList(Vec<ContentChunk>),
}

impl AssistantMessageContent {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_content_chunk_list(&self) -> bool {
        matches!(self, Self::ContentChunkList(_))
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

    pub fn as_content_chunk_list(&self) -> Option<&Vec<ContentChunk>> {
        match self {
            Self::ContentChunkList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_content_chunk_list(self) -> Option<Vec<ContentChunk>> {
        match self {
            Self::ContentChunkList(value) => Some(value),
            _ => None,
        }
    }
}
