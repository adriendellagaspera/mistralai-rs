pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum StreamDeploymentLogsDeploymentsResponseData {
    DeploymentLogRecord(DeploymentLogRecord),

    StreamError(StreamError),
}

impl StreamDeploymentLogsDeploymentsResponseData {
    pub fn is_deployment_log_record(&self) -> bool {
        matches!(self, Self::DeploymentLogRecord(_))
    }

    pub fn is_stream_error(&self) -> bool {
        matches!(self, Self::StreamError(_))
    }

    pub fn as_deployment_log_record(&self) -> Option<&DeploymentLogRecord> {
        match self {
            Self::DeploymentLogRecord(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_deployment_log_record(self) -> Option<DeploymentLogRecord> {
        match self {
            Self::DeploymentLogRecord(value) => Some(value),
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

impl fmt::Display for StreamDeploymentLogsDeploymentsResponseData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DeploymentLogRecord(value) => write!(
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
