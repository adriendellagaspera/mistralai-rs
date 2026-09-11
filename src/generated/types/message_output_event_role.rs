#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum MessageOutputEventRole {
    #[default]
    #[serde(rename = "assistant")]
    Assistant,
}
