#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CredentialsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_preset_credentials_for_auth: Option<Vec<OutboundAuthenticationType>>,
    pub credentials: Vec<AuthenticationConfiguration>,
}
