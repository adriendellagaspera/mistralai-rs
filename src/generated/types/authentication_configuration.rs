#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthenticationConfiguration {
    pub authentication_type: OutboundAuthenticationType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    pub name: String,
}
