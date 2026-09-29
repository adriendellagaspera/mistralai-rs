pub use crate::prelude::*;

/// Attributes for activity task retrying events.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ActivityTaskRetryingAttributes {
    /// The registered name of the activity being executed.
    #[serde(default)]
    pub activity_name: String,
    /// The attempt number that failed (1-indexed).
    #[serde(default)]
    pub attempt: i64,
    /// Details about the failure that caused the retry.
    #[serde(default)]
    pub failure: Failure,
    /// Unique identifier for the activity task within the workflow.
    #[serde(default)]
    pub task_id: String,
}

impl ActivityTaskRetryingAttributes {
    pub fn builder() -> ActivityTaskRetryingAttributesBuilder {
        <ActivityTaskRetryingAttributesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActivityTaskRetryingAttributesBuilder {
    activity_name: Option<String>,
    attempt: Option<i64>,
    failure: Option<Failure>,
    task_id: Option<String>,
}

impl ActivityTaskRetryingAttributesBuilder {
    pub fn activity_name(mut self, value: impl Into<String>) -> Self {
        self.activity_name = Some(value.into());
        self
    }

    pub fn attempt(mut self, value: i64) -> Self {
        self.attempt = Some(value);
        self
    }

    pub fn failure(mut self, value: Failure) -> Self {
        self.failure = Some(value);
        self
    }

    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ActivityTaskRetryingAttributes`].
    /// This method will fail if any of the following fields are not set:
    /// - [`activity_name`](ActivityTaskRetryingAttributesBuilder::activity_name)
    /// - [`attempt`](ActivityTaskRetryingAttributesBuilder::attempt)
    /// - [`failure`](ActivityTaskRetryingAttributesBuilder::failure)
    /// - [`task_id`](ActivityTaskRetryingAttributesBuilder::task_id)
    pub fn build(self) -> Result<ActivityTaskRetryingAttributes, BuildError> {
        Ok(ActivityTaskRetryingAttributes {
            activity_name: self
                .activity_name
                .ok_or_else(|| BuildError::missing_field("activity_name"))?,
            attempt: self
                .attempt
                .ok_or_else(|| BuildError::missing_field("attempt"))?,
            failure: self
                .failure
                .ok_or_else(|| BuildError::missing_field("failure"))?,
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
        })
    }
}
