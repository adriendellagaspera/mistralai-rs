pub use crate::prelude::*;

/// Query parameters for list_workers
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListWorkersQueryRequest {
    /// Filter by worker activity. active=only active, inactive=only inactive, None=no filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_status: Option<ListWorkersDeploymentsRequestWorkerStatus>,
    /// Maximum number of workers to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor from a previous response's `next_cursor`. Resend `worker_status` unchanged alongside it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListWorkersQueryRequest {
    pub fn builder() -> ListWorkersQueryRequestBuilder {
        <ListWorkersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWorkersQueryRequestBuilder {
    worker_status: Option<ListWorkersDeploymentsRequestWorkerStatus>,
    limit: Option<i64>,
    cursor: Option<String>,
}

impl ListWorkersQueryRequestBuilder {
    pub fn worker_status(mut self, value: ListWorkersDeploymentsRequestWorkerStatus) -> Self {
        self.worker_status = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListWorkersQueryRequest`].
    pub fn build(self) -> Result<ListWorkersQueryRequest, BuildError> {
        Ok(ListWorkersQueryRequest {
            worker_status: self.worker_status,
            limit: self.limit,
            cursor: self.cursor,
        })
    }
}
