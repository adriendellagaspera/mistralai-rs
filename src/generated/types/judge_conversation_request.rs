#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JudgeConversationRequest {
    pub messages: Vec<JudgeConversationRequestMessagesItem>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub properties: Option<Option<JudgeConversationRequestProperties>>,
}
