#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum OAuth2TokenTokenType {
    #[default]
    #[serde(rename = "Bearer")]
    Bearer,
}
