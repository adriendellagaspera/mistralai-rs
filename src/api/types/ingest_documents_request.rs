pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct IngestDocumentsRequest {
    #[serde(default)]
    pub documents: Vec<HashMap<String, serde_json::Value>>,
}

impl IngestDocumentsRequest {
    pub fn builder() -> IngestDocumentsRequestBuilder {
        <IngestDocumentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IngestDocumentsRequestBuilder {
    documents: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl IngestDocumentsRequestBuilder {
    pub fn documents(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.documents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IngestDocumentsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`documents`](IngestDocumentsRequestBuilder::documents)
    pub fn build(self) -> Result<IngestDocumentsRequest, BuildError> {
        Ok(IngestDocumentsRequest {
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
        })
    }
}
