#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ToolType {
    #[default]
    #[serde(rename = "rag")]
    Rag,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "event")]
    Event,
}
