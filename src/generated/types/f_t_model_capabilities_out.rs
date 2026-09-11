#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct FTModelCapabilitiesOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_chat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_fim: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fine_tuning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_calling: Option<bool>,
}
