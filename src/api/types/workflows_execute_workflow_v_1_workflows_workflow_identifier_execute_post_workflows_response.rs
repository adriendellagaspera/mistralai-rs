pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ExecuteWorkflowV1WorkflowsWorkflowIdentifierExecutePostWorkflowsResponse {
    WorkflowExecutionResponse(WorkflowExecutionResponse),

    WorkflowExecutionSyncResponse(WorkflowExecutionSyncResponse),
}

impl ExecuteWorkflowV1WorkflowsWorkflowIdentifierExecutePostWorkflowsResponse {
    pub fn is_workflow_execution_response(&self) -> bool {
        matches!(self, Self::WorkflowExecutionResponse(_))
    }

    pub fn is_workflow_execution_sync_response(&self) -> bool {
        matches!(self, Self::WorkflowExecutionSyncResponse(_))
    }

    pub fn as_workflow_execution_response(&self) -> Option<&WorkflowExecutionResponse> {
        match self {
            Self::WorkflowExecutionResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_workflow_execution_response(self) -> Option<WorkflowExecutionResponse> {
        match self {
            Self::WorkflowExecutionResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_workflow_execution_sync_response(&self) -> Option<&WorkflowExecutionSyncResponse> {
        match self {
            Self::WorkflowExecutionSyncResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_workflow_execution_sync_response(self) -> Option<WorkflowExecutionSyncResponse> {
        match self {
            Self::WorkflowExecutionSyncResponse(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ExecuteWorkflowV1WorkflowsWorkflowIdentifierExecutePostWorkflowsResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WorkflowExecutionResponse(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WorkflowExecutionSyncResponse(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
