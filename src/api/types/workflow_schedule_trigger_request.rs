pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowScheduleTriggerRequest {
    /// Optional overlap policy override to use for the immediate trigger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlap: Option<ScheduleOverlapPolicy>,
}

impl WorkflowScheduleTriggerRequest {
    pub fn builder() -> WorkflowScheduleTriggerRequestBuilder {
        <WorkflowScheduleTriggerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowScheduleTriggerRequestBuilder {
    overlap: Option<ScheduleOverlapPolicy>,
}

impl WorkflowScheduleTriggerRequestBuilder {
    pub fn overlap(mut self, value: ScheduleOverlapPolicy) -> Self {
        self.overlap = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowScheduleTriggerRequest`].
    pub fn build(self) -> Result<WorkflowScheduleTriggerRequest, BuildError> {
        Ok(WorkflowScheduleTriggerRequest {
            overlap: self.overlap,
        })
    }
}
