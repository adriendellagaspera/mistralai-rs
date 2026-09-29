pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkflowEventType {
    WorkflowExecutionStarted,
    WorkflowExecutionCompleted,
    WorkflowExecutionFailed,
    WorkflowExecutionCanceled,
    WorkflowExecutionContinuedAsNew,
    WorkflowTaskTimedOut,
    WorkflowTaskFailed,
    CustomTaskStarted,
    CustomTaskInProgress,
    CustomTaskCompleted,
    CustomTaskFailed,
    CustomTaskTimedOut,
    CustomTaskCanceled,
    ActivityTaskStarted,
    ActivityTaskCompleted,
    ActivityTaskRetrying,
    ActivityTaskFailed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WorkflowEventType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::WorkflowExecutionStarted => {
                serializer.serialize_str("WORKFLOW_EXECUTION_STARTED")
            }
            Self::WorkflowExecutionCompleted => {
                serializer.serialize_str("WORKFLOW_EXECUTION_COMPLETED")
            }
            Self::WorkflowExecutionFailed => serializer.serialize_str("WORKFLOW_EXECUTION_FAILED"),
            Self::WorkflowExecutionCanceled => {
                serializer.serialize_str("WORKFLOW_EXECUTION_CANCELED")
            }
            Self::WorkflowExecutionContinuedAsNew => {
                serializer.serialize_str("WORKFLOW_EXECUTION_CONTINUED_AS_NEW")
            }
            Self::WorkflowTaskTimedOut => serializer.serialize_str("WORKFLOW_TASK_TIMED_OUT"),
            Self::WorkflowTaskFailed => serializer.serialize_str("WORKFLOW_TASK_FAILED"),
            Self::CustomTaskStarted => serializer.serialize_str("CUSTOM_TASK_STARTED"),
            Self::CustomTaskInProgress => serializer.serialize_str("CUSTOM_TASK_IN_PROGRESS"),
            Self::CustomTaskCompleted => serializer.serialize_str("CUSTOM_TASK_COMPLETED"),
            Self::CustomTaskFailed => serializer.serialize_str("CUSTOM_TASK_FAILED"),
            Self::CustomTaskTimedOut => serializer.serialize_str("CUSTOM_TASK_TIMED_OUT"),
            Self::CustomTaskCanceled => serializer.serialize_str("CUSTOM_TASK_CANCELED"),
            Self::ActivityTaskStarted => serializer.serialize_str("ACTIVITY_TASK_STARTED"),
            Self::ActivityTaskCompleted => serializer.serialize_str("ACTIVITY_TASK_COMPLETED"),
            Self::ActivityTaskRetrying => serializer.serialize_str("ACTIVITY_TASK_RETRYING"),
            Self::ActivityTaskFailed => serializer.serialize_str("ACTIVITY_TASK_FAILED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WorkflowEventType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "WORKFLOW_EXECUTION_STARTED" => Ok(Self::WorkflowExecutionStarted),
            "WORKFLOW_EXECUTION_COMPLETED" => Ok(Self::WorkflowExecutionCompleted),
            "WORKFLOW_EXECUTION_FAILED" => Ok(Self::WorkflowExecutionFailed),
            "WORKFLOW_EXECUTION_CANCELED" => Ok(Self::WorkflowExecutionCanceled),
            "WORKFLOW_EXECUTION_CONTINUED_AS_NEW" => Ok(Self::WorkflowExecutionContinuedAsNew),
            "WORKFLOW_TASK_TIMED_OUT" => Ok(Self::WorkflowTaskTimedOut),
            "WORKFLOW_TASK_FAILED" => Ok(Self::WorkflowTaskFailed),
            "CUSTOM_TASK_STARTED" => Ok(Self::CustomTaskStarted),
            "CUSTOM_TASK_IN_PROGRESS" => Ok(Self::CustomTaskInProgress),
            "CUSTOM_TASK_COMPLETED" => Ok(Self::CustomTaskCompleted),
            "CUSTOM_TASK_FAILED" => Ok(Self::CustomTaskFailed),
            "CUSTOM_TASK_TIMED_OUT" => Ok(Self::CustomTaskTimedOut),
            "CUSTOM_TASK_CANCELED" => Ok(Self::CustomTaskCanceled),
            "ACTIVITY_TASK_STARTED" => Ok(Self::ActivityTaskStarted),
            "ACTIVITY_TASK_COMPLETED" => Ok(Self::ActivityTaskCompleted),
            "ACTIVITY_TASK_RETRYING" => Ok(Self::ActivityTaskRetrying),
            "ACTIVITY_TASK_FAILED" => Ok(Self::ActivityTaskFailed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WorkflowEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WorkflowExecutionStarted => write!(f, "WORKFLOW_EXECUTION_STARTED"),
            Self::WorkflowExecutionCompleted => write!(f, "WORKFLOW_EXECUTION_COMPLETED"),
            Self::WorkflowExecutionFailed => write!(f, "WORKFLOW_EXECUTION_FAILED"),
            Self::WorkflowExecutionCanceled => write!(f, "WORKFLOW_EXECUTION_CANCELED"),
            Self::WorkflowExecutionContinuedAsNew => {
                write!(f, "WORKFLOW_EXECUTION_CONTINUED_AS_NEW")
            }
            Self::WorkflowTaskTimedOut => write!(f, "WORKFLOW_TASK_TIMED_OUT"),
            Self::WorkflowTaskFailed => write!(f, "WORKFLOW_TASK_FAILED"),
            Self::CustomTaskStarted => write!(f, "CUSTOM_TASK_STARTED"),
            Self::CustomTaskInProgress => write!(f, "CUSTOM_TASK_IN_PROGRESS"),
            Self::CustomTaskCompleted => write!(f, "CUSTOM_TASK_COMPLETED"),
            Self::CustomTaskFailed => write!(f, "CUSTOM_TASK_FAILED"),
            Self::CustomTaskTimedOut => write!(f, "CUSTOM_TASK_TIMED_OUT"),
            Self::CustomTaskCanceled => write!(f, "CUSTOM_TASK_CANCELED"),
            Self::ActivityTaskStarted => write!(f, "ACTIVITY_TASK_STARTED"),
            Self::ActivityTaskCompleted => write!(f, "ACTIVITY_TASK_COMPLETED"),
            Self::ActivityTaskRetrying => write!(f, "ACTIVITY_TASK_RETRYING"),
            Self::ActivityTaskFailed => write!(f, "ACTIVITY_TASK_FAILED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
