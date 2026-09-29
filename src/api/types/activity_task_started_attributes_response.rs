pub use crate::prelude::*;

/// Attributes for activity task started events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActivityTaskStartedAttributesResponse {
    /// The registered name of the activity being executed.
    #[serde(default)]
    pub activity_name: String,
    /// The input arguments passed to the activity.
    pub input: JsonPayloadResponse,
    /// Unique identifier for the activity task within the workflow.
    #[serde(default)]
    pub task_id: String,
}

impl ActivityTaskStartedAttributesResponse {
    pub fn builder() -> ActivityTaskStartedAttributesResponseBuilder {
        <ActivityTaskStartedAttributesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActivityTaskStartedAttributesResponseBuilder {
    activity_name: Option<String>,
    input: Option<JsonPayloadResponse>,
    task_id: Option<String>,
}

impl ActivityTaskStartedAttributesResponseBuilder {
    pub fn activity_name(mut self, value: impl Into<String>) -> Self {
        self.activity_name = Some(value.into());
        self
    }

    pub fn input(mut self, value: JsonPayloadResponse) -> Self {
        self.input = Some(value);
        self
    }

    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ActivityTaskStartedAttributesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`activity_name`](ActivityTaskStartedAttributesResponseBuilder::activity_name)
    /// - [`input`](ActivityTaskStartedAttributesResponseBuilder::input)
    /// - [`task_id`](ActivityTaskStartedAttributesResponseBuilder::task_id)
    pub fn build(self) -> Result<ActivityTaskStartedAttributesResponse, BuildError> {
        Ok(ActivityTaskStartedAttributesResponse {
            activity_name: self
                .activity_name
                .ok_or_else(|| BuildError::missing_field("activity_name"))?,
            input: self
                .input
                .ok_or_else(|| BuildError::missing_field("input"))?,
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
        })
    }
}
