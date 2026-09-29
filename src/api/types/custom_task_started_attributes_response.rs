pub use crate::prelude::*;

/// Attributes for custom task started events.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CustomTaskStartedAttributesResponse {
    /// Unique identifier for the custom task within the workflow.
    #[serde(default)]
    pub custom_task_id: String,
    /// The type/category of the custom task (e.g., 'llm_call', 'api_request').
    #[serde(default)]
    pub custom_task_type: String,
    /// The initial state or payload for the custom task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<JsonPayloadResponse>,
}

impl CustomTaskStartedAttributesResponse {
    pub fn builder() -> CustomTaskStartedAttributesResponseBuilder {
        <CustomTaskStartedAttributesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomTaskStartedAttributesResponseBuilder {
    custom_task_id: Option<String>,
    custom_task_type: Option<String>,
    payload: Option<JsonPayloadResponse>,
}

impl CustomTaskStartedAttributesResponseBuilder {
    pub fn custom_task_id(mut self, value: impl Into<String>) -> Self {
        self.custom_task_id = Some(value.into());
        self
    }

    pub fn custom_task_type(mut self, value: impl Into<String>) -> Self {
        self.custom_task_type = Some(value.into());
        self
    }

    pub fn payload(mut self, value: JsonPayloadResponse) -> Self {
        self.payload = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomTaskStartedAttributesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`custom_task_id`](CustomTaskStartedAttributesResponseBuilder::custom_task_id)
    /// - [`custom_task_type`](CustomTaskStartedAttributesResponseBuilder::custom_task_type)
    pub fn build(self) -> Result<CustomTaskStartedAttributesResponse, BuildError> {
        Ok(CustomTaskStartedAttributesResponse {
            custom_task_id: self
                .custom_task_id
                .ok_or_else(|| BuildError::missing_field("custom_task_id"))?,
            custom_task_type: self
                .custom_task_type
                .ok_or_else(|| BuildError::missing_field("custom_task_type"))?,
            payload: self.payload,
        })
    }
}
