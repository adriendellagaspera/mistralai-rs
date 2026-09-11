#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocumentOut {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub attributes: Option<Option<DocumentOutAttributes>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub extension: Option<String>,
    pub hash: Option<String>,
    pub id: uuid::Uuid,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub last_processed_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub library_id: uuid::Uuid,
    pub mime_type: Option<String>,
    pub name: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub number_of_pages: Option<Option<i64>>,
    pub process_status: ProcessStatus,
    pub processing_status: String,
    pub size: Option<i64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub summary: Option<Option<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub tokens_processing_main_content: Option<Option<i64>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub tokens_processing_summary: Option<Option<i64>>,
    pub tokens_processing_total: i64,
    pub uploaded_by_id: Option<uuid::Uuid>,
    pub uploaded_by_type: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub url: Option<Option<String>>,
}
