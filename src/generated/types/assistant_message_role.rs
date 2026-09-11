#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum AssistantMessageRole {
    #[default]
    #[serde(rename = "assistant")]
    Assistant,
}
