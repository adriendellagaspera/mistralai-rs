pub use crate::prelude::*;

/// Query parameters for get_workflow_execution_logs
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWorkflowExecutionLogsQueryRequest {
    /// Filter logs by workflow run ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// Filter logs by activity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    /// Only return logs at or after this timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<DateTime<FixedOffset>>,
    /// Only return logs before this timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<DateTime<FixedOffset>>,
    /// First-page sort order: 'asc' (oldest first) or 'desc'. Ignored when `cursor` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<GetWorkflowExecutionLogsExecutionsRequestOrder>,
    /// Pagination cursor from a previous response's `next_cursor`; carries the window and order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Maximum number of logs to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl GetWorkflowExecutionLogsQueryRequest {
    pub fn builder() -> GetWorkflowExecutionLogsQueryRequestBuilder {
        <GetWorkflowExecutionLogsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWorkflowExecutionLogsQueryRequestBuilder {
    run_id: Option<String>,
    activity_id: Option<String>,
    after: Option<DateTime<FixedOffset>>,
    before: Option<DateTime<FixedOffset>>,
    order: Option<GetWorkflowExecutionLogsExecutionsRequestOrder>,
    cursor: Option<String>,
    limit: Option<i64>,
}

impl GetWorkflowExecutionLogsQueryRequestBuilder {
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

    pub fn before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.before = Some(value);
        self
    }

    pub fn order(mut self, value: GetWorkflowExecutionLogsExecutionsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWorkflowExecutionLogsQueryRequest`].
    pub fn build(self) -> Result<GetWorkflowExecutionLogsQueryRequest, BuildError> {
        Ok(GetWorkflowExecutionLogsQueryRequest {
            run_id: self.run_id,
            activity_id: self.activity_id,
            after: self.after,
            before: self.before,
            order: self.order,
            cursor: self.cursor,
            limit: self.limit,
        })
    }
}
