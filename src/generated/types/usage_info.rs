#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UsageInfo {
    #[serde(default)]
    pub completion_tokens: i64,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub num_cached_tokens: Option<Option<i64>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub prompt_audio_seconds: Option<Option<i64>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub prompt_token_details: Option<Option<PromptTokensDetails>>,
    #[serde(default)]
    pub prompt_tokens: i64,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub prompt_tokens_details: Option<Option<PromptTokensDetails>>,
    #[serde(default)]
    pub total_tokens: i64,
}
