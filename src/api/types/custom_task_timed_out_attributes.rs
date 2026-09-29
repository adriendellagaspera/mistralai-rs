pub use crate::prelude::*;

/// Attributes for custom task timed out events.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CustomTaskTimedOutAttributes {
    /// Unique identifier for the custom task within the workflow.
    #[serde(default)]
    pub custom_task_id: String,
    /// The type/category of the custom task (e.g., 'llm_call', 'api_request').
    #[serde(default)]
    pub custom_task_type: String,
    /// The type of timeout that occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_type: Option<String>,
}

impl CustomTaskTimedOutAttributes {
    pub fn builder() -> CustomTaskTimedOutAttributesBuilder {
        <CustomTaskTimedOutAttributesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomTaskTimedOutAttributesBuilder {
    custom_task_id: Option<String>,
    custom_task_type: Option<String>,
    timeout_type: Option<String>,
}

impl CustomTaskTimedOutAttributesBuilder {
    pub fn custom_task_id(mut self, value: impl Into<String>) -> Self {
        self.custom_task_id = Some(value.into());
        self
    }

    pub fn custom_task_type(mut self, value: impl Into<String>) -> Self {
        self.custom_task_type = Some(value.into());
        self
    }

    pub fn timeout_type(mut self, value: impl Into<String>) -> Self {
        self.timeout_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CustomTaskTimedOutAttributes`].
    /// This method will fail if any of the following fields are not set:
    /// - [`custom_task_id`](CustomTaskTimedOutAttributesBuilder::custom_task_id)
    /// - [`custom_task_type`](CustomTaskTimedOutAttributesBuilder::custom_task_type)
    pub fn build(self) -> Result<CustomTaskTimedOutAttributes, BuildError> {
        Ok(CustomTaskTimedOutAttributes {
            custom_task_id: self
                .custom_task_id
                .ok_or_else(|| BuildError::missing_field("custom_task_id"))?,
            custom_task_type: self
                .custom_task_type
                .ok_or_else(|| BuildError::missing_field("custom_task_type"))?,
            timeout_type: self.timeout_type,
        })
    }
}
