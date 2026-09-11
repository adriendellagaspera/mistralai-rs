#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AudioChunk {
    pub input_audio: AudioChunkInputAudio,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AudioChunkType>,
}
