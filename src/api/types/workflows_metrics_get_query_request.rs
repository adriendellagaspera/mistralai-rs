pub use crate::prelude::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowsMetricsGetQueryRequest {
    /// Filter workflows started after this time (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<FixedOffset>>,
    /// Filter workflows started before this time (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<FixedOffset>>,
}

impl WorkflowsMetricsGetQueryRequest {
    pub fn builder() -> WorkflowsMetricsGetQueryRequestBuilder {
        <WorkflowsMetricsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowsMetricsGetQueryRequestBuilder {
    start_time: Option<DateTime<FixedOffset>>,
    end_time: Option<DateTime<FixedOffset>>,
}

impl WorkflowsMetricsGetQueryRequestBuilder {
    pub fn start_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowsMetricsGetQueryRequest`].
    pub fn build(self) -> Result<WorkflowsMetricsGetQueryRequest, BuildError> {
        Ok(WorkflowsMetricsGetQueryRequest {
            start_time: self.start_time,
            end_time: self.end_time,
        })
    }
}
