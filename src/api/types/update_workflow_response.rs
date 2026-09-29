pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateWorkflowResponse {
    #[serde(default)]
    pub update_name: String,
    pub result: serde_json::Value,
}

impl UpdateWorkflowResponse {
    pub fn builder() -> UpdateWorkflowResponseBuilder {
        <UpdateWorkflowResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWorkflowResponseBuilder {
    update_name: Option<String>,
    result: Option<serde_json::Value>,
}

impl UpdateWorkflowResponseBuilder {
    pub fn update_name(mut self, value: impl Into<String>) -> Self {
        self.update_name = Some(value.into());
        self
    }

    pub fn result(mut self, value: serde_json::Value) -> Self {
        self.result = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateWorkflowResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`update_name`](UpdateWorkflowResponseBuilder::update_name)
    /// - [`result`](UpdateWorkflowResponseBuilder::result)
    pub fn build(self) -> Result<UpdateWorkflowResponse, BuildError> {
        Ok(UpdateWorkflowResponse {
            update_name: self
                .update_name
                .ok_or_else(|| BuildError::missing_field("update_name"))?,
            result: self
                .result
                .ok_or_else(|| BuildError::missing_field("result"))?,
        })
    }
}
