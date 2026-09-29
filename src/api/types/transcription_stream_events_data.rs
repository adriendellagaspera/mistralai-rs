pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum TranscriptionStreamEventsData {
    #[serde(rename = "transcription.done")]
    #[non_exhaustive]
    TranscriptionDone {
        #[serde(skip_serializing_if = "Option::is_none")]
        language: Option<String>,
        #[serde(default)]
        model: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        segments: Option<Vec<TranscriptionSegmentChunk>>,
        #[serde(default)]
        text: String,
        #[serde(default)]
        usage: UsageInfo,
    },

    #[serde(rename = "transcription.language")]
    #[non_exhaustive]
    TranscriptionLanguage {
        #[serde(default)]
        audio_language: String,
    },

    #[serde(rename = "transcription.segment")]
    #[non_exhaustive]
    TranscriptionSegment {
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        end: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        speaker_id: Option<String>,
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        start: f64,
        #[serde(default)]
        text: String,
    },

    #[serde(rename = "transcription.text.delta")]
    #[non_exhaustive]
    TranscriptionTextDelta {
        #[serde(default)]
        text: String,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl TranscriptionStreamEventsData {
    pub fn transcription_done(model: String, text: String, usage: UsageInfo) -> Self {
        Self::TranscriptionDone {
            language: None,
            model,
            segments: None,
            text,
            usage,
        }
    }

    pub fn transcription_language(audio_language: String) -> Self {
        Self::TranscriptionLanguage { audio_language }
    }

    pub fn transcription_segment(end: f64, start: f64, text: String) -> Self {
        Self::TranscriptionSegment {
            end,
            speaker_id: None,
            start,
            text,
        }
    }

    pub fn transcription_text_delta(text: String) -> Self {
        Self::TranscriptionTextDelta { text }
    }

    pub fn transcription_done_with_language(
        language: String,
        model: String,
        segments: Option<Vec<TranscriptionSegmentChunk>>,
        text: String,
        usage: UsageInfo,
    ) -> Self {
        Self::TranscriptionDone {
            language: Some(language),
            model,
            segments,
            text,
            usage,
        }
    }

    pub fn transcription_done_with_segments(
        language: Option<String>,
        model: String,
        segments: Vec<TranscriptionSegmentChunk>,
        text: String,
        usage: UsageInfo,
    ) -> Self {
        Self::TranscriptionDone {
            language,
            model,
            segments: Some(segments),
            text,
            usage,
        }
    }

    pub fn transcription_segment_with_speaker_id(
        end: f64,
        speaker_id: String,
        start: f64,
        text: String,
    ) -> Self {
        Self::TranscriptionSegment {
            end,
            speaker_id: Some(speaker_id),
            start,
            text,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
