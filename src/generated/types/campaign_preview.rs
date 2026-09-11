#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CampaignPreview {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub description: String,
    pub id: uuid::Uuid,
    pub judge: JudgePreview,
    pub max_nb_events: i64,
    pub name: String,
    pub owner_id: uuid::Uuid,
    pub search_params: FilterPayload,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub workspace_id: uuid::Uuid,
}
