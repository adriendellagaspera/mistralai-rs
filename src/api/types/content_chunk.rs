pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ContentChunk {
    #[serde(rename = "document_url")]
    #[non_exhaustive]
    DocumentUrl {
        #[serde(flatten)]
        data: DocumentUrlChunk,
    },

    #[serde(rename = "file")]
    #[non_exhaustive]
    File {
        #[serde(flatten)]
        data: FileChunk,
    },

    #[serde(rename = "image_url")]
    #[non_exhaustive]
    ImageUrl {
        #[serde(flatten)]
        data: ImageUrlChunk,
    },

    #[serde(rename = "input_audio")]
    #[non_exhaustive]
    InputAudio {
        #[serde(default)]
        input_audio: String,
    },

    #[serde(rename = "reference")]
    #[non_exhaustive]
    Reference {
        #[serde(flatten)]
        data: ReferenceChunk,
    },

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

impl ContentChunk {
    pub fn document_url(data: DocumentUrlChunk) -> Self {
        Self::DocumentUrl { data }
    }

    pub fn file(data: FileChunk) -> Self {
        Self::File { data }
    }

    pub fn image_url(data: ImageUrlChunk) -> Self {
        Self::ImageUrl { data }
    }

    pub fn input_audio(input_audio: String) -> Self {
        Self::InputAudio { input_audio }
    }

    pub fn reference(data: ReferenceChunk) -> Self {
        Self::Reference { data }
    }

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
