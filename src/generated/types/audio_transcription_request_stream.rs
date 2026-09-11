#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AudioTranscriptionRequestStream {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_bias: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diarize: Option<bool>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "binary_bytes_serde::double_option",
        default
    )]
    pub file: Option<Option<File>>,
    ///ID of a file uploaded to /v1/files
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub file_id: Option<Option<String>>,
    ///Url of a file to be transcribed
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub file_url: Option<Option<url::Url>>,
    ///Language of the audio, e.g. 'en'. Providing the language can boost accuracy.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub language: Option<Option<String>>,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub temperature: Option<Option<f64>>,
    ///Granularities of timestamps to include in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_granularities: Option<Vec<TimestampGranularity>>,
}
