pub use crate::prelude::*;

/// Request for search_evaluation_run_records_v2_v1_observability_evaluation_runs_v2__run_id__records_search_post (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchEvaluationRunRecordsV2V1ObservabilityEvaluationRunsV2RunIdRecordsSearchPostRequest
{
    #[serde(skip)]
    pub page_size: Option<i64>,
    #[serde(skip)]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<EvaluationRunRecordFilterInput>,
}

impl SearchEvaluationRunRecordsV2V1ObservabilityEvaluationRunsV2RunIdRecordsSearchPostRequest {
    pub fn builder() -> SearchEvaluationRunRecordsV2V1ObservabilityEvaluationRunsV2RunIdRecordsSearchPostRequestBuilder{
        <SearchEvaluationRunRecordsV2V1ObservabilityEvaluationRunsV2RunIdRecordsSearchPostRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchEvaluationRunRecordsV2V1ObservabilityEvaluationRunsV2RunIdRecordsSearchPostRequestBuilder
{
    page_size: Option<i64>,
    page: Option<i64>,
    body: Option<EvaluationRunRecordFilterInput>,
}

impl
    SearchEvaluationRunRecordsV2V1ObservabilityEvaluationRunsV2RunIdRecordsSearchPostRequestBuilder
{
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn body(mut self, value: EvaluationRunRecordFilterInput) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchEvaluationRunRecordsV2V1ObservabilityEvaluationRunsV2RunIdRecordsSearchPostRequest`].
    pub fn build(
        self,
    ) -> Result<
        SearchEvaluationRunRecordsV2V1ObservabilityEvaluationRunsV2RunIdRecordsSearchPostRequest,
        BuildError,
    > {
        Ok(SearchEvaluationRunRecordsV2V1ObservabilityEvaluationRunsV2RunIdRecordsSearchPostRequest {
            page_size: self.page_size,
            page: self.page,
            body: self.body,
        })
    }
}
