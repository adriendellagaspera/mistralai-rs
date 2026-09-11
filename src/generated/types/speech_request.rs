#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpeechRequest {
    ///Text to generate speech from.
    pub input: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub model: Option<Option<String>>,
    ///The base64-encoded audio reference for zero-shot voice cloning.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub ref_audio: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<SpeechOutputFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    ///The preset or custom voice to use for generating the speech.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub voice_id: Option<Option<String>>,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
