#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ChatCompletionFieldOptions {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub options: Option<Option<Vec<Option<ChatCompletionFieldOptionsOptionsItemUnion>>>>,
}
