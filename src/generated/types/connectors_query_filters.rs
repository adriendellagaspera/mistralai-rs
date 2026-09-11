#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ConnectorsQueryFilters {
    ///Filter for active connectors for a given user, workspace and organization.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub active: Option<Option<bool>>,
    ///Fetch connection secrets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_connection_secrets: Option<bool>,
}
