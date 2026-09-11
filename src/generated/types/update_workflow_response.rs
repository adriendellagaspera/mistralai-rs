#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateWorkflowResponse {
    ///The result of the Update workflow call
    pub result: serde_json::Value,
    pub update_name: String,
}
