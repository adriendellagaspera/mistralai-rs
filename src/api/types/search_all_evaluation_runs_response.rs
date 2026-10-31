pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchAllEvaluationRunsResponse {
    #[serde(default)]
    pub runs: PaginatedResultEvaluationRunV2WithRefsResponse,
}

impl SearchAllEvaluationRunsResponse {
    pub fn builder() -> SearchAllEvaluationRunsResponseBuilder {
        <SearchAllEvaluationRunsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchAllEvaluationRunsResponseBuilder {
    runs: Option<PaginatedResultEvaluationRunV2WithRefsResponse>,
}

impl SearchAllEvaluationRunsResponseBuilder {
    pub fn runs(mut self, value: PaginatedResultEvaluationRunV2WithRefsResponse) -> Self {
        self.runs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchAllEvaluationRunsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`runs`](SearchAllEvaluationRunsResponseBuilder::runs)
    pub fn build(self) -> Result<SearchAllEvaluationRunsResponse, BuildError> {
        Ok(SearchAllEvaluationRunsResponse {
            runs: self.runs.ok_or_else(|| BuildError::missing_field("runs"))?,
        })
    }
}
