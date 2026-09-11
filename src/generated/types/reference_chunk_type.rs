#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ReferenceChunkType {
    #[default]
    #[serde(rename = "reference")]
    Reference,
}
