pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ListRunsRequestStatus {
    WorkflowExecutionStatus(WorkflowExecutionStatus),

    WorkflowExecutionStatusList(Vec<WorkflowExecutionStatus>),
}

impl ListRunsRequestStatus {
    pub fn is_workflow_execution_status(&self) -> bool {
        matches!(self, Self::WorkflowExecutionStatus(_))
    }

    pub fn is_workflow_execution_status_list(&self) -> bool {
        matches!(self, Self::WorkflowExecutionStatusList(_))
    }

    pub fn as_workflow_execution_status(&self) -> Option<&WorkflowExecutionStatus> {
        match self {
            Self::WorkflowExecutionStatus(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_workflow_execution_status(self) -> Option<WorkflowExecutionStatus> {
        match self {
            Self::WorkflowExecutionStatus(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_workflow_execution_status_list(&self) -> Option<&Vec<WorkflowExecutionStatus>> {
        match self {
            Self::WorkflowExecutionStatusList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_workflow_execution_status_list(self) -> Option<Vec<WorkflowExecutionStatus>> {
        match self {
            Self::WorkflowExecutionStatusList(value) => Some(value),
            _ => None,
        }
    }
}
