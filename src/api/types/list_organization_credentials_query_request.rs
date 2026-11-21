pub use crate::prelude::*;

/// Query parameters for list_organization_credentials
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListOrganizationCredentialsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<AuthenticationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_default: Option<bool>,
}

impl ListOrganizationCredentialsQueryRequest {
    pub fn builder() -> ListOrganizationCredentialsQueryRequestBuilder {
        <ListOrganizationCredentialsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrganizationCredentialsQueryRequestBuilder {
    auth_type: Option<AuthenticationType>,
    fetch_default: Option<bool>,
}

impl ListOrganizationCredentialsQueryRequestBuilder {
    pub fn auth_type(mut self, value: AuthenticationType) -> Self {
        self.auth_type = Some(value);
        self
    }

    pub fn fetch_default(mut self, value: bool) -> Self {
        self.fetch_default = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOrganizationCredentialsQueryRequest`].
    pub fn build(self) -> Result<ListOrganizationCredentialsQueryRequest, BuildError> {
        Ok(ListOrganizationCredentialsQueryRequest {
            auth_type: self.auth_type,
            fetch_default: self.fetch_default,
        })
    }
}
