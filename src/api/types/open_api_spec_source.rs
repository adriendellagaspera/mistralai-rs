pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum OpenApiSpecSource {
    #[serde(rename = "file")]
    #[non_exhaustive]
    File {
        #[serde(default)]
        filename: String,
    },

    #[serde(rename = "url")]
    #[non_exhaustive]
    Url {
        #[serde(default)]
        url: String,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl OpenApiSpecSource {
    pub fn file(filename: String) -> Self {
        Self::File { filename }
    }

    pub fn url(url: String) -> Self {
        Self::Url { url }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
