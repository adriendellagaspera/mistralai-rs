pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MemberWorkspaceInfo {
    /// Whether this is the default Workspace for the Organization.
    #[serde(default)]
    pub is_default: bool,
    /// Workspace name.
    #[serde(default)]
    pub name: String,
    /// Deprecated single workspace role. Use 'raw_roles' instead.
    pub raw_role: MemberWorkspaceInfoRawRole,
    /// Workspace roles assigned to the member.
    pub raw_roles: MemberWorkspaceInfoRawRoles,
    /// Workspace ID.
    #[serde(default)]
    pub uuid: String,
}

impl MemberWorkspaceInfo {
    pub fn builder() -> MemberWorkspaceInfoBuilder {
        <MemberWorkspaceInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemberWorkspaceInfoBuilder {
    is_default: Option<bool>,
    name: Option<String>,
    raw_role: Option<MemberWorkspaceInfoRawRole>,
    raw_roles: Option<MemberWorkspaceInfoRawRoles>,
    uuid: Option<String>,
}

impl MemberWorkspaceInfoBuilder {
    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn raw_role(mut self, value: MemberWorkspaceInfoRawRole) -> Self {
        self.raw_role = Some(value);
        self
    }

    pub fn raw_roles(mut self, value: MemberWorkspaceInfoRawRoles) -> Self {
        self.raw_roles = Some(value);
        self
    }

    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MemberWorkspaceInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_default`](MemberWorkspaceInfoBuilder::is_default)
    /// - [`name`](MemberWorkspaceInfoBuilder::name)
    /// - [`raw_role`](MemberWorkspaceInfoBuilder::raw_role)
    /// - [`raw_roles`](MemberWorkspaceInfoBuilder::raw_roles)
    /// - [`uuid`](MemberWorkspaceInfoBuilder::uuid)
    pub fn build(self) -> Result<MemberWorkspaceInfo, BuildError> {
        Ok(MemberWorkspaceInfo {
            is_default: self
                .is_default
                .ok_or_else(|| BuildError::missing_field("is_default"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            raw_role: self
                .raw_role
                .ok_or_else(|| BuildError::missing_field("raw_role"))?,
            raw_roles: self
                .raw_roles
                .ok_or_else(|| BuildError::missing_field("raw_roles"))?,
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
        })
    }
}
