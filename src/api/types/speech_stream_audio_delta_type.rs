pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpeechStreamAudioDeltaType {
    #[serde(rename = "speech.audio.delta")]
    SpeechAudioDelta,
}
impl fmt::Display for SpeechStreamAudioDeltaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SpeechAudioDelta => "speech.audio.delta",
        };
        write!(f, "{}", s)
    }
}
