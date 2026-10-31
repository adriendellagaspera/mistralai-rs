pub use crate::prelude::*;

/// A single field-level validation failure within a rejected document.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentFieldErrorResponse {
    #[serde(default)]
    pub location: Vec<DocumentFieldErrorResponseLocationItem>,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub r#type: String,
}

impl DocumentFieldErrorResponse {
    pub fn builder() -> DocumentFieldErrorResponseBuilder {
        <DocumentFieldErrorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentFieldErrorResponseBuilder {
    location: Option<Vec<DocumentFieldErrorResponseLocationItem>>,
    message: Option<String>,
    r#type: Option<String>,
}

impl DocumentFieldErrorResponseBuilder {
    pub fn location(mut self, value: Vec<DocumentFieldErrorResponseLocationItem>) -> Self {
        self.location = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DocumentFieldErrorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`location`](DocumentFieldErrorResponseBuilder::location)
    /// - [`message`](DocumentFieldErrorResponseBuilder::message)
    /// - [`r#type`](DocumentFieldErrorResponseBuilder::r#type)
    pub fn build(self) -> Result<DocumentFieldErrorResponse, BuildError> {
        Ok(DocumentFieldErrorResponse {
            location: self
                .location
                .ok_or_else(|| BuildError::missing_field("location"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
