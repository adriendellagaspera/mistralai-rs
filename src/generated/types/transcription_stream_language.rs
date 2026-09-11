#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TranscriptionStreamLanguage {
    ///Constraint: pattern=`^\w{2}$`
    pub audio_language: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TranscriptionStreamLanguageType>,
}
