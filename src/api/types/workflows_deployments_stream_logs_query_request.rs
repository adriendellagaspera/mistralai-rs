pub use crate::prelude::*;

/// Query parameters for stream_logs
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowsDeploymentsStreamLogsQueryRequest {
    /// Filter logs by worker name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_name: Option<String>,
    /// Filter logs by workflow name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
    /// Start a fresh stream at this timestamp (ignored when resuming via last_event_id)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<DateTime<FixedOffset>>,
    /// Resume from this cursor (a prior response's SSE id)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_event_id: Option<String>,
}

impl WorkflowsDeploymentsStreamLogsQueryRequest {
    pub fn builder() -> WorkflowsDeploymentsStreamLogsQueryRequestBuilder {
        <WorkflowsDeploymentsStreamLogsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowsDeploymentsStreamLogsQueryRequestBuilder {
    worker_name: Option<String>,
    workflow_name: Option<String>,
    after: Option<DateTime<FixedOffset>>,
    last_event_id: Option<String>,
}

impl WorkflowsDeploymentsStreamLogsQueryRequestBuilder {
    pub fn worker_name(mut self, value: impl Into<String>) -> Self {
        self.worker_name = Some(value.into());
        self
    }

    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());
        self
    }

    pub fn after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.after = Some(value);
        self
    }

    pub fn last_event_id(mut self, value: impl Into<String>) -> Self {
        self.last_event_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowsDeploymentsStreamLogsQueryRequest`].
    pub fn build(self) -> Result<WorkflowsDeploymentsStreamLogsQueryRequest, BuildError> {
        Ok(WorkflowsDeploymentsStreamLogsQueryRequest {
            worker_name: self.worker_name,
            workflow_name: self.workflow_name,
            after: self.after,
            last_event_id: self.last_event_id,
        })
    }
}
