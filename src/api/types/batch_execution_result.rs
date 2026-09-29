pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchExecutionResult {
    /// Error message if operation failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Status of the operation (success/failure)
    #[serde(default)]
    pub status: String,
}

impl BatchExecutionResult {
    pub fn builder() -> BatchExecutionResultBuilder {
        <BatchExecutionResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchExecutionResultBuilder {
    error: Option<String>,
    status: Option<String>,
}

impl BatchExecutionResultBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BatchExecutionResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](BatchExecutionResultBuilder::status)
    pub fn build(self) -> Result<BatchExecutionResult, BuildError> {
        Ok(BatchExecutionResult {
            error: self.error,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
