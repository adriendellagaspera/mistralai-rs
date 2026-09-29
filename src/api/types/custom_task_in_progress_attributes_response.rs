pub use crate::prelude::*;

/// Attributes for custom task in-progress events with streaming updates.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomTaskInProgressAttributesResponse {
    /// Unique identifier for the custom task within the workflow.
    #[serde(default)]
    pub custom_task_id: String,
    /// The type/category of the custom task (e.g., 'llm_call', 'api_request').
    #[serde(default)]
    pub custom_task_type: String,
    /// The current state or incremental update for the task.
    pub payload: CustomTaskInProgressAttributesResponsePayload,
}

impl CustomTaskInProgressAttributesResponse {
    pub fn builder() -> CustomTaskInProgressAttributesResponseBuilder {
        <CustomTaskInProgressAttributesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomTaskInProgressAttributesResponseBuilder {
    custom_task_id: Option<String>,
    custom_task_type: Option<String>,
    payload: Option<CustomTaskInProgressAttributesResponsePayload>,
}

impl CustomTaskInProgressAttributesResponseBuilder {
    pub fn custom_task_id(mut self, value: impl Into<String>) -> Self {
        self.custom_task_id = Some(value.into());
        self
    }

    pub fn custom_task_type(mut self, value: impl Into<String>) -> Self {
        self.custom_task_type = Some(value.into());
        self
    }

    pub fn payload(mut self, value: CustomTaskInProgressAttributesResponsePayload) -> Self {
        self.payload = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomTaskInProgressAttributesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`custom_task_id`](CustomTaskInProgressAttributesResponseBuilder::custom_task_id)
    /// - [`custom_task_type`](CustomTaskInProgressAttributesResponseBuilder::custom_task_type)
    /// - [`payload`](CustomTaskInProgressAttributesResponseBuilder::payload)
    pub fn build(self) -> Result<CustomTaskInProgressAttributesResponse, BuildError> {
        Ok(CustomTaskInProgressAttributesResponse {
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
