pub use crate::prelude::*;

/// Query parameters for list_workspace_credentials
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListWorkspaceCredentialsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<AuthenticationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_default: Option<bool>,
}

impl ListWorkspaceCredentialsQueryRequest {
    pub fn builder() -> ListWorkspaceCredentialsQueryRequestBuilder {
        <ListWorkspaceCredentialsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWorkspaceCredentialsQueryRequestBuilder {
    auth_type: Option<AuthenticationType>,
    fetch_default: Option<bool>,
}

impl ListWorkspaceCredentialsQueryRequestBuilder {
    pub fn auth_type(mut self, value: AuthenticationType) -> Self {
        self.auth_type = Some(value);
        self
    }

    pub fn fetch_default(mut self, value: bool) -> Self {
        self.fetch_default = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListWorkspaceCredentialsQueryRequest`].
    pub fn build(self) -> Result<ListWorkspaceCredentialsQueryRequest, BuildError> {
        Ok(ListWorkspaceCredentialsQueryRequest {
            auth_type: self.auth_type,
            fetch_default: self.fetch_default,
        })
    }
}
