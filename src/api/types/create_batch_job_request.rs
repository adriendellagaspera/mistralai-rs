pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateBatchJobRequest {
    /// A list of `.jsonl` files for batch inference.
    /// Each line must be a JSON object with a `body` field containing the request payload:
    /// ```json
    /// {"custom_id": "0", "body": {"max_tokens": 100, "messages": [{"role": "user", "content": "What is the best French cheese?"}]}}
    /// {"custom_id": "1", "body": {"max_tokens": 100, "messages": [{"role": "user", "content": "What is the best French wine?"}]}}
    /// ```
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_files: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<Vec<BatchRequest>>,
    /// The endpoint to be used for batch inference.
    pub endpoint: ApiEndpoint,
    /// The model to be used for batch inference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// In case you want to use a specific agent from the **deprecated** agents api for batch inference, you can specify the agent ID here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// The metadata of your choice to be associated with the batch inference job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<String>>>,
    /// The timeout in hours for the batch inference job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_hours: Option<i64>,
}

impl CreateBatchJobRequest {
    pub fn builder() -> CreateBatchJobRequestBuilder {
        <CreateBatchJobRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBatchJobRequestBuilder {
    input_files: Option<Vec<String>>,
    requests: Option<Vec<BatchRequest>>,
    endpoint: Option<ApiEndpoint>,
    model: Option<String>,
    agent_id: Option<String>,
    metadata: Option<HashMap<String, Option<String>>>,
    timeout_hours: Option<i64>,
}

impl CreateBatchJobRequestBuilder {
    pub fn input_files(mut self, value: Vec<String>) -> Self {
        self.input_files = Some(value);
        self
    }

    pub fn requests(mut self, value: Vec<BatchRequest>) -> Self {
        self.requests = Some(value);
        self
    }

    pub fn endpoint(mut self, value: ApiEndpoint) -> Self {
        self.endpoint = Some(value);
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

    pub fn metadata(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn timeout_hours(mut self, value: i64) -> Self {
        self.timeout_hours = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateBatchJobRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`endpoint`](CreateBatchJobRequestBuilder::endpoint)
    pub fn build(self) -> Result<CreateBatchJobRequest, BuildError> {
        Ok(CreateBatchJobRequest {
            input_files: self.input_files,
            requests: self.requests,
            endpoint: self
                .endpoint
                .ok_or_else(|| BuildError::missing_field("endpoint"))?,
            model: self.model,
            agent_id: self.agent_id,
            metadata: self.metadata,
            timeout_hours: self.timeout_hours,
        })
    }
}
