#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct GuardrailConfig {
    ///If true, return HTTP 403 and block request in the event of a server-side error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_on_error: Option<bool>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub moderation_llm_v1: Option<Option<ModerationLLMV1Config>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub moderation_llm_v2: Option<Option<ModerationLLMV2Config>>,
}
