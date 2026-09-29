pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminScimSyncConfig {
    /// Delete Organization groups that are absent from the SCIM provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_missing_groups: Option<bool>,
    /// Remove Organization users that are inactive or absent from the SCIM provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprovision_users: Option<bool>,
    /// Synchronize group metadata from the SCIM provider.
    #[serde(default)]
    pub sync_groups: bool,
    /// Synchronize additions and removals of users in SCIM groups.
    #[serde(default)]
    pub sync_memberships: bool,
    /// Add users found in the SCIM provider to the Organization.
    #[serde(default)]
    pub sync_users: bool,
}

impl AdminScimSyncConfig {
    pub fn builder() -> AdminScimSyncConfigBuilder {
        <AdminScimSyncConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminScimSyncConfigBuilder {
    delete_missing_groups: Option<bool>,
    deprovision_users: Option<bool>,
    sync_groups: Option<bool>,
    sync_memberships: Option<bool>,
    sync_users: Option<bool>,
}

impl AdminScimSyncConfigBuilder {
    pub fn delete_missing_groups(mut self, value: bool) -> Self {
        self.delete_missing_groups = Some(value);
        self
    }

    pub fn deprovision_users(mut self, value: bool) -> Self {
        self.deprovision_users = Some(value);
        self
    }

    pub fn sync_groups(mut self, value: bool) -> Self {
        self.sync_groups = Some(value);
        self
    }

    pub fn sync_memberships(mut self, value: bool) -> Self {
        self.sync_memberships = Some(value);
        self
    }

    pub fn sync_users(mut self, value: bool) -> Self {
        self.sync_users = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdminScimSyncConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sync_groups`](AdminScimSyncConfigBuilder::sync_groups)
    /// - [`sync_memberships`](AdminScimSyncConfigBuilder::sync_memberships)
    /// - [`sync_users`](AdminScimSyncConfigBuilder::sync_users)
    pub fn build(self) -> Result<AdminScimSyncConfig, BuildError> {
        Ok(AdminScimSyncConfig {
            delete_missing_groups: self.delete_missing_groups,
            deprovision_users: self.deprovision_users,
            sync_groups: self
                .sync_groups
                .ok_or_else(|| BuildError::missing_field("sync_groups"))?,
            sync_memberships: self
                .sync_memberships
                .ok_or_else(|| BuildError::missing_field("sync_memberships"))?,
            sync_users: self
                .sync_users
                .ok_or_else(|| BuildError::missing_field("sync_users"))?,
        })
    }
}
