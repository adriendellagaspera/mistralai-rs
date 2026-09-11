#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PutDatasetRecordPayloadInSchema {
    pub payload: ConversationPayload,
}
