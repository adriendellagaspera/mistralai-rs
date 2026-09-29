pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum MessageOutputContentChunksItem {
    #[serde(rename = "text")]
    #[non_exhaustive]
    Text {
        #[serde(flatten)]
        data: TextChunk,
    },

    #[serde(rename = "image_url")]
    #[non_exhaustive]
    ImageUrl {
        #[serde(flatten)]
        data: ImageUrlChunk,
    },

    #[serde(rename = "tool_file")]
    #[non_exhaustive]
    ToolFile {
        #[serde(flatten)]
        data: ToolFileChunk,
    },

    #[serde(rename = "document_url")]
    #[non_exhaustive]
    DocumentUrl {
        #[serde(flatten)]
        data: DocumentUrlChunk,
    },

    #[serde(rename = "thinking")]
    #[non_exhaustive]
    Thinking {
        #[serde(flatten)]
        data: ThinkChunk,
    },

    #[serde(rename = "tool_reference")]
    #[non_exhaustive]
    ToolReference {
        #[serde(flatten)]
        data: ToolReferenceChunk,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl MessageOutputContentChunksItem {
    pub fn text(data: TextChunk) -> Self {
        Self::Text { data }
    }

    pub fn image_url(data: ImageUrlChunk) -> Self {
        Self::ImageUrl { data }
    }

    pub fn tool_file(data: ToolFileChunk) -> Self {
        Self::ToolFile { data }
    }

    pub fn document_url(data: DocumentUrlChunk) -> Self {
        Self::DocumentUrl { data }
    }

    pub fn thinking(data: ThinkChunk) -> Self {
        Self::Thinking { data }
    }

    pub fn tool_reference(data: ToolReferenceChunk) -> Self {
        Self::ToolReference { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
