pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum StreamExecutionsResponseData {
    StreamEventSsePayload(StreamEventSsePayload),

    StreamEventSseErrorData(StreamEventSseErrorData),
}

impl StreamExecutionsResponseData {
    pub fn is_stream_event_sse_payload(&self) -> bool {
        matches!(self, Self::StreamEventSsePayload(_))
    }

    pub fn is_stream_event_sse_error_data(&self) -> bool {
        matches!(self, Self::StreamEventSseErrorData(_))
    }

    pub fn as_stream_event_sse_payload(&self) -> Option<&StreamEventSsePayload> {
        match self {
            Self::StreamEventSsePayload(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_stream_event_sse_payload(self) -> Option<StreamEventSsePayload> {
        match self {
            Self::StreamEventSsePayload(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_stream_event_sse_error_data(&self) -> Option<&StreamEventSseErrorData> {
        match self {
            Self::StreamEventSseErrorData(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_stream_event_sse_error_data(self) -> Option<StreamEventSseErrorData> {
        match self {
            Self::StreamEventSseErrorData(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for StreamExecutionsResponseData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StreamEventSsePayload(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::StreamEventSseErrorData(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
