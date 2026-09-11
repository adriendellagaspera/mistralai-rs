#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatasetRecord {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dataset_id: uuid::Uuid,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub id: uuid::Uuid,
    pub payload: ConversationPayload,
    pub properties: DatasetRecordProperties,
    pub source: ConversationSource,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
