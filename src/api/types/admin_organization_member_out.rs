pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdminOrganizationMemberOut {
    /// Date the member was added to the Organization.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Email address of the Organization member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Whether the member is outside the SSO domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sso_outsider: Option<bool>,
    /// Name of the Organization member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identity provider ID for the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid_id: Option<String>,
    /// Deprecated single organization role. Use 'raw_roles' instead.
    pub raw_role: AdminOrganizationMemberOutRawRole,
    /// Organization roles assigned to the member.
    pub raw_roles: AdminOrganizationMemberOutRawRoles,
    /// Product seats assigned to the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_types: Option<Vec<AdminOrganizationMemberOutSubscriptionTypesItem>>,
    /// Subscriptions assigned to the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<MemberSubscriptionOut>>,
    /// Organization member ID.
    #[serde(default)]
    pub uuid: String,
    /// Workspaces the member belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<Vec<MemberWorkspaceInfo>>,
}

impl AdminOrganizationMemberOut {
    pub fn builder() -> AdminOrganizationMemberOutBuilder {
        <AdminOrganizationMemberOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminOrganizationMemberOutBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    email: Option<String>,
    is_sso_outsider: Option<bool>,
    name: Option<String>,
    oid_id: Option<String>,
    raw_role: Option<AdminOrganizationMemberOutRawRole>,
    raw_roles: Option<AdminOrganizationMemberOutRawRoles>,
    subscription_types: Option<Vec<AdminOrganizationMemberOutSubscriptionTypesItem>>,
    subscriptions: Option<Vec<MemberSubscriptionOut>>,
    uuid: Option<String>,
    workspaces: Option<Vec<MemberWorkspaceInfo>>,
}

impl AdminOrganizationMemberOutBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn is_sso_outsider(mut self, value: bool) -> Self {
        self.is_sso_outsider = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn oid_id(mut self, value: impl Into<String>) -> Self {
        self.oid_id = Some(value.into());
        self
    }

    pub fn raw_role(mut self, value: AdminOrganizationMemberOutRawRole) -> Self {
        self.raw_role = Some(value);
        self
    }

    pub fn raw_roles(mut self, value: AdminOrganizationMemberOutRawRoles) -> Self {
        self.raw_roles = Some(value);
        self
    }

    pub fn subscription_types(
        mut self,
        value: Vec<AdminOrganizationMemberOutSubscriptionTypesItem>,
    ) -> Self {
        self.subscription_types = Some(value);
        self
    }

    pub fn subscriptions(mut self, value: Vec<MemberSubscriptionOut>) -> Self {
        self.subscriptions = Some(value);
        self
    }

    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    pub fn workspaces(mut self, value: Vec<MemberWorkspaceInfo>) -> Self {
        self.workspaces = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdminOrganizationMemberOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](AdminOrganizationMemberOutBuilder::created_at)
    /// - [`raw_role`](AdminOrganizationMemberOutBuilder::raw_role)
    /// - [`raw_roles`](AdminOrganizationMemberOutBuilder::raw_roles)
    /// - [`uuid`](AdminOrganizationMemberOutBuilder::uuid)
    pub fn build(self) -> Result<AdminOrganizationMemberOut, BuildError> {
        Ok(AdminOrganizationMemberOut {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            email: self.email,
            is_sso_outsider: self.is_sso_outsider,
            name: self.name,
            oid_id: self.oid_id,
            raw_role: self
                .raw_role
                .ok_or_else(|| BuildError::missing_field("raw_role"))?,
            raw_roles: self
                .raw_roles
                .ok_or_else(|| BuildError::missing_field("raw_roles"))?,
            subscription_types: self.subscription_types,
            subscriptions: self.subscriptions,
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
            workspaces: self.workspaces,
        })
    }
}
