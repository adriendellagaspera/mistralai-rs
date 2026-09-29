pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ObservabilityErrorDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<ObservabilityErrorCode>,
    #[serde(default)]
    pub message: String,
}

impl ObservabilityErrorDetail {
    pub fn builder() -> ObservabilityErrorDetailBuilder {
        <ObservabilityErrorDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ObservabilityErrorDetailBuilder {
    error_code: Option<ObservabilityErrorCode>,
    message: Option<String>,
}

impl ObservabilityErrorDetailBuilder {
    pub fn error_code(mut self, value: ObservabilityErrorCode) -> Self {
        self.error_code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ObservabilityErrorDetail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](ObservabilityErrorDetailBuilder::message)
    pub fn build(self) -> Result<ObservabilityErrorDetail, BuildError> {
        Ok(ObservabilityErrorDetail {
            error_code: self.error_code,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
