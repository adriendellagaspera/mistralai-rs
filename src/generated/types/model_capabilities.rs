#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ModelCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_transcription: Option<bool>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vision: Option<bool>,
}
