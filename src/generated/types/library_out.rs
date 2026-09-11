#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LibraryOut {
    pub chunk_size: Option<i64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub description: Option<Option<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub emoji: Option<Option<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub explicit_user_members_count: Option<Option<i64>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub explicit_workspace_members_count: Option<Option<i64>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub generated_description: Option<Option<String>>,
    ///Generated Name
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub generated_name: Option<Option<String>>,
    pub id: uuid::Uuid,
    pub name: String,
    pub nb_documents: i64,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub org_sharing_role: Option<Option<String>>,
    pub owner_id: Option<uuid::Uuid>,
    pub owner_type: String,
    pub total_size: i64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
