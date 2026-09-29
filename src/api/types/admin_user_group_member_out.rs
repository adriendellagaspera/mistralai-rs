pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminUserGroupMemberOut {
    /// User ID of the group member.
    #[serde(default)]
    pub user_uuid: String,
    /// Name of the group member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Email address of the group member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl AdminUserGroupMemberOut {
    pub fn builder() -> AdminUserGroupMemberOutBuilder {
        <AdminUserGroupMemberOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminUserGroupMemberOutBuilder {
    user_uuid: Option<String>,
    name: Option<String>,
    email: Option<String>,
}

impl AdminUserGroupMemberOutBuilder {
    pub fn user_uuid(mut self, value: impl Into<String>) -> Self {
        self.user_uuid = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdminUserGroupMemberOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_uuid`](AdminUserGroupMemberOutBuilder::user_uuid)
    pub fn build(self) -> Result<AdminUserGroupMemberOut, BuildError> {
        Ok(AdminUserGroupMemberOut {
            user_uuid: self
                .user_uuid
                .ok_or_else(|| BuildError::missing_field("user_uuid"))?,
            name: self.name,
            email: self.email,
        })
    }
}
