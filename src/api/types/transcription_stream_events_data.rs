pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum TranscriptionStreamEventsData {
    #[serde(rename = "transcription.done")]
    #[non_exhaustive]
    TranscriptionDone {
        #[serde(default)]
        model: String,
        #[serde(default)]
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        language: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        segments: Option<Vec<TranscriptionSegmentChunk>>,
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
        text: String,
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        start: f64,
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        end: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        speaker_id: Option<String>,
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
            model,
            text,
            language: None,
            segments: None,
            usage,
        }
    }

    pub fn transcription_language(audio_language: String) -> Self {
        Self::TranscriptionLanguage { audio_language }
    }

    pub fn transcription_segment(text: String, start: f64, end: f64) -> Self {
        Self::TranscriptionSegment {
            text,
            start,
            end,
            speaker_id: None,
        }
    }

    pub fn transcription_text_delta(text: String) -> Self {
        Self::TranscriptionTextDelta { text }
    }

    pub fn transcription_done_with_language(
        model: String,
        text: String,
        language: String,
        segments: Option<Vec<TranscriptionSegmentChunk>>,
        usage: UsageInfo,
    ) -> Self {
        Self::TranscriptionDone {
            model,
            text,
            language: Some(language),
            segments,
            usage,
        }
    }

    pub fn transcription_done_with_segments(
        model: String,
        text: String,
        language: Option<String>,
        segments: Vec<TranscriptionSegmentChunk>,
        usage: UsageInfo,
    ) -> Self {
        Self::TranscriptionDone {
            model,
            text,
            language,
            segments: Some(segments),
            usage,
        }
    }

    pub fn transcription_segment_with_speaker_id(
        text: String,
        start: f64,
        end: f64,
        speaker_id: String,
    ) -> Self {
        Self::TranscriptionSegment {
            text,
            start,
            end,
            speaker_id: Some(speaker_id),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
