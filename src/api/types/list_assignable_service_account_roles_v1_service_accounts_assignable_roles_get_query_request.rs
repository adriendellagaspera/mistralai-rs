pub use crate::prelude::*;

/// Query parameters for list_assignable_service_account_roles_v1_service_accounts_assignable_roles_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAssignableServiceAccountRolesV1ServiceAccountsAssignableRolesGetQueryRequest {
    #[serde(default)]
    pub workspace_id: String,
}

impl ListAssignableServiceAccountRolesV1ServiceAccountsAssignableRolesGetQueryRequest {
    pub fn builder(
    ) -> ListAssignableServiceAccountRolesV1ServiceAccountsAssignableRolesGetQueryRequestBuilder
    {
        <ListAssignableServiceAccountRolesV1ServiceAccountsAssignableRolesGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAssignableServiceAccountRolesV1ServiceAccountsAssignableRolesGetQueryRequestBuilder {
    workspace_id: Option<String>,
}

impl ListAssignableServiceAccountRolesV1ServiceAccountsAssignableRolesGetQueryRequestBuilder {
    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListAssignableServiceAccountRolesV1ServiceAccountsAssignableRolesGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workspace_id`](ListAssignableServiceAccountRolesV1ServiceAccountsAssignableRolesGetQueryRequestBuilder::workspace_id)
    pub fn build(
        self,
    ) -> Result<
        ListAssignableServiceAccountRolesV1ServiceAccountsAssignableRolesGetQueryRequest,
        BuildError,
    > {
        Ok(
            ListAssignableServiceAccountRolesV1ServiceAccountsAssignableRolesGetQueryRequest {
                workspace_id: self
                    .workspace_id
                    .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
            },
        )
    }
}
