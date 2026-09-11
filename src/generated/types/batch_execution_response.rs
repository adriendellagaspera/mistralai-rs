#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct BatchExecutionResponse {
    ///Mapping of execution_id to result with status and optional error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<BatchExecutionResponseResults>,
}
