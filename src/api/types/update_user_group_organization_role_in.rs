pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateUserGroupOrganizationRoleIn {
    /// Organization role to assign to the group.
    pub organization_role: UpdateUserGroupOrganizationRoleInOrganizationRole,
}

impl UpdateUserGroupOrganizationRoleIn {
    pub fn builder() -> UpdateUserGroupOrganizationRoleInBuilder {
        <UpdateUserGroupOrganizationRoleInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateUserGroupOrganizationRoleInBuilder {
    organization_role: Option<UpdateUserGroupOrganizationRoleInOrganizationRole>,
}

impl UpdateUserGroupOrganizationRoleInBuilder {
    pub fn organization_role(
        mut self,
        value: UpdateUserGroupOrganizationRoleInOrganizationRole,
    ) -> Self {
        self.organization_role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateUserGroupOrganizationRoleIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_role`](UpdateUserGroupOrganizationRoleInBuilder::organization_role)
    pub fn build(self) -> Result<UpdateUserGroupOrganizationRoleIn, BuildError> {
        Ok(UpdateUserGroupOrganizationRoleIn {
            organization_role: self
                .organization_role
                .ok_or_else(|| BuildError::missing_field("organization_role"))?,
        })
    }
}
