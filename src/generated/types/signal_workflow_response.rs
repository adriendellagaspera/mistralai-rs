#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SignalWorkflowResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
