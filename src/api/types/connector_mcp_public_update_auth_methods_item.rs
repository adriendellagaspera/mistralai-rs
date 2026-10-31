pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ConnectorMcpPublicUpdateAuthMethodsItem {
    ConnectorMcpPublicUpdateAuthMethodsItemZero(ConnectorMcpPublicUpdateAuthMethodsItemZero),

    AuthenticationMethodCreateOrUpdateRequest(AuthenticationMethodCreateOrUpdateRequest),
}

impl ConnectorMcpPublicUpdateAuthMethodsItem {
    pub fn is_connector_mcp_public_update_auth_methods_item_zero(&self) -> bool {
        matches!(self, Self::ConnectorMcpPublicUpdateAuthMethodsItemZero(_))
    }

    pub fn is_authentication_method_create_or_update_request(&self) -> bool {
        matches!(self, Self::AuthenticationMethodCreateOrUpdateRequest(_))
    }

    pub fn as_connector_mcp_public_update_auth_methods_item_zero(
        &self,
    ) -> Option<&ConnectorMcpPublicUpdateAuthMethodsItemZero> {
        match self {
            Self::ConnectorMcpPublicUpdateAuthMethodsItemZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_connector_mcp_public_update_auth_methods_item_zero(
        self,
    ) -> Option<ConnectorMcpPublicUpdateAuthMethodsItemZero> {
        match self {
            Self::ConnectorMcpPublicUpdateAuthMethodsItemZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_authentication_method_create_or_update_request(
        &self,
    ) -> Option<&AuthenticationMethodCreateOrUpdateRequest> {
        match self {
            Self::AuthenticationMethodCreateOrUpdateRequest(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_authentication_method_create_or_update_request(
        self,
    ) -> Option<AuthenticationMethodCreateOrUpdateRequest> {
        match self {
            Self::AuthenticationMethodCreateOrUpdateRequest(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ConnectorMcpPublicUpdateAuthMethodsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectorMcpPublicUpdateAuthMethodsItemZero(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::AuthenticationMethodCreateOrUpdateRequest(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
