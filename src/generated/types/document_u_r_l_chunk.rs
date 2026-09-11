#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocumentURLChunk {
    ///The filename of the document
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub document_name: Option<Option<String>>,
    pub document_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<DocumentURLChunkType>,
}
