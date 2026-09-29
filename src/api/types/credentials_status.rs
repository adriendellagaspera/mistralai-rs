pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CredentialsStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_http_code: Option<HttpStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<CredentialsStatusErrorReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_checked_at: Option<DateTime<FixedOffset>>,
    pub status_type: AuthStatus,
}

impl CredentialsStatus {
    pub fn builder() -> CredentialsStatusBuilder {
        <CredentialsStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CredentialsStatusBuilder {
    error_http_code: Option<HttpStatus>,
    error_message: Option<CredentialsStatusErrorReason>,
    last_checked_at: Option<DateTime<FixedOffset>>,
    status_type: Option<AuthStatus>,
}

impl CredentialsStatusBuilder {
    pub fn error_http_code(mut self, value: HttpStatus) -> Self {
        self.error_http_code = Some(value);
        self
    }

    pub fn error_message(mut self, value: CredentialsStatusErrorReason) -> Self {
        self.error_message = Some(value);
        self
    }

    pub fn last_checked_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_checked_at = Some(value);
        self
    }

    pub fn status_type(mut self, value: AuthStatus) -> Self {
        self.status_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CredentialsStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status_type`](CredentialsStatusBuilder::status_type)
    pub fn build(self) -> Result<CredentialsStatus, BuildError> {
        Ok(CredentialsStatus {
            error_http_code: self.error_http_code,
            error_message: self.error_message,
            last_checked_at: self.last_checked_at,
            status_type: self
                .status_type
                .ok_or_else(|| BuildError::missing_field("status_type"))?,
        })
    }
}
