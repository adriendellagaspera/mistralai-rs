#[derive(Debug, Clone)]
pub enum SpeechStreamEventsData {
    SpeechStreamAudioDelta(SpeechStreamAudioDelta),
    SpeechStreamDone(SpeechStreamDone),
}
