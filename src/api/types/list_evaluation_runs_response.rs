pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListEvaluationRunsResponse {
    #[serde(default)]
    pub runs: PaginatedResultEvaluationRunV2Response,
}

impl ListEvaluationRunsResponse {
    pub fn builder() -> ListEvaluationRunsResponseBuilder {
        <ListEvaluationRunsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEvaluationRunsResponseBuilder {
    runs: Option<PaginatedResultEvaluationRunV2Response>,
}

impl ListEvaluationRunsResponseBuilder {
    pub fn runs(mut self, value: PaginatedResultEvaluationRunV2Response) -> Self {
        self.runs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEvaluationRunsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`runs`](ListEvaluationRunsResponseBuilder::runs)
    pub fn build(self) -> Result<ListEvaluationRunsResponse, BuildError> {
        Ok(ListEvaluationRunsResponse {
            runs: self.runs.ok_or_else(|| BuildError::missing_field("runs"))?,
        })
    }
}
