#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ConnectorMCPUpdate {
    ///New authentication data for your mcp connector.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub auth_data: Option<Option<AuthData>>,
    ///Optional new connection config.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub connection_config: Option<Option<ConnectorMCPUpdateConnectionConfig>>,
    ///Optional new connection secrets
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub connection_secrets: Option<Option<ConnectorMCPUpdateConnectionSecrets>>,
    ///The description of the connector.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub description: Option<Option<String>>,
    ///New headers for your mcp connector.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub headers: Option<Option<ConnectorMCPUpdateHeaders>>,
    ///The optional url of the icon you want to associate to the connector.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub icon_url: Option<Option<String>>,
    ///The name of the connector.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub name: Option<Option<String>>,
    ///New server url for your mcp connector.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub server: Option<Option<url::Url>>,
    ///Optional system prompt for the connector.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub system_prompt: Option<Option<String>>,
}
