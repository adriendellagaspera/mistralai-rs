pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OAuth2Token {
    #[serde(default)]
    pub access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_type: Option<OAuth2TokenTokenType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
}

impl OAuth2Token {
    pub fn builder() -> OAuth2TokenBuilder {
        <OAuth2TokenBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OAuth2TokenBuilder {
    access_token: Option<String>,
    token_type: Option<OAuth2TokenTokenType>,
    expires_in: Option<i64>,
    scope: Option<String>,
    refresh_token: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
}

impl OAuth2TokenBuilder {
    pub fn access_token(mut self, value: impl Into<String>) -> Self {
        self.access_token = Some(value.into());
        self
    }

    pub fn token_type(mut self, value: OAuth2TokenTokenType) -> Self {
        self.token_type = Some(value);
        self
    }

    pub fn expires_in(mut self, value: i64) -> Self {
        self.expires_in = Some(value);
        self
    }

    pub fn scope(mut self, value: impl Into<String>) -> Self {
        self.scope = Some(value.into());
        self
    }

    pub fn refresh_token(mut self, value: impl Into<String>) -> Self {
        self.refresh_token = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OAuth2Token`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access_token`](OAuth2TokenBuilder::access_token)
    pub fn build(self) -> Result<OAuth2Token, BuildError> {
        Ok(OAuth2Token {
            access_token: self
                .access_token
                .ok_or_else(|| BuildError::missing_field("access_token"))?,
            token_type: self.token_type,
            expires_in: self.expires_in,
            scope: self.scope,
            refresh_token: self.refresh_token,
            expires_at: self.expires_at,
        })
    }
}
