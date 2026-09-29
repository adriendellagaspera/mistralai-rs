pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StreamEventSseErrorData {
    #[serde(default)]
    pub error: String,
    #[serde(default)]
    pub reason: String,
}

impl StreamEventSseErrorData {
    pub fn builder() -> StreamEventSseErrorDataBuilder {
        <StreamEventSseErrorDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamEventSseErrorDataBuilder {
    error: Option<String>,
    reason: Option<String>,
}

impl StreamEventSseErrorDataBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StreamEventSseErrorData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](StreamEventSseErrorDataBuilder::error)
    /// - [`reason`](StreamEventSseErrorDataBuilder::reason)
    pub fn build(self) -> Result<StreamEventSseErrorData, BuildError> {
        Ok(StreamEventSseErrorData {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
