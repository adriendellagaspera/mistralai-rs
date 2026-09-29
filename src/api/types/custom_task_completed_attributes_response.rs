pub use crate::prelude::*;

/// Attributes for custom task completed events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomTaskCompletedAttributesResponse {
    /// Unique identifier for the custom task within the workflow.
    #[serde(default)]
    pub custom_task_id: String,
    /// The type/category of the custom task (e.g., 'llm_call', 'api_request').
    #[serde(default)]
    pub custom_task_type: String,
    /// The final result of the custom task.
    pub payload: JsonPayloadResponse,
}

impl CustomTaskCompletedAttributesResponse {
    pub fn builder() -> CustomTaskCompletedAttributesResponseBuilder {
        <CustomTaskCompletedAttributesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomTaskCompletedAttributesResponseBuilder {
    custom_task_id: Option<String>,
    custom_task_type: Option<String>,
    payload: Option<JsonPayloadResponse>,
}

impl CustomTaskCompletedAttributesResponseBuilder {
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

    /// Consumes the builder and constructs a [`CustomTaskCompletedAttributesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`custom_task_id`](CustomTaskCompletedAttributesResponseBuilder::custom_task_id)
    /// - [`custom_task_type`](CustomTaskCompletedAttributesResponseBuilder::custom_task_type)
    /// - [`payload`](CustomTaskCompletedAttributesResponseBuilder::payload)
    pub fn build(self) -> Result<CustomTaskCompletedAttributesResponse, BuildError> {
        Ok(CustomTaskCompletedAttributesResponse {
            custom_task_id: self
                .custom_task_id
                .ok_or_else(|| BuildError::missing_field("custom_task_id"))?,
            custom_task_type: self
                .custom_task_type
                .ok_or_else(|| BuildError::missing_field("custom_task_type"))?,
            payload: self
                .payload
                .ok_or_else(|| BuildError::missing_field("payload"))?,
        })
    }
}
