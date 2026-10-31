pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum UpdateHttpConnectorRequestAuthMethodsItem {
    UpdateHttpConnectorRequestAuthMethodsItemZero(UpdateHttpConnectorRequestAuthMethodsItemZero),

    AuthenticationMethodCreateOrUpdateRequest(AuthenticationMethodCreateOrUpdateRequest),
}

impl UpdateHttpConnectorRequestAuthMethodsItem {
    pub fn is_update_http_connector_request_auth_methods_item_zero(&self) -> bool {
        matches!(self, Self::UpdateHttpConnectorRequestAuthMethodsItemZero(_))
    }

    pub fn is_authentication_method_create_or_update_request(&self) -> bool {
        matches!(self, Self::AuthenticationMethodCreateOrUpdateRequest(_))
    }

    pub fn as_update_http_connector_request_auth_methods_item_zero(
        &self,
    ) -> Option<&UpdateHttpConnectorRequestAuthMethodsItemZero> {
        match self {
            Self::UpdateHttpConnectorRequestAuthMethodsItemZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_update_http_connector_request_auth_methods_item_zero(
        self,
    ) -> Option<UpdateHttpConnectorRequestAuthMethodsItemZero> {
        match self {
            Self::UpdateHttpConnectorRequestAuthMethodsItemZero(value) => Some(value),
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

impl fmt::Display for UpdateHttpConnectorRequestAuthMethodsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UpdateHttpConnectorRequestAuthMethodsItemZero(value) => write!(
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
