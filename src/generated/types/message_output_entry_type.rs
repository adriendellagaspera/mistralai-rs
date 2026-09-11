#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum MessageOutputEntryType {
    #[default]
    #[serde(rename = "message.output")]
    MessageOutput,
}
