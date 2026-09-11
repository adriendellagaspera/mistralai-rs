#[derive(Debug, Clone)]
pub enum TranscriptionStreamEventsData {
    TranscriptionStreamTextDelta(TranscriptionStreamTextDelta),
    TranscriptionStreamLanguage(TranscriptionStreamLanguage),
    TranscriptionStreamSegmentDelta(TranscriptionStreamSegmentDelta),
    TranscriptionStreamDone(TranscriptionStreamDone),
}
