pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesError {
    pub code: ResponsesErrorCode,
    #[serde(default)]
    pub message: String,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesError {
    pub fn builder() -> ResponsesErrorBuilder {
        <ResponsesErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesErrorBuilder {
    code: Option<ResponsesErrorCode>,
    message: Option<String>,
}

impl ResponsesErrorBuilder {
    pub fn code(mut self, value: ResponsesErrorCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResponsesError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ResponsesErrorBuilder::code)
    /// - [`message`](ResponsesErrorBuilder::message)
    pub fn build(self) -> Result<ResponsesError, BuildError> {
        Ok(ResponsesError {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            extra: Default::default(),
        })
    }
}
