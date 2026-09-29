pub use crate::prelude::*;

/// Attributes for custom task failed events.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CustomTaskFailedAttributes {
    /// Unique identifier for the custom task within the workflow.
    #[serde(default)]
    pub custom_task_id: String,
    /// The type/category of the custom task (e.g., 'llm_call', 'api_request').
    #[serde(default)]
    pub custom_task_type: String,
    /// Details about the failure that caused the task to fail.
    #[serde(default)]
    pub failure: Failure,
}

impl CustomTaskFailedAttributes {
    pub fn builder() -> CustomTaskFailedAttributesBuilder {
        <CustomTaskFailedAttributesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomTaskFailedAttributesBuilder {
    custom_task_id: Option<String>,
    custom_task_type: Option<String>,
    failure: Option<Failure>,
}

impl CustomTaskFailedAttributesBuilder {
    pub fn custom_task_id(mut self, value: impl Into<String>) -> Self {
        self.custom_task_id = Some(value.into());
        self
    }

    pub fn custom_task_type(mut self, value: impl Into<String>) -> Self {
        self.custom_task_type = Some(value.into());
        self
    }

    pub fn failure(mut self, value: Failure) -> Self {
        self.failure = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomTaskFailedAttributes`].
    /// This method will fail if any of the following fields are not set:
    /// - [`custom_task_id`](CustomTaskFailedAttributesBuilder::custom_task_id)
    /// - [`custom_task_type`](CustomTaskFailedAttributesBuilder::custom_task_type)
    /// - [`failure`](CustomTaskFailedAttributesBuilder::failure)
    pub fn build(self) -> Result<CustomTaskFailedAttributes, BuildError> {
        Ok(CustomTaskFailedAttributes {
            custom_task_id: self
                .custom_task_id
                .ok_or_else(|| BuildError::missing_field("custom_task_id"))?,
            custom_task_type: self
                .custom_task_type
                .ok_or_else(|| BuildError::missing_field("custom_task_type"))?,
            failure: self
                .failure
                .ok_or_else(|| BuildError::missing_field("failure"))?,
        })
    }
}
