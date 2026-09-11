#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WandbIntegrationOut {
    ///A display name to set for the run. If not set, will use the job ID as the name.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub name: Option<Option<String>>,
    ///The name of the project that the new run will be created under.
    pub project: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub run_name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<WandbIntegrationOutType>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub url: Option<Option<String>>,
}
