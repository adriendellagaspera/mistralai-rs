#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct WorkflowExecutionRequest {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub custom_tracing_attributes: Option<Option<WorkflowExecutionRequestCustomTracingAttributes>>,
    ///Name of the deployment to route this execution to
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub deployment_name: Option<Option<String>>,
    ///Encoded input to the workflow, used when payload encoding is enabled.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub encoded_input: Option<Option<NetworkEncodedInput>>,
    ///Allows you to specify a custom execution ID. If not provided, a random ID will be generated.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub execution_id: Option<Option<String>>,
    ///The input to the workflow. This should be a dictionary that matches the workflow's input schema.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub input: Option<Option<WorkflowExecutionRequestInput>>,
    ///Deprecated. Use deployment_name instead.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub task_queue: Option<Option<String>>,
    ///Maximum time to wait for completion when wait_for_result is true.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub timeout_seconds: Option<Option<f64>>,
    ///If true, wait for the workflow to complete and return the result directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_result: Option<bool>,
}
