#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Connector {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub auth_type: Option<Option<String>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub id: uuid::Uuid,
    pub modified_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub server: Option<Option<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub tools: Option<Option<Vec<IntegrationsSchemasApiToolTool>>>,
}
