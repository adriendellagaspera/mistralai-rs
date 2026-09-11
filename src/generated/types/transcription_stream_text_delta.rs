#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TranscriptionStreamTextDelta {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TranscriptionStreamTextDeltaType>,
}
