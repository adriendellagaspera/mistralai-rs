pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum StreamLogsExecutionsResponseData {
    ExecutionLogRecord(ExecutionLogRecord),

    StreamError(StreamError),
}

impl StreamLogsExecutionsResponseData {
    pub fn is_execution_log_record(&self) -> bool {
        matches!(self, Self::ExecutionLogRecord(_))
    }

    pub fn is_stream_error(&self) -> bool {
        matches!(self, Self::StreamError(_))
    }

    pub fn as_execution_log_record(&self) -> Option<&ExecutionLogRecord> {
        match self {
            Self::ExecutionLogRecord(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_execution_log_record(self) -> Option<ExecutionLogRecord> {
        match self {
            Self::ExecutionLogRecord(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_stream_error(&self) -> Option<&StreamError> {
        match self {
            Self::StreamError(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_stream_error(self) -> Option<StreamError> {
        match self {
            Self::StreamError(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for StreamLogsExecutionsResponseData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExecutionLogRecord(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::StreamError(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
