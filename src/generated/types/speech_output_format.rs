#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum SpeechOutputFormat {
    #[default]
    #[serde(rename = "pcm")]
    Pcm,
    #[serde(rename = "wav")]
    Wav,
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "flac")]
    Flac,
    #[serde(rename = "opus")]
    Opus,
}
