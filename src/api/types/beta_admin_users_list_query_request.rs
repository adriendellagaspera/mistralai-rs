pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BetaAdminUsersListQueryRequest {
    /// Page number to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Maximum number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Email address to filter users and invitations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Also return deactivated and blocked members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_inactive: Option<bool>,
}

impl BetaAdminUsersListQueryRequest {
    pub fn builder() -> BetaAdminUsersListQueryRequestBuilder {
        <BetaAdminUsersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BetaAdminUsersListQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    email: Option<String>,
    include_inactive: Option<bool>,
}

impl BetaAdminUsersListQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn include_inactive(mut self, value: bool) -> Self {
        self.include_inactive = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BetaAdminUsersListQueryRequest`].
    pub fn build(self) -> Result<BetaAdminUsersListQueryRequest, BuildError> {
        Ok(BetaAdminUsersListQueryRequest {
            page: self.page,
            page_size: self.page_size,
            email: self.email,
            include_inactive: self.include_inactive,
        })
    }
}
