pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ObservabilityErrorDetail {
    #[serde(default)]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<ObservabilityErrorCode>,
}

impl ObservabilityErrorDetail {
    pub fn builder() -> ObservabilityErrorDetailBuilder {
        <ObservabilityErrorDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ObservabilityErrorDetailBuilder {
    message: Option<String>,
    error_code: Option<ObservabilityErrorCode>,
}

impl ObservabilityErrorDetailBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn error_code(mut self, value: ObservabilityErrorCode) -> Self {
        self.error_code = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ObservabilityErrorDetail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](ObservabilityErrorDetailBuilder::message)
    pub fn build(self) -> Result<ObservabilityErrorDetail, BuildError> {
        Ok(ObservabilityErrorDetail {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            error_code: self.error_code,
        })
    }
}
