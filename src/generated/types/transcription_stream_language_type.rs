#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum TranscriptionStreamLanguageType {
    #[default]
    #[serde(rename = "transcription.language")]
    TranscriptionLanguage,
}
