#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ModerationLLMV2Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ModerationLLMAction>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub custom_category_thresholds: Option<Option<ModerationLLMV2CategoryThresholds>>,
    ///If true, only evaluate categories in custom_category_thresholds; others are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_other_categories: Option<bool>,
    ///Override model name. Should be omitted in general.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
}
