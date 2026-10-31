pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteDocumentsRequest {
    #[serde(default)]
    pub document_ids: Vec<String>,
}

impl DeleteDocumentsRequest {
    pub fn builder() -> DeleteDocumentsRequestBuilder {
        <DeleteDocumentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteDocumentsRequestBuilder {
    document_ids: Option<Vec<String>>,
}

impl DeleteDocumentsRequestBuilder {
    pub fn document_ids(mut self, value: Vec<String>) -> Self {
        self.document_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteDocumentsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_ids`](DeleteDocumentsRequestBuilder::document_ids)
    pub fn build(self) -> Result<DeleteDocumentsRequest, BuildError> {
        Ok(DeleteDocumentsRequest {
            document_ids: self
                .document_ids
                .ok_or_else(|| BuildError::missing_field("document_ids"))?,
        })
    }
}
