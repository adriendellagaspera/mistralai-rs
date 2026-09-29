pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowSchedulePauseRequest {
    /// Optional note recorded in Temporal when pausing or resuming a schedule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl WorkflowSchedulePauseRequest {
    pub fn builder() -> WorkflowSchedulePauseRequestBuilder {
        <WorkflowSchedulePauseRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowSchedulePauseRequestBuilder {
    note: Option<String>,
}

impl WorkflowSchedulePauseRequestBuilder {
    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowSchedulePauseRequest`].
    pub fn build(self) -> Result<WorkflowSchedulePauseRequest, BuildError> {
        Ok(WorkflowSchedulePauseRequest { note: self.note })
    }
}
