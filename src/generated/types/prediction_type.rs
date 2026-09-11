#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum PredictionType {
    #[default]
    #[serde(rename = "content")]
    Content,
}
