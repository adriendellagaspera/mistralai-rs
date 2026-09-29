pub use crate::prelude::*;

/// Query parameters for connector_list_organization_credentials_v1
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectorListOrganizationCredentialsV1QueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<OutboundAuthenticationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_default: Option<bool>,
}

impl ConnectorListOrganizationCredentialsV1QueryRequest {
    pub fn builder() -> ConnectorListOrganizationCredentialsV1QueryRequestBuilder {
        <ConnectorListOrganizationCredentialsV1QueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorListOrganizationCredentialsV1QueryRequestBuilder {
    auth_type: Option<OutboundAuthenticationType>,
    fetch_default: Option<bool>,
}

impl ConnectorListOrganizationCredentialsV1QueryRequestBuilder {
    pub fn auth_type(mut self, value: OutboundAuthenticationType) -> Self {
        self.auth_type = Some(value);
        self
    }

    pub fn fetch_default(mut self, value: bool) -> Self {
        self.fetch_default = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectorListOrganizationCredentialsV1QueryRequest`].
    pub fn build(self) -> Result<ConnectorListOrganizationCredentialsV1QueryRequest, BuildError> {
        Ok(ConnectorListOrganizationCredentialsV1QueryRequest {
            auth_type: self.auth_type,
            fetch_default: self.fetch_default,
        })
    }
}
