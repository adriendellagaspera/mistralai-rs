pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ThinkChunkThinkingItem {
    #[serde(rename = "text")]
    #[non_exhaustive]
    Text {
        #[serde(flatten)]
        data: TextChunk,
    },

    #[serde(rename = "tool_reference")]
    #[non_exhaustive]
    ToolReference {
        #[serde(flatten)]
        data: ToolReferenceChunk,
    },

    #[serde(rename = "reference")]
    #[non_exhaustive]
    Reference {
        #[serde(flatten)]
        data: ReferenceChunk,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ThinkChunkThinkingItem {
    pub fn text(data: TextChunk) -> Self {
        Self::Text { data }
    }

    pub fn tool_reference(data: ToolReferenceChunk) -> Self {
        Self::ToolReference { data }
    }

    pub fn reference(data: ReferenceChunk) -> Self {
        Self::Reference { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
