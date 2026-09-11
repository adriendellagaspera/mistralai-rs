#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JudgePreview {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub base_revision: Option<Option<uuid::Uuid>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub description: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub down_revision: Option<Option<uuid::Uuid>>,
    pub id: uuid::Uuid,
    pub instructions: String,
    pub model_name: String,
    pub name: String,
    pub output: JudgePreviewOutput,
    pub owner_id: uuid::Uuid,
    pub tools: Vec<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub up_revision: Option<Option<uuid::Uuid>>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub workspace_id: uuid::Uuid,
}
