#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomConnector {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub authorization: Option<Option<CustomConnectorAuthorizationInline>>,
    pub connector_id: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub tool_configuration: Option<Option<ToolConfiguration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CustomConnectorType>,
}
