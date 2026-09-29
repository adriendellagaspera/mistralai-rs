pub use crate::prelude::*;

/// Query parameters for libraries_documents_get_text_content_v1
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LibrariesDocumentsGetTextContentV1QueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_end: Option<i64>,
}

impl LibrariesDocumentsGetTextContentV1QueryRequest {
    pub fn builder() -> LibrariesDocumentsGetTextContentV1QueryRequestBuilder {
        <LibrariesDocumentsGetTextContentV1QueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LibrariesDocumentsGetTextContentV1QueryRequestBuilder {
    page_start: Option<i64>,
    page_end: Option<i64>,
}

impl LibrariesDocumentsGetTextContentV1QueryRequestBuilder {
    pub fn page_start(mut self, value: i64) -> Self {
        self.page_start = Some(value);
        self
    }

    pub fn page_end(mut self, value: i64) -> Self {
        self.page_end = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LibrariesDocumentsGetTextContentV1QueryRequest`].
    pub fn build(self) -> Result<LibrariesDocumentsGetTextContentV1QueryRequest, BuildError> {
        Ok(LibrariesDocumentsGetTextContentV1QueryRequest {
            page_start: self.page_start,
            page_end: self.page_end,
        })
    }
}
