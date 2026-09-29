pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowExecutionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_tracing_attributes: Option<HashMap<String, Option<String>>>,
    /// Name of the deployment to route this execution to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_name: Option<String>,
    /// Allows you to specify a custom execution ID. If not provided, a random ID will be generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    /// Plugin-specific data to propagate into WorkflowContext.extensions at execution time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    /// If true, ignore the caller's trace context and start a new, independent trace for this execution instead of joining the caller's trace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_new_trace: Option<bool>,
    /// The input to the workflow. This should be a dictionary or a BaseModel that matches the workflow's input schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<HashMap<String, serde_json::Value>>,
    /// Deprecated. Use deployment_name instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_queue: Option<String>,
    /// Maximum time to wait for completion when wait_for_result is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<f64>,
    /// If true, wait for the workflow to complete and return the result directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_result: Option<bool>,
}

impl WorkflowExecutionRequest {
    pub fn builder() -> WorkflowExecutionRequestBuilder {
        <WorkflowExecutionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowExecutionRequestBuilder {
    custom_tracing_attributes: Option<HashMap<String, Option<String>>>,
    deployment_name: Option<String>,
    execution_id: Option<String>,
    extensions: Option<HashMap<String, serde_json::Value>>,
    force_new_trace: Option<bool>,
    input: Option<HashMap<String, serde_json::Value>>,
    task_queue: Option<String>,
    timeout_seconds: Option<f64>,
    wait_for_result: Option<bool>,
}

impl WorkflowExecutionRequestBuilder {
    pub fn custom_tracing_attributes(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.custom_tracing_attributes = Some(value);
        self
    }

    pub fn deployment_name(mut self, value: impl Into<String>) -> Self {
        self.deployment_name = Some(value.into());
        self
    }

    pub fn execution_id(mut self, value: impl Into<String>) -> Self {
        self.execution_id = Some(value.into());
        self
    }

    pub fn extensions(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.extensions = Some(value);
        self
    }

    pub fn force_new_trace(mut self, value: bool) -> Self {
        self.force_new_trace = Some(value);
        self
    }

    pub fn input(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input = Some(value);
        self
    }

    pub fn task_queue(mut self, value: impl Into<String>) -> Self {
        self.task_queue = Some(value.into());
        self
    }

    pub fn timeout_seconds(mut self, value: f64) -> Self {
        self.timeout_seconds = Some(value);
        self
    }

    pub fn wait_for_result(mut self, value: bool) -> Self {
        self.wait_for_result = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowExecutionRequest`].
    pub fn build(self) -> Result<WorkflowExecutionRequest, BuildError> {
        Ok(WorkflowExecutionRequest {
            custom_tracing_attributes: self.custom_tracing_attributes,
            deployment_name: self.deployment_name,
            execution_id: self.execution_id,
            extensions: self.extensions,
            force_new_trace: self.force_new_trace,
            input: self.input,
            task_queue: self.task_queue,
            timeout_seconds: self.timeout_seconds,
            wait_for_result: self.wait_for_result,
        })
    }
}
