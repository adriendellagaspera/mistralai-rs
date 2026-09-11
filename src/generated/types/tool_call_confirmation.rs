#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolCallConfirmation {
    pub confirmation: ToolCallConfirmationConfirmation,
    pub tool_call_id: String,
}
