#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum FileChunkType {
    #[default]
    #[serde(rename = "file")]
    File,
}
