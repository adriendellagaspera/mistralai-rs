pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowScheduleUpdateRequest {
    /// Partial schedule definition to update. Unset fields preserve existing values.
    #[serde(default)]
    pub schedule: PartialScheduleDefinition,
}

impl WorkflowScheduleUpdateRequest {
    pub fn builder() -> WorkflowScheduleUpdateRequestBuilder {
        <WorkflowScheduleUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowScheduleUpdateRequestBuilder {
    schedule: Option<PartialScheduleDefinition>,
}

impl WorkflowScheduleUpdateRequestBuilder {
    pub fn schedule(mut self, value: PartialScheduleDefinition) -> Self {
        self.schedule = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowScheduleUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`schedule`](WorkflowScheduleUpdateRequestBuilder::schedule)
    pub fn build(self) -> Result<WorkflowScheduleUpdateRequest, BuildError> {
        Ok(WorkflowScheduleUpdateRequest {
            schedule: self
                .schedule
                .ok_or_else(|| BuildError::missing_field("schedule"))?,
        })
    }
}
