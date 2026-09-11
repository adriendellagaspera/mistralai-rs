#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SharingIn {
    pub level: ShareEnum,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub org_id: Option<Option<uuid::Uuid>>,
    pub share_with_type: EntityType,
    ///The id of the entity (user, workspace or organization) to share with
    pub share_with_uuid: uuid::Uuid,
}
