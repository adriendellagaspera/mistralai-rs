#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConnectorAuthenticationHeader {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_secret: Option<bool>,
    pub name: String,
}
