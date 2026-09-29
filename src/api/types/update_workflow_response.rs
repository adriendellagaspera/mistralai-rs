pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateWorkflowResponse {
    pub result: serde_json::Value,
    #[serde(default)]
    pub update_name: String,
}

impl UpdateWorkflowResponse {
    pub fn builder() -> UpdateWorkflowResponseBuilder {
        <UpdateWorkflowResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWorkflowResponseBuilder {
    result: Option<serde_json::Value>,
    update_name: Option<String>,
}

impl UpdateWorkflowResponseBuilder {
    pub fn result(mut self, value: serde_json::Value) -> Self {
        self.result = Some(value);
        self
    }

    pub fn update_name(mut self, value: impl Into<String>) -> Self {
        self.update_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateWorkflowResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`result`](UpdateWorkflowResponseBuilder::result)
    /// - [`update_name`](UpdateWorkflowResponseBuilder::update_name)
    pub fn build(self) -> Result<UpdateWorkflowResponse, BuildError> {
        Ok(UpdateWorkflowResponse {
            result: self
                .result
                .ok_or_else(|| BuildError::missing_field("result"))?,
            update_name: self
                .update_name
                .ok_or_else(|| BuildError::missing_field("update_name"))?,
        })
    }
}
