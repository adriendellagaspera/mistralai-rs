pub use crate::prelude::*;

/// Query parameters for list_user_credentials
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListUserCredentialsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<AuthenticationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_default: Option<bool>,
}

impl ListUserCredentialsQueryRequest {
    pub fn builder() -> ListUserCredentialsQueryRequestBuilder {
        <ListUserCredentialsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListUserCredentialsQueryRequestBuilder {
    auth_type: Option<AuthenticationType>,
    fetch_default: Option<bool>,
}

impl ListUserCredentialsQueryRequestBuilder {
    pub fn auth_type(mut self, value: AuthenticationType) -> Self {
        self.auth_type = Some(value);
        self
    }

    pub fn fetch_default(mut self, value: bool) -> Self {
        self.fetch_default = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListUserCredentialsQueryRequest`].
    pub fn build(self) -> Result<ListUserCredentialsQueryRequest, BuildError> {
        Ok(ListUserCredentialsQueryRequest {
            auth_type: self.auth_type,
            fetch_default: self.fetch_default,
        })
    }
}
