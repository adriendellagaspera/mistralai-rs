#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct WorkflowUpdateRequest {
    ///Whether to make the workflow available in the chat assistant
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub available_in_chat_assistant: Option<Option<bool>>,
    ///New description value
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub description: Option<Option<String>>,
    ///New display name value
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub display_name: Option<Option<String>>,
}
