pub use crate::prelude::*;

/// Request for search_all_evaluation_runs_v2_v1_observability_evaluation_runs_v2_search_post (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchAllEvaluationRunsV2V1ObservabilityEvaluationRunsV2SearchPostRequest {
    #[serde(skip)]
    pub page_size: Option<i64>,
    #[serde(skip)]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<EvaluationRunFilterInput>,
}

impl SearchAllEvaluationRunsV2V1ObservabilityEvaluationRunsV2SearchPostRequest {
    pub fn builder(
    ) -> SearchAllEvaluationRunsV2V1ObservabilityEvaluationRunsV2SearchPostRequestBuilder {
        <SearchAllEvaluationRunsV2V1ObservabilityEvaluationRunsV2SearchPostRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchAllEvaluationRunsV2V1ObservabilityEvaluationRunsV2SearchPostRequestBuilder {
    page_size: Option<i64>,
    page: Option<i64>,
    body: Option<EvaluationRunFilterInput>,
}

impl SearchAllEvaluationRunsV2V1ObservabilityEvaluationRunsV2SearchPostRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn body(mut self, value: EvaluationRunFilterInput) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchAllEvaluationRunsV2V1ObservabilityEvaluationRunsV2SearchPostRequest`].
    pub fn build(
        self,
    ) -> Result<SearchAllEvaluationRunsV2V1ObservabilityEvaluationRunsV2SearchPostRequest, BuildError>
    {
        Ok(
            SearchAllEvaluationRunsV2V1ObservabilityEvaluationRunsV2SearchPostRequest {
                page_size: self.page_size,
                page: self.page,
                body: self.body,
            },
        )
    }
}
