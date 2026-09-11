#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConnectorMCPCreate {
    ///Optional additional authentication data for the connector.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub auth_data: Option<Option<AuthData>>,
    ///The description of the connector.
    pub description: String,
    ///Optional organization-level headers to be sent with the request to the mcp server.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub headers: Option<Option<ConnectorMCPCreateHeaders>>,
    ///The optional url of the icon you want to associate to the connector.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub icon_url: Option<Option<String>>,
    ///The name of the connector. Should be 64 char length maximum, alphanumeric, only underscores/dashes.
    pub name: String,
    ///The url of the MCP server.
    ///Constraint: minLength=1, maxLength=2083
    pub server: url::Url,
    ///Optional system prompt for the connector.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub system_prompt: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<ResourceVisibility>,
}
