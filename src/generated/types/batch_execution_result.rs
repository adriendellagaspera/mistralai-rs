#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BatchExecutionResult {
    ///Error message if operation failed
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub error: Option<Option<String>>,
    ///Status of the operation (success/failure)
    pub status: String,
}
