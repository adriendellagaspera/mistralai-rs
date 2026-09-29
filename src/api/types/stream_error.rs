pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StreamError {
    #[serde(default)]
    pub error: String,
    #[serde(default)]
    pub reason: String,
}

impl StreamError {
    pub fn builder() -> StreamErrorBuilder {
        <StreamErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamErrorBuilder {
    error: Option<String>,
    reason: Option<String>,
}

impl StreamErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StreamError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](StreamErrorBuilder::error)
    /// - [`reason`](StreamErrorBuilder::reason)
    pub fn build(self) -> Result<StreamError, BuildError> {
        Ok(StreamError {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
