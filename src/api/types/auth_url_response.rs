pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AuthUrlResponse {
    #[serde(default)]
    pub auth_url: String,
    #[serde(default)]
    pub ttl: i64,
}

impl AuthUrlResponse {
    pub fn builder() -> AuthUrlResponseBuilder {
        <AuthUrlResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthUrlResponseBuilder {
    auth_url: Option<String>,
    ttl: Option<i64>,
}

impl AuthUrlResponseBuilder {
    pub fn auth_url(mut self, value: impl Into<String>) -> Self {
        self.auth_url = Some(value.into());
        self
    }

    pub fn ttl(mut self, value: i64) -> Self {
        self.ttl = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AuthUrlResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auth_url`](AuthUrlResponseBuilder::auth_url)
    /// - [`ttl`](AuthUrlResponseBuilder::ttl)
    pub fn build(self) -> Result<AuthUrlResponse, BuildError> {
        Ok(AuthUrlResponse {
            auth_url: self
                .auth_url
                .ok_or_else(|| BuildError::missing_field("auth_url"))?,
            ttl: self.ttl.ok_or_else(|| BuildError::missing_field("ttl"))?,
        })
    }
}
