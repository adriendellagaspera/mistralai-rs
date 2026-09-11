#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum MessageInputEntryType {
    #[default]
    #[serde(rename = "message.input")]
    MessageInput,
}
