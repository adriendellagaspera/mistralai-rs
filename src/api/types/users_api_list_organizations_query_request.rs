pub use crate::prelude::*;

/// Query parameters for users_api_list_organizations
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsersApiListOrganizationsQueryRequest {
    /// Number of organizations to skip before returning results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Maximum number of organizations to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl UsersApiListOrganizationsQueryRequest {
    pub fn builder() -> UsersApiListOrganizationsQueryRequestBuilder {
        <UsersApiListOrganizationsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsersApiListOrganizationsQueryRequestBuilder {
    offset: Option<i64>,
    limit: Option<i64>,
}

impl UsersApiListOrganizationsQueryRequestBuilder {
    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsersApiListOrganizationsQueryRequest`].
    pub fn build(self) -> Result<UsersApiListOrganizationsQueryRequest, BuildError> {
        Ok(UsersApiListOrganizationsQueryRequest {
            offset: self.offset,
            limit: self.limit,
        })
    }
}
