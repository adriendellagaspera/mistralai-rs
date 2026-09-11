#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dataset {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub description: String,
    pub id: uuid::Uuid,
    pub name: String,
    pub owner_id: uuid::Uuid,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub workspace_id: uuid::Uuid,
}
