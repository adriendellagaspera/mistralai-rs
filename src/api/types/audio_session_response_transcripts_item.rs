pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "role")]
#[non_exhaustive]
pub enum AudioSessionResponseTranscriptsItem {
    #[serde(rename = "assistant")]
    #[non_exhaustive]
    Assistant {
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration_ms: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_interruption: Option<bool>,
        #[serde(default)]
        latency: AudioLatencyBreakdownAssistant,
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset")]
        timestamp: DateTime<FixedOffset>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<AudioToolCall>>,
    },

    #[serde(rename = "tool")]
    #[non_exhaustive]
    Tool {
        #[serde(skip_serializing_if = "Option::is_none")]
        duration_ms: Option<f64>,
        #[serde(default)]
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        result: Option<serde_json::Value>,
        status: AudioToolTranscriptTurnStatus,
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset")]
        timestamp: DateTime<FixedOffset>,
        #[serde(default)]
        tool_call_id: String,
    },

    #[serde(rename = "user")]
    #[non_exhaustive]
    User {
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration_ms: Option<f64>,
        #[serde(default)]
        latency: AudioLatencyBreakdownUser,
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset")]
        timestamp: DateTime<FixedOffset>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl AudioSessionResponseTranscriptsItem {
    pub fn assistant(
        latency: AudioLatencyBreakdownAssistant,
        timestamp: DateTime<FixedOffset>,
    ) -> Self {
        Self::Assistant {
            content: None,
            duration_ms: None,
            is_interruption: None,
            latency,
            timestamp,
            tool_calls: None,
        }
    }

    pub fn tool(
        name: String,
        status: AudioToolTranscriptTurnStatus,
        timestamp: DateTime<FixedOffset>,
        tool_call_id: String,
    ) -> Self {
        Self::Tool {
            duration_ms: None,
            name,
            result: None,
            status,
            timestamp,
            tool_call_id,
        }
    }

    pub fn user(latency: AudioLatencyBreakdownUser, timestamp: DateTime<FixedOffset>) -> Self {
        Self::User {
            content: None,
            duration_ms: None,
            latency,
            timestamp,
        }
    }

    pub fn assistant_with_content(
        content: String,
        duration_ms: Option<f64>,
        is_interruption: Option<bool>,
        latency: AudioLatencyBreakdownAssistant,
        timestamp: DateTime<FixedOffset>,
        tool_calls: Option<Vec<AudioToolCall>>,
    ) -> Self {
        Self::Assistant {
            content: Some(content),
            duration_ms,
            is_interruption,
            latency,
            timestamp,
            tool_calls,
        }
    }

    pub fn assistant_with_duration_ms(
        content: Option<String>,
        duration_ms: f64,
        is_interruption: Option<bool>,
        latency: AudioLatencyBreakdownAssistant,
        timestamp: DateTime<FixedOffset>,
        tool_calls: Option<Vec<AudioToolCall>>,
    ) -> Self {
        Self::Assistant {
            content,
            duration_ms: Some(duration_ms),
            is_interruption,
            latency,
            timestamp,
            tool_calls,
        }
    }

    pub fn assistant_with_is_interruption(
        content: Option<String>,
        duration_ms: Option<f64>,
        is_interruption: bool,
        latency: AudioLatencyBreakdownAssistant,
        timestamp: DateTime<FixedOffset>,
        tool_calls: Option<Vec<AudioToolCall>>,
    ) -> Self {
        Self::Assistant {
            content,
            duration_ms,
            is_interruption: Some(is_interruption),
            latency,
            timestamp,
            tool_calls,
        }
    }

    pub fn assistant_with_tool_calls(
        content: Option<String>,
        duration_ms: Option<f64>,
        is_interruption: Option<bool>,
        latency: AudioLatencyBreakdownAssistant,
        timestamp: DateTime<FixedOffset>,
        tool_calls: Vec<AudioToolCall>,
    ) -> Self {
        Self::Assistant {
            content,
            duration_ms,
            is_interruption,
            latency,
            timestamp,
            tool_calls: Some(tool_calls),
        }
    }

    pub fn tool_with_duration_ms(
        duration_ms: f64,
        name: String,
        result: Option<serde_json::Value>,
        status: AudioToolTranscriptTurnStatus,
        timestamp: DateTime<FixedOffset>,
        tool_call_id: String,
    ) -> Self {
        Self::Tool {
            duration_ms: Some(duration_ms),
            name,
            result,
            status,
            timestamp,
            tool_call_id,
        }
    }

    pub fn tool_with_result(
        duration_ms: Option<f64>,
        name: String,
        result: serde_json::Value,
        status: AudioToolTranscriptTurnStatus,
        timestamp: DateTime<FixedOffset>,
        tool_call_id: String,
    ) -> Self {
        Self::Tool {
            duration_ms,
            name,
            result: Some(result),
            status,
            timestamp,
            tool_call_id,
        }
    }

    pub fn user_with_content(
        content: String,
        duration_ms: Option<f64>,
        latency: AudioLatencyBreakdownUser,
        timestamp: DateTime<FixedOffset>,
    ) -> Self {
        Self::User {
            content: Some(content),
            duration_ms,
            latency,
            timestamp,
        }
    }

    pub fn user_with_duration_ms(
        content: Option<String>,
        duration_ms: f64,
        latency: AudioLatencyBreakdownUser,
        timestamp: DateTime<FixedOffset>,
    ) -> Self {
        Self::User {
            content,
            duration_ms: Some(duration_ms),
            latency,
            timestamp,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
