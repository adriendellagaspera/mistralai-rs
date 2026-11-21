pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpeechStreamDoneType {
    #[serde(rename = "speech.audio.done")]
    SpeechAudioDone,
}
impl fmt::Display for SpeechStreamDoneType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SpeechAudioDone => "speech.audio.done",
        };
        write!(f, "{}", s)
    }
}
