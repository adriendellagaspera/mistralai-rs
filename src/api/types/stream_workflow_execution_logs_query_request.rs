pub use crate::prelude::*;

/// Query parameters for stream_workflow_execution_logs
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StreamWorkflowExecutionLogsQueryRequest {
    /// Filter logs by workflow run ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// Filter logs by activity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    /// Start a fresh stream at this timestamp (ignored when resuming via last_event_id)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<DateTime<FixedOffset>>,
    /// Resume from this cursor (a prior response's SSE id)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_event_id: Option<String>,
}

impl StreamWorkflowExecutionLogsQueryRequest {
    pub fn builder() -> StreamWorkflowExecutionLogsQueryRequestBuilder {
        <StreamWorkflowExecutionLogsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamWorkflowExecutionLogsQueryRequestBuilder {
    run_id: Option<String>,
    activity_id: Option<String>,
    after: Option<DateTime<FixedOffset>>,
    last_event_id: Option<String>,
}

impl StreamWorkflowExecutionLogsQueryRequestBuilder {
    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn activity_id(mut self, value: impl Into<String>) -> Self {
        self.activity_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`StreamWorkflowExecutionLogsQueryRequest`].
    pub fn build(self) -> Result<StreamWorkflowExecutionLogsQueryRequest, BuildError> {
        Ok(StreamWorkflowExecutionLogsQueryRequest {
            run_id: self.run_id,
            activity_id: self.activity_id,
            after: self.after,
            last_event_id: self.last_event_id,
        })
    }
}
