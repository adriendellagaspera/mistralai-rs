pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "principal_type")]
#[non_exhaustive]
pub enum WhoamiResponsePrincipal {
    #[serde(rename = "service_account")]
    #[non_exhaustive]
    ServiceAccount {
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(default)]
        service_account_id: String,
    },

    #[serde(rename = "user")]
    #[non_exhaustive]
    User {
        #[serde(skip_serializing_if = "Option::is_none")]
        email: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        first_name: Option<String>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        last_name: Option<String>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl WhoamiResponsePrincipal {
    pub fn service_account(service_account_id: String) -> Self {
        Self::ServiceAccount {
            name: None,
            service_account_id,
        }
    }

    pub fn user(id: String) -> Self {
        Self::User {
            email: None,
            first_name: None,
            id,
            last_name: None,
        }
    }

    pub fn service_account_with_name(name: String, service_account_id: String) -> Self {
        Self::ServiceAccount {
            name: Some(name),
            service_account_id,
        }
    }

    pub fn user_with_email(
        email: String,
        first_name: Option<String>,
        id: String,
        last_name: Option<String>,
    ) -> Self {
        Self::User {
            email: Some(email),
            first_name,
            id,
            last_name,
        }
    }

    pub fn user_with_first_name(
        email: Option<String>,
        first_name: String,
        id: String,
        last_name: Option<String>,
    ) -> Self {
        Self::User {
            email,
            first_name: Some(first_name),
            id,
            last_name,
        }
    }

    pub fn user_with_last_name(
        email: Option<String>,
        first_name: Option<String>,
        id: String,
        last_name: String,
    ) -> Self {
        Self::User {
            email,
            first_name,
            id,
            last_name: Some(last_name),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
