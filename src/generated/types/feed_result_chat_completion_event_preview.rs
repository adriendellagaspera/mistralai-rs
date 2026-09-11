#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct FeedResultChatCompletionEventPreview {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub cursor: Option<Option<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub next: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ChatCompletionEventPreview>>,
}
