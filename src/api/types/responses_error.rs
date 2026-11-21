pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesError {
    pub code: ResponsesErrorCode,
    #[serde(default)]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misalignment: Option<ResponsesMisalignment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    pub reason: ResponsesErrorReason,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
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
    misalignment: Option<ResponsesMisalignment>,
    param: Option<String>,
    reason: Option<ResponsesErrorReason>,
    r#type: Option<String>,
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

    pub fn misalignment(mut self, value: ResponsesMisalignment) -> Self {
        self.misalignment = Some(value);
        self
    }

    pub fn param(mut self, value: impl Into<String>) -> Self {
        self.param = Some(value.into());
        self
    }

    pub fn reason(mut self, value: ResponsesErrorReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResponsesError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ResponsesErrorBuilder::code)
    /// - [`message`](ResponsesErrorBuilder::message)
    /// - [`reason`](ResponsesErrorBuilder::reason)
    pub fn build(self) -> Result<ResponsesError, BuildError> {
        Ok(ResponsesError {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            misalignment: self.misalignment,
            param: self.param,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            r#type: self.r#type,
            extra: Default::default(),
        })
    }
}
