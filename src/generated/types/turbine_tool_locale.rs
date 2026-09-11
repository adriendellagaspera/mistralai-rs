#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TurbineToolLocale {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub description: Option<Option<TurbineToolLocaleDescription>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub done_description: Option<Option<TurbineToolLocaleDoneDescription>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub name: Option<Option<TurbineToolLocaleName>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub usage_sentence: Option<Option<TurbineToolLocaleUsageSentence>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub working_description: Option<Option<TurbineToolLocaleWorkingDescription>>,
}
