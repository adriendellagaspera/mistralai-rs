pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MemberWorkspaceInfo {
    /// Workspace ID.
    #[serde(default)]
    pub uuid: String,
    /// Workspace name.
    #[serde(default)]
    pub name: String,
    /// Whether this is the default Workspace for the Organization.
    #[serde(default)]
    pub is_default: bool,
    /// Workspace roles assigned to the member.
    pub raw_roles: MemberWorkspaceInfoRawRoles,
    /// Deprecated single workspace role. Use 'raw_roles' instead.
    pub raw_role: MemberWorkspaceInfoRawRole,
}

impl MemberWorkspaceInfo {
    pub fn builder() -> MemberWorkspaceInfoBuilder {
        <MemberWorkspaceInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemberWorkspaceInfoBuilder {
    uuid: Option<String>,
    name: Option<String>,
    is_default: Option<bool>,
    raw_roles: Option<MemberWorkspaceInfoRawRoles>,
    raw_role: Option<MemberWorkspaceInfoRawRole>,
}

impl MemberWorkspaceInfoBuilder {
    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn raw_roles(mut self, value: MemberWorkspaceInfoRawRoles) -> Self {
        self.raw_roles = Some(value);
        self
    }

    pub fn raw_role(mut self, value: MemberWorkspaceInfoRawRole) -> Self {
        self.raw_role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MemberWorkspaceInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`uuid`](MemberWorkspaceInfoBuilder::uuid)
    /// - [`name`](MemberWorkspaceInfoBuilder::name)
    /// - [`is_default`](MemberWorkspaceInfoBuilder::is_default)
    /// - [`raw_roles`](MemberWorkspaceInfoBuilder::raw_roles)
    /// - [`raw_role`](MemberWorkspaceInfoBuilder::raw_role)
    pub fn build(self) -> Result<MemberWorkspaceInfo, BuildError> {
        Ok(MemberWorkspaceInfo {
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            is_default: self
                .is_default
                .ok_or_else(|| BuildError::missing_field("is_default"))?,
            raw_roles: self
                .raw_roles
                .ok_or_else(|| BuildError::missing_field("raw_roles"))?,
            raw_role: self
                .raw_role
                .ok_or_else(|| BuildError::missing_field("raw_role"))?,
        })
    }
}
