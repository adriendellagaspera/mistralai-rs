pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowScheduleResponse {
    /// The ID of the schedule
    #[serde(default)]
    pub schedule_id: String,
}

impl WorkflowScheduleResponse {
    pub fn builder() -> WorkflowScheduleResponseBuilder {
        <WorkflowScheduleResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowScheduleResponseBuilder {
    schedule_id: Option<String>,
}

impl WorkflowScheduleResponseBuilder {
    pub fn schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowScheduleResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`schedule_id`](WorkflowScheduleResponseBuilder::schedule_id)
    pub fn build(self) -> Result<WorkflowScheduleResponse, BuildError> {
        Ok(WorkflowScheduleResponse {
            schedule_id: self
                .schedule_id
                .ok_or_else(|| BuildError::missing_field("schedule_id"))?,
        })
    }
}
