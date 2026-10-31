pub use crate::prelude::*;

/// Request for search_evaluation_runs_v2_v1_observability_evaluations_v2__evaluation_slug__runs_search_post (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchEvaluationRunsV2V1ObservabilityEvaluationsV2EvaluationSlugRunsSearchPostRequest {
    #[serde(skip)]
    pub page_size: Option<i64>,
    #[serde(skip)]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<EvaluationRunFilterInput>,
}

impl SearchEvaluationRunsV2V1ObservabilityEvaluationsV2EvaluationSlugRunsSearchPostRequest {
    pub fn builder(
    ) -> SearchEvaluationRunsV2V1ObservabilityEvaluationsV2EvaluationSlugRunsSearchPostRequestBuilder
    {
        <SearchEvaluationRunsV2V1ObservabilityEvaluationsV2EvaluationSlugRunsSearchPostRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchEvaluationRunsV2V1ObservabilityEvaluationsV2EvaluationSlugRunsSearchPostRequestBuilder
{
    page_size: Option<i64>,
    page: Option<i64>,
    body: Option<EvaluationRunFilterInput>,
}

impl SearchEvaluationRunsV2V1ObservabilityEvaluationsV2EvaluationSlugRunsSearchPostRequestBuilder {
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

    /// Consumes the builder and constructs a [`SearchEvaluationRunsV2V1ObservabilityEvaluationsV2EvaluationSlugRunsSearchPostRequest`].
    pub fn build(
        self,
    ) -> Result<
        SearchEvaluationRunsV2V1ObservabilityEvaluationsV2EvaluationSlugRunsSearchPostRequest,
        BuildError,
    > {
        Ok(
            SearchEvaluationRunsV2V1ObservabilityEvaluationsV2EvaluationSlugRunsSearchPostRequest {
                page_size: self.page_size,
                page: self.page,
                body: self.body,
            },
        )
    }
}
