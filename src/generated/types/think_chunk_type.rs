#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ThinkChunkType {
    #[default]
    #[serde(rename = "thinking")]
    Thinking,
}
