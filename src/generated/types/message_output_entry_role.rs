#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum MessageOutputEntryRole {
    #[default]
    #[serde(rename = "assistant")]
    Assistant,
}
