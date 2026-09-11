#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostDatasetRecordInSchema {
    pub payload: ConversationPayload,
    pub properties: PostDatasetRecordInSchemaProperties,
}
