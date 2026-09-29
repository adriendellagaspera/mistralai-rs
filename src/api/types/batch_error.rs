pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default)]
    pub message: String,
}

impl BatchError {
    pub fn builder() -> BatchErrorBuilder {
        <BatchErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchErrorBuilder {
    count: Option<i64>,
    message: Option<String>,
}

impl BatchErrorBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BatchError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](BatchErrorBuilder::message)
    pub fn build(self) -> Result<BatchError, BuildError> {
        Ok(BatchError {
            count: self.count,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
