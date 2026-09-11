#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum AudioChunkType {
    #[default]
    #[serde(rename = "input_audio")]
    InputAudio,
}
