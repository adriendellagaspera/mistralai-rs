#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ConversationSource {
    #[default]
    #[serde(rename = "EXPLORER")]
    Explorer,
    #[serde(rename = "UPLOADED_FILE")]
    UploadedFile,
    #[serde(rename = "DIRECT_INPUT")]
    DirectInput,
    #[serde(rename = "PLAYGROUND")]
    Playground,
}
