pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ResourceChunkContent {
    #[serde(rename = "image_url")]
    #[non_exhaustive]
    ImageUrl {
        #[serde(flatten)]
        data: ImageUrlChunk,
    },

    #[serde(rename = "text")]
    #[non_exhaustive]
    Text {
        #[serde(flatten)]
        data: TextChunk,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ResourceChunkContent {
    pub fn image_url(data: ImageUrlChunk) -> Self {
        Self::ImageUrl { data }
    }

    pub fn text(data: TextChunk) -> Self {
        Self::Text { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
