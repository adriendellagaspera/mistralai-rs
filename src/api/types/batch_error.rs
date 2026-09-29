pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchError {
    #[serde(default)]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl BatchError {
    pub fn builder() -> BatchErrorBuilder {
        <BatchErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchErrorBuilder {
    message: Option<String>,
    count: Option<i64>,
}

impl BatchErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](BatchErrorBuilder::message)
    pub fn build(self) -> Result<BatchError, BuildError> {
        Ok(BatchError {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            count: self.count,
        })
    }
}
