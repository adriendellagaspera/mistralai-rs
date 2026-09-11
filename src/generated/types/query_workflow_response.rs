#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryWorkflowResponse {
    pub query_name: String,
    ///The result of the Query workflow call
    pub result: serde_json::Value,
}
