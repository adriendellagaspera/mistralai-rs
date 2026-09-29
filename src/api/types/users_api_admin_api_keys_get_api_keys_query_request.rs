pub use crate::prelude::*;

/// Query parameters for users_api_admin_api_keys_get_api_keys
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsersApiAdminApiKeysGetApiKeysQueryRequest {
    /// Maximum number of results to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of results to skip before returning results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Filter API keys by name substring or the last 4 characters of the key. Matching is case-insensitive and accent-insensitive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UsersApiAdminApiKeysGetApiKeysQueryRequest {
    pub fn builder() -> UsersApiAdminApiKeysGetApiKeysQueryRequestBuilder {
        <UsersApiAdminApiKeysGetApiKeysQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsersApiAdminApiKeysGetApiKeysQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    name: Option<String>,
}

impl UsersApiAdminApiKeysGetApiKeysQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsersApiAdminApiKeysGetApiKeysQueryRequest`].
    pub fn build(self) -> Result<UsersApiAdminApiKeysGetApiKeysQueryRequest, BuildError> {
        Ok(UsersApiAdminApiKeysGetApiKeysQueryRequest {
            limit: self.limit,
            offset: self.offset,
            name: self.name,
        })
    }
}
