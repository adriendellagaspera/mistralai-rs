pub use crate::prelude::*;

/// Attributes for activity task completed events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActivityTaskCompletedAttributesResponse {
    /// The registered name of the activity being executed.
    #[serde(default)]
    pub activity_name: String,
    /// The result returned by the activity.
    pub result: JsonPayloadResponse,
    /// Unique identifier for the activity task within the workflow.
    #[serde(default)]
    pub task_id: String,
}

impl ActivityTaskCompletedAttributesResponse {
    pub fn builder() -> ActivityTaskCompletedAttributesResponseBuilder {
        <ActivityTaskCompletedAttributesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActivityTaskCompletedAttributesResponseBuilder {
    activity_name: Option<String>,
    result: Option<JsonPayloadResponse>,
    task_id: Option<String>,
}

impl ActivityTaskCompletedAttributesResponseBuilder {
    pub fn activity_name(mut self, value: impl Into<String>) -> Self {
        self.activity_name = Some(value.into());
        self
    }

    pub fn result(mut self, value: JsonPayloadResponse) -> Self {
        self.result = Some(value);
        self
    }

    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ActivityTaskCompletedAttributesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`activity_name`](ActivityTaskCompletedAttributesResponseBuilder::activity_name)
    /// - [`result`](ActivityTaskCompletedAttributesResponseBuilder::result)
    /// - [`task_id`](ActivityTaskCompletedAttributesResponseBuilder::task_id)
    pub fn build(self) -> Result<ActivityTaskCompletedAttributesResponse, BuildError> {
        Ok(ActivityTaskCompletedAttributesResponse {
            activity_name: self
                .activity_name
                .ok_or_else(|| BuildError::missing_field("activity_name"))?,
            result: self
                .result
                .ok_or_else(|| BuildError::missing_field("result"))?,
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
        })
    }
}
