pub use crate::prelude::*;

/// Attributes for activity task started events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActivityTaskStartedAttributesResponse {
    /// Unique identifier for the activity task within the workflow.
    #[serde(default)]
    pub task_id: String,
    /// The registered name of the activity being executed.
    #[serde(default)]
    pub activity_name: String,
    /// The input arguments passed to the activity.
    pub input: JsonPayloadResponse,
}

impl ActivityTaskStartedAttributesResponse {
    pub fn builder() -> ActivityTaskStartedAttributesResponseBuilder {
        <ActivityTaskStartedAttributesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActivityTaskStartedAttributesResponseBuilder {
    task_id: Option<String>,
    activity_name: Option<String>,
    input: Option<JsonPayloadResponse>,
}

impl ActivityTaskStartedAttributesResponseBuilder {
    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    pub fn activity_name(mut self, value: impl Into<String>) -> Self {
        self.activity_name = Some(value.into());
        self
    }

    pub fn input(mut self, value: JsonPayloadResponse) -> Self {
        self.input = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ActivityTaskStartedAttributesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`task_id`](ActivityTaskStartedAttributesResponseBuilder::task_id)
    /// - [`activity_name`](ActivityTaskStartedAttributesResponseBuilder::activity_name)
    /// - [`input`](ActivityTaskStartedAttributesResponseBuilder::input)
    pub fn build(self) -> Result<ActivityTaskStartedAttributesResponse, BuildError> {
        Ok(ActivityTaskStartedAttributesResponse {
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
            activity_name: self
                .activity_name
                .ok_or_else(|| BuildError::missing_field("activity_name"))?,
            input: self
                .input
                .ok_or_else(|| BuildError::missing_field("input"))?,
        })
    }
}
