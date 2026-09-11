#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum DocumentURLChunkType {
    #[default]
    #[serde(rename = "document_url")]
    DocumentUrl,
}
