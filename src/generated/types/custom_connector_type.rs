#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum CustomConnectorType {
    #[default]
    #[serde(rename = "connector")]
    Connector,
}
