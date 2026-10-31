pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct IngestDocumentsResponse {
    #[serde(default)]
    pub accepted: i64,
    #[serde(default)]
    pub rejected: i64,
    #[serde(default)]
    pub results: Vec<IngestDocumentsResponseResultsItem>,
}

impl IngestDocumentsResponse {
    pub fn builder() -> IngestDocumentsResponseBuilder {
        <IngestDocumentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IngestDocumentsResponseBuilder {
    accepted: Option<i64>,
    rejected: Option<i64>,
    results: Option<Vec<IngestDocumentsResponseResultsItem>>,
}

impl IngestDocumentsResponseBuilder {
    pub fn accepted(mut self, value: i64) -> Self {
        self.accepted = Some(value);
        self
    }

    pub fn rejected(mut self, value: i64) -> Self {
        self.rejected = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<IngestDocumentsResponseResultsItem>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IngestDocumentsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accepted`](IngestDocumentsResponseBuilder::accepted)
    /// - [`rejected`](IngestDocumentsResponseBuilder::rejected)
    /// - [`results`](IngestDocumentsResponseBuilder::results)
    pub fn build(self) -> Result<IngestDocumentsResponse, BuildError> {
        Ok(IngestDocumentsResponse {
            accepted: self
                .accepted
                .ok_or_else(|| BuildError::missing_field("accepted"))?,
            rejected: self
                .rejected
                .ok_or_else(|| BuildError::missing_field("rejected"))?,
            results: self
                .results
                .ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
