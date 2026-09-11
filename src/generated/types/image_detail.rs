#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ImageDetail {
    #[default]
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "high")]
    High,
}
