pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectionCredentialsInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearer_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth: Option<OAuth2ClientCredentialsInput>,
}

impl ConnectionCredentialsInput {
    pub fn builder() -> ConnectionCredentialsInputBuilder {
        <ConnectionCredentialsInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionCredentialsInputBuilder {
    bearer_token: Option<String>,
    headers: Option<HashMap<String, Option<String>>>,
    oauth: Option<OAuth2ClientCredentialsInput>,
}

impl ConnectionCredentialsInputBuilder {
    pub fn bearer_token(mut self, value: impl Into<String>) -> Self {
        self.bearer_token = Some(value.into());
        self
    }

    pub fn headers(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn oauth(mut self, value: OAuth2ClientCredentialsInput) -> Self {
        self.oauth = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectionCredentialsInput`].
    pub fn build(self) -> Result<ConnectionCredentialsInput, BuildError> {
        Ok(ConnectionCredentialsInput {
            bearer_token: self.bearer_token,
            headers: self.headers,
            oauth: self.oauth,
        })
    }
}
