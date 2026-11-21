pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum VoiceAgentEvent {
    #[serde(rename = "agent.state.changed")]
    #[non_exhaustive]
    AgentStateChanged {
        old_state: VoiceAgentStateChangedEventOldState,
        state: VoiceAgentStateChangedEventState,
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        ts: f64,
    },

    #[serde(rename = "error")]
    #[non_exhaustive]
    Error {
        #[serde(default)]
        code: String,
        #[serde(default)]
        fatal: bool,
        #[serde(default)]
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        payload: Option<HashMap<String, serde_json::Value>>,
        source: VoiceAgentErrorEventSource,
    },

    #[serde(rename = "tool_call.ended")]
    #[non_exhaustive]
    ToolCallEnded {
        #[serde(default)]
        arguments: String,
        #[serde(default)]
        id: String,
        #[serde(default)]
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        phase: Option<VoiceAgentToolCallEndedEventPhase>,
        #[serde(skip_serializing_if = "Option::is_none")]
        result: Option<String>,
        status: VoiceAgentToolCallEndedEventStatus,
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        ts: f64,
    },

    #[serde(rename = "tool_call.started")]
    #[non_exhaustive]
    ToolCallStarted {
        #[serde(default)]
        id: String,
        #[serde(default)]
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        phase: Option<VoiceAgentToolCallStartedEventPhase>,
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        ts: f64,
    },

    #[serde(rename = "transcription.segment")]
    #[non_exhaustive]
    TranscriptionSegment {
        #[serde(default)]
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        end_time: Option<f64>,
        #[serde(default)]
        r#final: bool,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        language: Option<String>,
        role: VoiceAgentTranscriptEventRole,
        #[serde(skip_serializing_if = "Option::is_none")]
        start_time: Option<f64>,
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        ts: f64,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl VoiceAgentEvent {
    pub fn agent_state_changed(
        old_state: VoiceAgentStateChangedEventOldState,
        state: VoiceAgentStateChangedEventState,
        ts: f64,
    ) -> Self {
        Self::AgentStateChanged {
            old_state,
            state,
            ts,
        }
    }

    pub fn error(
        code: String,
        fatal: bool,
        message: String,
        source: VoiceAgentErrorEventSource,
    ) -> Self {
        Self::Error {
            code,
            fatal,
            message,
            payload: None,
            source,
        }
    }

    pub fn tool_call_ended(
        arguments: String,
        id: String,
        name: String,
        status: VoiceAgentToolCallEndedEventStatus,
        ts: f64,
    ) -> Self {
        Self::ToolCallEnded {
            arguments,
            id,
            name,
            phase: None,
            result: None,
            status,
            ts,
        }
    }

    pub fn tool_call_started(id: String, name: String, ts: f64) -> Self {
        Self::ToolCallStarted {
            id,
            name,
            phase: None,
            ts,
        }
    }

    pub fn transcription_segment(
        content: String,
        r#final: bool,
        id: String,
        role: VoiceAgentTranscriptEventRole,
        ts: f64,
    ) -> Self {
        Self::TranscriptionSegment {
            content,
            end_time: None,
            r#final,
            id,
            language: None,
            role,
            start_time: None,
            ts,
        }
    }

    pub fn error_with_payload(
        code: String,
        fatal: bool,
        message: String,
        payload: HashMap<String, serde_json::Value>,
        source: VoiceAgentErrorEventSource,
    ) -> Self {
        Self::Error {
            code,
            fatal,
            message,
            payload: Some(payload),
            source,
        }
    }

    pub fn tool_call_ended_with_phase(
        arguments: String,
        id: String,
        name: String,
        phase: VoiceAgentToolCallEndedEventPhase,
        result: Option<String>,
        status: VoiceAgentToolCallEndedEventStatus,
        ts: f64,
    ) -> Self {
        Self::ToolCallEnded {
            arguments,
            id,
            name,
            phase: Some(phase),
            result,
            status,
            ts,
        }
    }

    pub fn tool_call_ended_with_result(
        arguments: String,
        id: String,
        name: String,
        phase: Option<VoiceAgentToolCallEndedEventPhase>,
        result: String,
        status: VoiceAgentToolCallEndedEventStatus,
        ts: f64,
    ) -> Self {
        Self::ToolCallEnded {
            arguments,
            id,
            name,
            phase,
            result: Some(result),
            status,
            ts,
        }
    }

    pub fn tool_call_started_with_phase(
        id: String,
        name: String,
        phase: VoiceAgentToolCallStartedEventPhase,
        ts: f64,
    ) -> Self {
        Self::ToolCallStarted {
            id,
            name,
            phase: Some(phase),
            ts,
        }
    }

    pub fn transcription_segment_with_end_time(
        content: String,
        end_time: f64,
        r#final: bool,
        id: String,
        language: Option<String>,
        role: VoiceAgentTranscriptEventRole,
        start_time: Option<f64>,
        ts: f64,
    ) -> Self {
        Self::TranscriptionSegment {
            content,
            end_time: Some(end_time),
            r#final,
            id,
            language,
            role,
            start_time,
            ts,
        }
    }

    pub fn transcription_segment_with_language(
        content: String,
        end_time: Option<f64>,
        r#final: bool,
        id: String,
        language: String,
        role: VoiceAgentTranscriptEventRole,
        start_time: Option<f64>,
        ts: f64,
    ) -> Self {
        Self::TranscriptionSegment {
            content,
            end_time,
            r#final,
            id,
            language: Some(language),
            role,
            start_time,
            ts,
        }
    }

    pub fn transcription_segment_with_start_time(
        content: String,
        end_time: Option<f64>,
        r#final: bool,
        id: String,
        language: Option<String>,
        role: VoiceAgentTranscriptEventRole,
        start_time: f64,
        ts: f64,
    ) -> Self {
        Self::TranscriptionSegment {
            content,
            end_time,
            r#final,
            id,
            language,
            role,
            start_time: Some(start_time),
            ts,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
