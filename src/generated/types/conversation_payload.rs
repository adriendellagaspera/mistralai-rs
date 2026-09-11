///
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConversationPayload {
    pub messages: Vec<ConversationPayloadMessagesItem>,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
