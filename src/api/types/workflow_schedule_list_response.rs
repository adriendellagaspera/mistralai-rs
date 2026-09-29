pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowScheduleListResponse {
    /// Token for the next page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// A list of workflow schedules
    #[serde(default)]
    pub schedules: Vec<ScheduleDefinitionOutput>,
}

impl WorkflowScheduleListResponse {
    pub fn builder() -> WorkflowScheduleListResponseBuilder {
        <WorkflowScheduleListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowScheduleListResponseBuilder {
    next_page_token: Option<String>,
    schedules: Option<Vec<ScheduleDefinitionOutput>>,
}

impl WorkflowScheduleListResponseBuilder {
    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    pub fn schedules(mut self, value: Vec<ScheduleDefinitionOutput>) -> Self {
        self.schedules = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowScheduleListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`schedules`](WorkflowScheduleListResponseBuilder::schedules)
    pub fn build(self) -> Result<WorkflowScheduleListResponse, BuildError> {
        Ok(WorkflowScheduleListResponse {
            next_page_token: self.next_page_token,
            schedules: self
                .schedules
                .ok_or_else(|| BuildError::missing_field("schedules"))?,
        })
    }
}
