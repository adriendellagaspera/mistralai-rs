#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpeechStreamDone {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SpeechStreamDoneType>,
    pub usage: UsageInfo,
}
