pub use crate::prelude::*;

/// Binary contents of a resource.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BlobResourceContents {
    #[serde(rename = "_meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub blob: String,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub uri: String,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl BlobResourceContents {
    pub fn builder() -> BlobResourceContentsBuilder {
        <BlobResourceContentsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BlobResourceContentsBuilder {
    meta: Option<HashMap<String, serde_json::Value>>,
    blob: Option<String>,
    mime_type: Option<String>,
    uri: Option<String>,
}

impl BlobResourceContentsBuilder {
    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn blob(mut self, value: impl Into<String>) -> Self {
        self.blob = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn uri(mut self, value: impl Into<String>) -> Self {
        self.uri = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BlobResourceContents`].
    /// This method will fail if any of the following fields are not set:
    /// - [`blob`](BlobResourceContentsBuilder::blob)
    /// - [`uri`](BlobResourceContentsBuilder::uri)
    pub fn build(self) -> Result<BlobResourceContents, BuildError> {
        Ok(BlobResourceContents {
            meta: self.meta,
            blob: self.blob.ok_or_else(|| BuildError::missing_field("blob"))?,
            mime_type: self.mime_type,
            uri: self.uri.ok_or_else(|| BuildError::missing_field("uri"))?,
            extra: Default::default(),
        })
    }
}
