///The response after appending new entries to the conversation.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConversationResponse {
    pub conversation_id: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub guardrails: Option<Option<Vec<ConversationResponseGuardrailsItem>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ConversationResponseObject>,
    pub outputs: Vec<ConversationResponseOutputsItemUnion>,
    pub usage: ConversationUsageInfo,
}
