pub use crate::prelude::*;

/// Attributes for custom task canceled events.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CustomTaskCanceledAttributes {
    /// Unique identifier for the custom task within the workflow.
    #[serde(default)]
    pub custom_task_id: String,
    /// The type/category of the custom task (e.g., 'llm_call', 'api_request').
    #[serde(default)]
    pub custom_task_type: String,
    /// Optional reason provided for the cancellation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl CustomTaskCanceledAttributes {
    pub fn builder() -> CustomTaskCanceledAttributesBuilder {
        <CustomTaskCanceledAttributesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomTaskCanceledAttributesBuilder {
    custom_task_id: Option<String>,
    custom_task_type: Option<String>,
    reason: Option<String>,
}

impl CustomTaskCanceledAttributesBuilder {
    pub fn custom_task_id(mut self, value: impl Into<String>) -> Self {
        self.custom_task_id = Some(value.into());
        self
    }

    pub fn custom_task_type(mut self, value: impl Into<String>) -> Self {
        self.custom_task_type = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CustomTaskCanceledAttributes`].
    /// This method will fail if any of the following fields are not set:
    /// - [`custom_task_id`](CustomTaskCanceledAttributesBuilder::custom_task_id)
    /// - [`custom_task_type`](CustomTaskCanceledAttributesBuilder::custom_task_type)
    pub fn build(self) -> Result<CustomTaskCanceledAttributes, BuildError> {
        Ok(CustomTaskCanceledAttributes {
            custom_task_id: self
                .custom_task_id
                .ok_or_else(|| BuildError::missing_field("custom_task_id"))?,
            custom_task_type: self
                .custom_task_type
                .ok_or_else(|| BuildError::missing_field("custom_task_type"))?,
            reason: self.reason,
        })
    }
}
