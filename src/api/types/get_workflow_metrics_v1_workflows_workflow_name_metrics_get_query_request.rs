pub use crate::prelude::*;

/// Query parameters for get_workflow_metrics_v1_workflows__workflow_name__metrics_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWorkflowMetricsV1WorkflowsWorkflowNameMetricsGetQueryRequest {
    /// Filter workflows started after this time (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<FixedOffset>>,
    /// Filter workflows started before this time (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<FixedOffset>>,
}

impl GetWorkflowMetricsV1WorkflowsWorkflowNameMetricsGetQueryRequest {
    pub fn builder() -> GetWorkflowMetricsV1WorkflowsWorkflowNameMetricsGetQueryRequestBuilder {
        <GetWorkflowMetricsV1WorkflowsWorkflowNameMetricsGetQueryRequestBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWorkflowMetricsV1WorkflowsWorkflowNameMetricsGetQueryRequestBuilder {
    start_time: Option<DateTime<FixedOffset>>,
    end_time: Option<DateTime<FixedOffset>>,
}

impl GetWorkflowMetricsV1WorkflowsWorkflowNameMetricsGetQueryRequestBuilder {
    pub fn start_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWorkflowMetricsV1WorkflowsWorkflowNameMetricsGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<GetWorkflowMetricsV1WorkflowsWorkflowNameMetricsGetQueryRequest, BuildError> {
        Ok(
            GetWorkflowMetricsV1WorkflowsWorkflowNameMetricsGetQueryRequest {
                start_time: self.start_time,
                end_time: self.end_time,
            },
        )
    }
}
