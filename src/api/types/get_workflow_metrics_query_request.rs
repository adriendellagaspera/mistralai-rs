pub use crate::prelude::*;

/// Query parameters for get_workflow_metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWorkflowMetricsQueryRequest {
    /// Filter workflows started after this time (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<FixedOffset>>,
    /// Filter workflows started before this time (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<FixedOffset>>,
}

impl GetWorkflowMetricsQueryRequest {
    pub fn builder() -> GetWorkflowMetricsQueryRequestBuilder {
        <GetWorkflowMetricsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWorkflowMetricsQueryRequestBuilder {
    start_time: Option<DateTime<FixedOffset>>,
    end_time: Option<DateTime<FixedOffset>>,
}

impl GetWorkflowMetricsQueryRequestBuilder {
    pub fn start_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWorkflowMetricsQueryRequest`].
    pub fn build(self) -> Result<GetWorkflowMetricsQueryRequest, BuildError> {
        Ok(GetWorkflowMetricsQueryRequest {
            start_time: self.start_time,
            end_time: self.end_time,
        })
    }
}
