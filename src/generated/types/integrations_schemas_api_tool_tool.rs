#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntegrationsSchemasApiToolTool {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub active: Option<Option<bool>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub execution_config: Option<ExecutionConfig>,
    pub id: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub jsonschema: Option<Option<IntegrationsSchemasApiToolToolJsonschema>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub locale: Option<Option<IntegrationsSchemasTurbineToolLocale>>,
    pub modified_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub system_prompt: Option<Option<String>>,
    pub visibility: ResourceVisibility,
}
