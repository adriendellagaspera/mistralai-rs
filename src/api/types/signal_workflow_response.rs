pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SignalWorkflowResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl SignalWorkflowResponse {
    pub fn builder() -> SignalWorkflowResponseBuilder {
        <SignalWorkflowResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignalWorkflowResponseBuilder {
    message: Option<String>,
}

impl SignalWorkflowResponseBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SignalWorkflowResponse`].
    pub fn build(self) -> Result<SignalWorkflowResponse, BuildError> {
        Ok(SignalWorkflowResponse {
            message: self.message,
        })
    }
}
