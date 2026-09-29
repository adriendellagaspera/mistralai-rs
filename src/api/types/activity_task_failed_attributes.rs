pub use crate::prelude::*;

/// Attributes for activity task failed events (final failure after all retries).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ActivityTaskFailedAttributes {
    /// Unique identifier for the activity task within the workflow.
    #[serde(default)]
    pub task_id: String,
    /// The registered name of the activity being executed.
    #[serde(default)]
    pub activity_name: String,
    /// The final attempt number that failed (1-indexed).
    #[serde(default)]
    pub attempt: i64,
    /// Details about the failure that caused the activity to fail.
    #[serde(default)]
    pub failure: Failure,
}

impl ActivityTaskFailedAttributes {
    pub fn builder() -> ActivityTaskFailedAttributesBuilder {
        <ActivityTaskFailedAttributesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActivityTaskFailedAttributesBuilder {
    task_id: Option<String>,
    activity_name: Option<String>,
    attempt: Option<i64>,
    failure: Option<Failure>,
}

impl ActivityTaskFailedAttributesBuilder {
    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

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

    /// Consumes the builder and constructs a [`ActivityTaskFailedAttributes`].
    /// This method will fail if any of the following fields are not set:
    /// - [`task_id`](ActivityTaskFailedAttributesBuilder::task_id)
    /// - [`activity_name`](ActivityTaskFailedAttributesBuilder::activity_name)
    /// - [`attempt`](ActivityTaskFailedAttributesBuilder::attempt)
    /// - [`failure`](ActivityTaskFailedAttributesBuilder::failure)
    pub fn build(self) -> Result<ActivityTaskFailedAttributes, BuildError> {
        Ok(ActivityTaskFailedAttributes {
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
            activity_name: self
                .activity_name
                .ok_or_else(|| BuildError::missing_field("activity_name"))?,
            attempt: self
                .attempt
                .ok_or_else(|| BuildError::missing_field("attempt"))?,
            failure: self
                .failure
                .ok_or_else(|| BuildError::missing_field("failure"))?,
        })
    }
}
