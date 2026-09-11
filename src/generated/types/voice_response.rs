///Schema for voice response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VoiceResponse {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub age: Option<Option<i64>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub color: Option<Option<String>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub gender: Option<Option<String>>,
    pub id: uuid::Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_notice: Option<i64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub slug: Option<Option<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub tags: Option<Option<Vec<String>>>,
    pub user_id: Option<String>,
}
