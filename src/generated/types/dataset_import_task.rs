#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatasetImportTask {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub creator_id: uuid::Uuid,
    pub dataset_id: uuid::Uuid,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub id: uuid::Uuid,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub message: Option<Option<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub progress: Option<Option<i64>>,
    pub status: BaseTaskStatus,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub workspace_id: uuid::Uuid,
}
