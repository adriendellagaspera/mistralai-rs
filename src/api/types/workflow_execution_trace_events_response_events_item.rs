pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum WorkflowExecutionTraceEventsResponseEventsItem {
    WorkflowExecutionTraceEvent(WorkflowExecutionTraceEvent),

    WorkflowExecutionProgressTraceEvent(WorkflowExecutionProgressTraceEvent),
}

impl WorkflowExecutionTraceEventsResponseEventsItem {
    pub fn is_workflow_execution_trace_event(&self) -> bool {
        matches!(self, Self::WorkflowExecutionTraceEvent(_))
    }

    pub fn is_workflow_execution_progress_trace_event(&self) -> bool {
        matches!(self, Self::WorkflowExecutionProgressTraceEvent(_))
    }

    pub fn as_workflow_execution_trace_event(&self) -> Option<&WorkflowExecutionTraceEvent> {
        match self {
            Self::WorkflowExecutionTraceEvent(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_workflow_execution_trace_event(self) -> Option<WorkflowExecutionTraceEvent> {
        match self {
            Self::WorkflowExecutionTraceEvent(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_workflow_execution_progress_trace_event(
        &self,
    ) -> Option<&WorkflowExecutionProgressTraceEvent> {
        match self {
            Self::WorkflowExecutionProgressTraceEvent(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_workflow_execution_progress_trace_event(
        self,
    ) -> Option<WorkflowExecutionProgressTraceEvent> {
        match self {
            Self::WorkflowExecutionProgressTraceEvent(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for WorkflowExecutionTraceEventsResponseEventsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WorkflowExecutionTraceEvent(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WorkflowExecutionProgressTraceEvent(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
