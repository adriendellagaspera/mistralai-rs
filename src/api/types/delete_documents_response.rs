pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteDocumentsResponse {
    #[serde(default)]
    pub deleted_document_ids: Vec<String>,
    #[serde(default)]
    pub missing: Vec<String>,
}

impl DeleteDocumentsResponse {
    pub fn builder() -> DeleteDocumentsResponseBuilder {
        <DeleteDocumentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteDocumentsResponseBuilder {
    deleted_document_ids: Option<Vec<String>>,
    missing: Option<Vec<String>>,
}

impl DeleteDocumentsResponseBuilder {
    pub fn deleted_document_ids(mut self, value: Vec<String>) -> Self {
        self.deleted_document_ids = Some(value);
        self
    }

    pub fn missing(mut self, value: Vec<String>) -> Self {
        self.missing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteDocumentsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted_document_ids`](DeleteDocumentsResponseBuilder::deleted_document_ids)
    /// - [`missing`](DeleteDocumentsResponseBuilder::missing)
    pub fn build(self) -> Result<DeleteDocumentsResponse, BuildError> {
        Ok(DeleteDocumentsResponse {
            deleted_document_ids: self
                .deleted_document_ids
                .ok_or_else(|| BuildError::missing_field("deleted_document_ids"))?,
            missing: self
                .missing
                .ok_or_else(|| BuildError::missing_field("missing"))?,
        })
    }
}
