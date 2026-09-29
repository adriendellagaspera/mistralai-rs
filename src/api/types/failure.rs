pub use crate::prelude::*;

/// Represents an error or exception that occurred during execution.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Failure {
    /// A human-readable description of the failure.
    #[serde(default)]
    pub message: String,
}

impl Failure {
    pub fn builder() -> FailureBuilder {
        <FailureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FailureBuilder {
    message: Option<String>,
}

impl FailureBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Failure`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](FailureBuilder::message)
    pub fn build(self) -> Result<Failure, BuildError> {
        Ok(Failure {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
