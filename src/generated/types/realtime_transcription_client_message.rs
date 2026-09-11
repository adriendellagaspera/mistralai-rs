#[derive(Debug, Clone)]
pub enum RealtimeTranscriptionClientMessage {
    RealtimeTranscriptionSessionUpdateMessage(RealtimeTranscriptionSessionUpdateMessage),
    RealtimeTranscriptionInputAudioAppend(RealtimeTranscriptionInputAudioAppend),
    RealtimeTranscriptionInputAudioFlush(RealtimeTranscriptionInputAudioFlush),
    RealtimeTranscriptionInputAudioEnd(RealtimeTranscriptionInputAudioEnd),
}
