#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SharingOut {
    pub library_id: uuid::Uuid,
    pub org_id: uuid::Uuid,
    pub role: String,
    pub share_with_type: String,
    pub share_with_uuid: Option<uuid::Uuid>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub user_id: Option<Option<uuid::Uuid>>,
}
