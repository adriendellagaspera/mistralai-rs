pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum AuthenticationMethodCreateOrUpdateRequestMethodType {
    OutboundAuthenticationType(OutboundAuthenticationType),

    InboundAuthenticationType(InboundAuthenticationType),
}

impl AuthenticationMethodCreateOrUpdateRequestMethodType {
    pub fn is_outbound_authentication_type(&self) -> bool {
        matches!(self, Self::OutboundAuthenticationType(_))
    }

    pub fn is_inbound_authentication_type(&self) -> bool {
        matches!(self, Self::InboundAuthenticationType(_))
    }

    pub fn as_outbound_authentication_type(&self) -> Option<&OutboundAuthenticationType> {
        match self {
            Self::OutboundAuthenticationType(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_outbound_authentication_type(self) -> Option<OutboundAuthenticationType> {
        match self {
            Self::OutboundAuthenticationType(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_inbound_authentication_type(&self) -> Option<&InboundAuthenticationType> {
        match self {
            Self::InboundAuthenticationType(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_inbound_authentication_type(self) -> Option<InboundAuthenticationType> {
        match self {
            Self::InboundAuthenticationType(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for AuthenticationMethodCreateOrUpdateRequestMethodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutboundAuthenticationType(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::InboundAuthenticationType(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
