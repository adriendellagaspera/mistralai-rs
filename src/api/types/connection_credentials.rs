pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectionCredentials {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth: Option<OAuth2Token>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearer_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_installation_id: Option<String>,
}

impl ConnectionCredentials {
    pub fn builder() -> ConnectionCredentialsBuilder {
        <ConnectionCredentialsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionCredentialsBuilder {
    oauth: Option<OAuth2Token>,
    headers: Option<HashMap<String, Option<String>>>,
    bearer_token: Option<String>,
    github_installation_id: Option<String>,
}

impl ConnectionCredentialsBuilder {
    pub fn oauth(mut self, value: OAuth2Token) -> Self {
        self.oauth = Some(value);
        self
    }

    pub fn headers(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn bearer_token(mut self, value: impl Into<String>) -> Self {
        self.bearer_token = Some(value.into());
        self
    }

    pub fn github_installation_id(mut self, value: impl Into<String>) -> Self {
        self.github_installation_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectionCredentials`].
    pub fn build(self) -> Result<ConnectionCredentials, BuildError> {
        Ok(ConnectionCredentials {
            oauth: self.oauth,
            headers: self.headers,
            bearer_token: self.bearer_token,
            github_installation_id: self.github_installation_id,
        })
    }
}
