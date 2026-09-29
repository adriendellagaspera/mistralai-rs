pub use crate::prelude::*;

/// Query parameters for jobs_api_routes_batch_get_batch_jobs
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct JobsApiRoutesBatchGetBatchJobsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<BatchJobStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<JobsApiRoutesBatchGetBatchJobsBatchRequestOrderBy>,
}

impl JobsApiRoutesBatchGetBatchJobsQueryRequest {
    pub fn builder() -> JobsApiRoutesBatchGetBatchJobsQueryRequestBuilder {
        <JobsApiRoutesBatchGetBatchJobsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JobsApiRoutesBatchGetBatchJobsQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    model: Option<String>,
    agent_id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    created_after: Option<DateTime<FixedOffset>>,
    created_by_me: Option<bool>,
    status: Option<Vec<BatchJobStatus>>,
    order_by: Option<JobsApiRoutesBatchGetBatchJobsBatchRequestOrderBy>,
}

impl JobsApiRoutesBatchGetBatchJobsQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn created_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_after = Some(value);
        self
    }

    pub fn created_by_me(mut self, value: bool) -> Self {
        self.created_by_me = Some(value);
        self
    }

    pub fn status(mut self, value: Vec<BatchJobStatus>) -> Self {
        self.status = Some(value);
        self
    }

    pub fn order_by(mut self, value: JobsApiRoutesBatchGetBatchJobsBatchRequestOrderBy) -> Self {
        self.order_by = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JobsApiRoutesBatchGetBatchJobsQueryRequest`].
    pub fn build(self) -> Result<JobsApiRoutesBatchGetBatchJobsQueryRequest, BuildError> {
        Ok(JobsApiRoutesBatchGetBatchJobsQueryRequest {
            page: self.page,
            page_size: self.page_size,
            model: self.model,
            agent_id: self.agent_id,
            metadata: self.metadata,
            created_after: self.created_after,
            created_by_me: self.created_by_me,
            status: self.status,
            order_by: self.order_by,
        })
    }
}
