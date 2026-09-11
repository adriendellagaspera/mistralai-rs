#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum AudioEncoding {
    #[default]
    #[serde(rename = "pcm_s16le")]
    PcmS16le,
    #[serde(rename = "pcm_s32le")]
    PcmS32le,
    #[serde(rename = "pcm_f16le")]
    PcmF16le,
    #[serde(rename = "pcm_f32le")]
    PcmF32le,
    #[serde(rename = "pcm_mulaw")]
    PcmMulaw,
    #[serde(rename = "pcm_alaw")]
    PcmAlaw,
}
