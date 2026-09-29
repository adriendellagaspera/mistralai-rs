pub use crate::prelude::*;

/// Query parameters for jobs_api_routes_batch_get_batch_job
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct JobsApiRoutesBatchGetBatchJobQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<bool>,
}

impl JobsApiRoutesBatchGetBatchJobQueryRequest {
    pub fn builder() -> JobsApiRoutesBatchGetBatchJobQueryRequestBuilder {
        <JobsApiRoutesBatchGetBatchJobQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JobsApiRoutesBatchGetBatchJobQueryRequestBuilder {
    inline: Option<bool>,
}

impl JobsApiRoutesBatchGetBatchJobQueryRequestBuilder {
    pub fn inline(mut self, value: bool) -> Self {
        self.inline = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JobsApiRoutesBatchGetBatchJobQueryRequest`].
    pub fn build(self) -> Result<JobsApiRoutesBatchGetBatchJobQueryRequest, BuildError> {
        Ok(JobsApiRoutesBatchGetBatchJobQueryRequest {
            inline: self.inline,
        })
    }
}
