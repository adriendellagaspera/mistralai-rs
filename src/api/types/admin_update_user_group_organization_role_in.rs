pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdminUpdateUserGroupOrganizationRoleIn {
    /// Organization role to assign to the group: a role name (`member`, `billing_manager`, `organization_admin`) or a role UUID.
    pub organization_role: AdminUpdateUserGroupOrganizationRoleInOrganizationRole,
}

impl AdminUpdateUserGroupOrganizationRoleIn {
    pub fn builder() -> AdminUpdateUserGroupOrganizationRoleInBuilder {
        <AdminUpdateUserGroupOrganizationRoleInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminUpdateUserGroupOrganizationRoleInBuilder {
    organization_role: Option<AdminUpdateUserGroupOrganizationRoleInOrganizationRole>,
}

impl AdminUpdateUserGroupOrganizationRoleInBuilder {
    pub fn organization_role(
        mut self,
        value: AdminUpdateUserGroupOrganizationRoleInOrganizationRole,
    ) -> Self {
        self.organization_role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdminUpdateUserGroupOrganizationRoleIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_role`](AdminUpdateUserGroupOrganizationRoleInBuilder::organization_role)
    pub fn build(self) -> Result<AdminUpdateUserGroupOrganizationRoleIn, BuildError> {
        Ok(AdminUpdateUserGroupOrganizationRoleIn {
            organization_role: self
                .organization_role
                .ok_or_else(|| BuildError::missing_field("organization_role"))?,
        })
    }
}
