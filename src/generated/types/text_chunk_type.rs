#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum TextChunkType {
    #[default]
    #[serde(rename = "text")]
    Text,
}
