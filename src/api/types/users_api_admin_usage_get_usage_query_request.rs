pub use crate::prelude::*;

/// Query parameters for users_api_admin_usage_get_usage
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsersApiAdminUsageGetUsageQueryRequest {
    /// Month to return usage for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<String>,
    /// Year to return usage for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<String>,
    /// Workspace ID to filter results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// Regional inference zone to filter results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_zone: Option<ApiZone>,
}

impl UsersApiAdminUsageGetUsageQueryRequest {
    pub fn builder() -> UsersApiAdminUsageGetUsageQueryRequestBuilder {
        <UsersApiAdminUsageGetUsageQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsersApiAdminUsageGetUsageQueryRequestBuilder {
    month: Option<String>,
    year: Option<String>,
    workspace_id: Option<String>,
    api_zone: Option<ApiZone>,
}

impl UsersApiAdminUsageGetUsageQueryRequestBuilder {
    pub fn month(mut self, value: impl Into<String>) -> Self {
        self.month = Some(value.into());
        self
    }

    pub fn year(mut self, value: impl Into<String>) -> Self {
        self.year = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn api_zone(mut self, value: ApiZone) -> Self {
        self.api_zone = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsersApiAdminUsageGetUsageQueryRequest`].
    pub fn build(self) -> Result<UsersApiAdminUsageGetUsageQueryRequest, BuildError> {
        Ok(UsersApiAdminUsageGetUsageQueryRequest {
            month: self.month,
            year: self.year,
            workspace_id: self.workspace_id,
            api_zone: self.api_zone,
        })
    }
}
