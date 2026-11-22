pub use crate::prelude::*;

/// Query parameters for generate_index_summary
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GenerateIndexSummaryQueryRequest {
    /// Regenerate the summary even if one already exists for this language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl GenerateIndexSummaryQueryRequest {
    pub fn builder() -> GenerateIndexSummaryQueryRequestBuilder {
        <GenerateIndexSummaryQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GenerateIndexSummaryQueryRequestBuilder {
    force: Option<bool>,
}

impl GenerateIndexSummaryQueryRequestBuilder {
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GenerateIndexSummaryQueryRequest`].
    pub fn build(self) -> Result<GenerateIndexSummaryQueryRequest, BuildError> {
        Ok(GenerateIndexSummaryQueryRequest { force: self.force })
    }
}
