pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SystemMessageContent {
    String(String),

    SystemMessageContentChunksList(Vec<SystemMessageContentChunks>),
}

impl SystemMessageContent {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_system_message_content_chunks_list(&self) -> bool {
        matches!(self, Self::SystemMessageContentChunksList(_))
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

    pub fn as_system_message_content_chunks_list(
        &self,
    ) -> Option<&Vec<SystemMessageContentChunks>> {
        match self {
            Self::SystemMessageContentChunksList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_system_message_content_chunks_list(
        self,
    ) -> Option<Vec<SystemMessageContentChunks>> {
        match self {
            Self::SystemMessageContentChunksList(value) => Some(value),
            _ => None,
        }
    }
}
