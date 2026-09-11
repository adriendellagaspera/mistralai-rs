#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum APIKeyAuthType {
    #[default]
    #[serde(rename = "api-key")]
    ApiKey,
}
