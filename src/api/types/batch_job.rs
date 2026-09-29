pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchJob {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
    #[serde(default)]
    pub completed_requests: i64,
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_file: Option<String>,
    #[serde(default)]
    pub errors: Vec<BatchError>,
    #[serde(default)]
    pub failed_requests: i64,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub input_files: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<BatchJobObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    pub status: BatchJobStatus,
    #[serde(default)]
    pub succeeded_requests: i64,
    #[serde(default)]
    pub total_requests: i64,
}

impl BatchJob {
    pub fn builder() -> BatchJobBuilder {
        <BatchJobBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchJobBuilder {
    agent_id: Option<String>,
    completed_at: Option<i64>,
    completed_requests: Option<i64>,
    created_at: Option<i64>,
    endpoint: Option<String>,
    error_file: Option<String>,
    errors: Option<Vec<BatchError>>,
    failed_requests: Option<i64>,
    id: Option<String>,
    input_files: Option<Vec<String>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    model: Option<String>,
    object: Option<BatchJobObject>,
    output_file: Option<String>,
    outputs: Option<Vec<HashMap<String, serde_json::Value>>>,
    started_at: Option<i64>,
    status: Option<BatchJobStatus>,
    succeeded_requests: Option<i64>,
    total_requests: Option<i64>,
}

impl BatchJobBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn completed_at(mut self, value: i64) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn completed_requests(mut self, value: i64) -> Self {
        self.completed_requests = Some(value);
        self
    }

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn endpoint(mut self, value: impl Into<String>) -> Self {
        self.endpoint = Some(value.into());
        self
    }

    pub fn error_file(mut self, value: impl Into<String>) -> Self {
        self.error_file = Some(value.into());
        self
    }

    pub fn errors(mut self, value: Vec<BatchError>) -> Self {
        self.errors = Some(value);
        self
    }

    pub fn failed_requests(mut self, value: i64) -> Self {
        self.failed_requests = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn input_files(mut self, value: Vec<String>) -> Self {
        self.input_files = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn object(mut self, value: BatchJobObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn output_file(mut self, value: impl Into<String>) -> Self {
        self.output_file = Some(value.into());
        self
    }

    pub fn outputs(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.outputs = Some(value);
        self
    }

    pub fn started_at(mut self, value: i64) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn status(mut self, value: BatchJobStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn succeeded_requests(mut self, value: i64) -> Self {
        self.succeeded_requests = Some(value);
        self
    }

    pub fn total_requests(mut self, value: i64) -> Self {
        self.total_requests = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchJob`].
    /// This method will fail if any of the following fields are not set:
    /// - [`completed_requests`](BatchJobBuilder::completed_requests)
    /// - [`created_at`](BatchJobBuilder::created_at)
    /// - [`endpoint`](BatchJobBuilder::endpoint)
    /// - [`errors`](BatchJobBuilder::errors)
    /// - [`failed_requests`](BatchJobBuilder::failed_requests)
    /// - [`id`](BatchJobBuilder::id)
    /// - [`input_files`](BatchJobBuilder::input_files)
    /// - [`status`](BatchJobBuilder::status)
    /// - [`succeeded_requests`](BatchJobBuilder::succeeded_requests)
    /// - [`total_requests`](BatchJobBuilder::total_requests)
    pub fn build(self) -> Result<BatchJob, BuildError> {
        Ok(BatchJob {
            agent_id: self.agent_id,
            completed_at: self.completed_at,
            completed_requests: self
                .completed_requests
                .ok_or_else(|| BuildError::missing_field("completed_requests"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            endpoint: self
                .endpoint
                .ok_or_else(|| BuildError::missing_field("endpoint"))?,
            error_file: self.error_file,
            errors: self
                .errors
                .ok_or_else(|| BuildError::missing_field("errors"))?,
            failed_requests: self
                .failed_requests
                .ok_or_else(|| BuildError::missing_field("failed_requests"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            input_files: self
                .input_files
                .ok_or_else(|| BuildError::missing_field("input_files"))?,
            metadata: self.metadata,
            model: self.model,
            object: self.object,
            output_file: self.output_file,
            outputs: self.outputs,
            started_at: self.started_at,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            succeeded_requests: self
                .succeeded_requests
                .ok_or_else(|| BuildError::missing_field("succeeded_requests"))?,
            total_requests: self
                .total_requests
                .ok_or_else(|| BuildError::missing_field("total_requests"))?,
        })
    }
}
