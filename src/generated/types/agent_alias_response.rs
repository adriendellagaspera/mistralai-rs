#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentAliasResponse {
    pub alias: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,
}
