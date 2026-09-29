pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "event_type")]
#[non_exhaustive]
pub enum ListWorkflowEventResponseEventsItem {
    #[serde(rename = "WORKFLOW_EXECUTION_STARTED")]
    #[non_exhaustive]
    WorkflowExecutionStarted {
        #[serde(flatten)]
        data: WorkflowExecutionStartedResponse,
    },

    #[serde(rename = "WORKFLOW_EXECUTION_COMPLETED")]
    #[non_exhaustive]
    WorkflowExecutionCompleted {
        #[serde(flatten)]
        data: WorkflowExecutionCompletedResponse,
    },

    #[serde(rename = "WORKFLOW_EXECUTION_FAILED")]
    #[non_exhaustive]
    WorkflowExecutionFailed {
        #[serde(flatten)]
        data: WorkflowExecutionFailedResponse,
    },

    #[serde(rename = "WORKFLOW_EXECUTION_CANCELED")]
    #[non_exhaustive]
    WorkflowExecutionCanceled {
        #[serde(flatten)]
        data: WorkflowExecutionCanceledResponse,
    },

    #[serde(rename = "WORKFLOW_EXECUTION_CONTINUED_AS_NEW")]
    #[non_exhaustive]
    WorkflowExecutionContinuedAsNew {
        #[serde(flatten)]
        data: WorkflowExecutionContinuedAsNewResponse,
    },

    #[serde(rename = "WORKFLOW_TASK_TIMED_OUT")]
    #[non_exhaustive]
    WorkflowTaskTimedOut {
        #[serde(flatten)]
        data: WorkflowTaskTimedOutResponse,
    },

    #[serde(rename = "WORKFLOW_TASK_FAILED")]
    #[non_exhaustive]
    WorkflowTaskFailed {
        #[serde(flatten)]
        data: WorkflowTaskFailedResponse,
    },

    #[serde(rename = "CUSTOM_TASK_STARTED")]
    #[non_exhaustive]
    CustomTaskStarted {
        #[serde(flatten)]
        data: CustomTaskStartedResponse,
    },

    #[serde(rename = "CUSTOM_TASK_IN_PROGRESS")]
    #[non_exhaustive]
    CustomTaskInProgress {
        #[serde(flatten)]
        data: CustomTaskInProgressResponse,
    },

    #[serde(rename = "CUSTOM_TASK_COMPLETED")]
    #[non_exhaustive]
    CustomTaskCompleted {
        #[serde(flatten)]
        data: CustomTaskCompletedResponse,
    },

    #[serde(rename = "CUSTOM_TASK_FAILED")]
    #[non_exhaustive]
    CustomTaskFailed {
        #[serde(flatten)]
        data: CustomTaskFailedResponse,
    },

    #[serde(rename = "CUSTOM_TASK_TIMED_OUT")]
    #[non_exhaustive]
    CustomTaskTimedOut {
        #[serde(flatten)]
        data: CustomTaskTimedOutResponse,
    },

    #[serde(rename = "CUSTOM_TASK_CANCELED")]
    #[non_exhaustive]
    CustomTaskCanceled {
        #[serde(flatten)]
        data: CustomTaskCanceledResponse,
    },

    #[serde(rename = "ACTIVITY_TASK_STARTED")]
    #[non_exhaustive]
    ActivityTaskStarted {
        #[serde(flatten)]
        data: ActivityTaskStartedResponse,
    },

    #[serde(rename = "ACTIVITY_TASK_COMPLETED")]
    #[non_exhaustive]
    ActivityTaskCompleted {
        #[serde(flatten)]
        data: ActivityTaskCompletedResponse,
    },

    #[serde(rename = "ACTIVITY_TASK_RETRYING")]
    #[non_exhaustive]
    ActivityTaskRetrying {
        #[serde(flatten)]
        data: ActivityTaskRetryingResponse,
    },

    #[serde(rename = "ACTIVITY_TASK_FAILED")]
    #[non_exhaustive]
    ActivityTaskFailed {
        #[serde(flatten)]
        data: ActivityTaskFailedResponse,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ListWorkflowEventResponseEventsItem {
    pub fn workflow_execution_started(data: WorkflowExecutionStartedResponse) -> Self {
        Self::WorkflowExecutionStarted { data }
    }

    pub fn workflow_execution_completed(data: WorkflowExecutionCompletedResponse) -> Self {
        Self::WorkflowExecutionCompleted { data }
    }

    pub fn workflow_execution_failed(data: WorkflowExecutionFailedResponse) -> Self {
        Self::WorkflowExecutionFailed { data }
    }

    pub fn workflow_execution_canceled(data: WorkflowExecutionCanceledResponse) -> Self {
        Self::WorkflowExecutionCanceled { data }
    }

    pub fn workflow_execution_continued_as_new(
        data: WorkflowExecutionContinuedAsNewResponse,
    ) -> Self {
        Self::WorkflowExecutionContinuedAsNew { data }
    }

    pub fn workflow_task_timed_out(data: WorkflowTaskTimedOutResponse) -> Self {
        Self::WorkflowTaskTimedOut { data }
    }

    pub fn workflow_task_failed(data: WorkflowTaskFailedResponse) -> Self {
        Self::WorkflowTaskFailed { data }
    }

    pub fn custom_task_started(data: CustomTaskStartedResponse) -> Self {
        Self::CustomTaskStarted { data }
    }

    pub fn custom_task_in_progress(data: CustomTaskInProgressResponse) -> Self {
        Self::CustomTaskInProgress { data }
    }

    pub fn custom_task_completed(data: CustomTaskCompletedResponse) -> Self {
        Self::CustomTaskCompleted { data }
    }

    pub fn custom_task_failed(data: CustomTaskFailedResponse) -> Self {
        Self::CustomTaskFailed { data }
    }

    pub fn custom_task_timed_out(data: CustomTaskTimedOutResponse) -> Self {
        Self::CustomTaskTimedOut { data }
    }

    pub fn custom_task_canceled(data: CustomTaskCanceledResponse) -> Self {
        Self::CustomTaskCanceled { data }
    }

    pub fn activity_task_started(data: ActivityTaskStartedResponse) -> Self {
        Self::ActivityTaskStarted { data }
    }

    pub fn activity_task_completed(data: ActivityTaskCompletedResponse) -> Self {
        Self::ActivityTaskCompleted { data }
    }

    pub fn activity_task_retrying(data: ActivityTaskRetryingResponse) -> Self {
        Self::ActivityTaskRetrying { data }
    }

    pub fn activity_task_failed(data: ActivityTaskFailedResponse) -> Self {
        Self::ActivityTaskFailed { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
