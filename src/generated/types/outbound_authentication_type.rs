#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum OutboundAuthenticationType {
    #[default]
    #[serde(rename = "oauth2")]
    Oauth2,
    #[serde(rename = "bearer")]
    Bearer,
    #[serde(rename = "none")]
    None,
}
