#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum MessageInputEntryRole {
    #[default]
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
}
