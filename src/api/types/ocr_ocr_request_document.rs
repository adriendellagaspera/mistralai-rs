pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum OcrRequestDocument {
    #[serde(rename = "file")]
    #[non_exhaustive]
    File {
        #[serde(flatten)]
        data: FileChunk,
    },

    #[serde(rename = "document_url")]
    #[non_exhaustive]
    DocumentUrl {
        #[serde(flatten)]
        data: DocumentUrlChunk,
    },

    #[serde(rename = "image_url")]
    #[non_exhaustive]
    ImageUrl {
        #[serde(flatten)]
        data: ImageUrlChunk,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl OcrRequestDocument {
    pub fn file(data: FileChunk) -> Self {
        Self::File { data }
    }

    pub fn document_url(data: DocumentUrlChunk) -> Self {
        Self::DocumentUrl { data }
    }

    pub fn image_url(data: ImageUrlChunk) -> Self {
        Self::ImageUrl { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
