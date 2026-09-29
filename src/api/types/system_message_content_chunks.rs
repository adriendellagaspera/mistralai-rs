pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum SystemMessageContentChunks {
    #[serde(rename = "text")]
    #[non_exhaustive]
    Text {
        #[serde(flatten)]
        data: TextChunk,
    },

    #[serde(rename = "thinking")]
    #[non_exhaustive]
    Thinking {
        #[serde(flatten)]
        data: ThinkChunk,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl SystemMessageContentChunks {
    pub fn text(data: TextChunk) -> Self {
        Self::Text { data }
    }

    pub fn thinking(data: ThinkChunk) -> Self {
        Self::Thinking { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
