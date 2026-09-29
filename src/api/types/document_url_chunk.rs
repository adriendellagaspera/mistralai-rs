pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentUrlChunk {
    /// The filename of the document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    #[serde(default)]
    pub document_url: String,
}

impl DocumentUrlChunk {
    pub fn builder() -> DocumentUrlChunkBuilder {
        <DocumentUrlChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentUrlChunkBuilder {
    document_name: Option<String>,
    document_url: Option<String>,
}

impl DocumentUrlChunkBuilder {
    pub fn document_name(mut self, value: impl Into<String>) -> Self {
        self.document_name = Some(value.into());
        self
    }

    pub fn document_url(mut self, value: impl Into<String>) -> Self {
        self.document_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DocumentUrlChunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_url`](DocumentUrlChunkBuilder::document_url)
    pub fn build(self) -> Result<DocumentUrlChunk, BuildError> {
        Ok(DocumentUrlChunk {
            document_name: self.document_name,
            document_url: self
                .document_url
                .ok_or_else(|| BuildError::missing_field("document_url"))?,
        })
    }
}
