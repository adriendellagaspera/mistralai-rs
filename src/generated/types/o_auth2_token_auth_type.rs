#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum OAuth2TokenAuthType {
    #[default]
    #[serde(rename = "oauth2-token")]
    Oauth2Token,
}
