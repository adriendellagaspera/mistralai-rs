pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdminUserOut {
    /// Organization member ID.
    #[serde(default)]
    pub uuid: String,
    /// Identity provider ID for the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid_id: Option<String>,
    /// Name of the Organization member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Email address of the Organization member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Whether the member is outside the SSO domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sso_outsider: Option<bool>,
    /// Workspaces the member belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<Vec<MemberWorkspaceInfo>>,
    /// Subscriptions assigned to the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<MemberSubscriptionOut>>,
    /// Organization roles assigned to the member.
    pub raw_roles: AdminUserOutRawRoles,
    /// Deprecated single organization role. Use 'raw_roles' instead.
    pub raw_role: AdminUserOutRawRole,
    /// Product seats assigned to the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_types: Option<Vec<AdminUserOutSubscriptionTypesItem>>,
    /// Date the member was added to the Organization.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// User first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// User last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

impl AdminUserOut {
    pub fn builder() -> AdminUserOutBuilder {
        <AdminUserOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminUserOutBuilder {
    uuid: Option<String>,
    oid_id: Option<String>,
    name: Option<String>,
    email: Option<String>,
    is_sso_outsider: Option<bool>,
    workspaces: Option<Vec<MemberWorkspaceInfo>>,
    subscriptions: Option<Vec<MemberSubscriptionOut>>,
    raw_roles: Option<AdminUserOutRawRoles>,
    raw_role: Option<AdminUserOutRawRole>,
    subscription_types: Option<Vec<AdminUserOutSubscriptionTypesItem>>,
    created_at: Option<DateTime<FixedOffset>>,
    first_name: Option<String>,
    last_name: Option<String>,
}

impl AdminUserOutBuilder {
    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    pub fn oid_id(mut self, value: impl Into<String>) -> Self {
        self.oid_id = Some(value.into());
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

    pub fn is_sso_outsider(mut self, value: bool) -> Self {
        self.is_sso_outsider = Some(value);
        self
    }

    pub fn workspaces(mut self, value: Vec<MemberWorkspaceInfo>) -> Self {
        self.workspaces = Some(value);
        self
    }

    pub fn subscriptions(mut self, value: Vec<MemberSubscriptionOut>) -> Self {
        self.subscriptions = Some(value);
        self
    }

    pub fn raw_roles(mut self, value: AdminUserOutRawRoles) -> Self {
        self.raw_roles = Some(value);
        self
    }

    pub fn raw_role(mut self, value: AdminUserOutRawRole) -> Self {
        self.raw_role = Some(value);
        self
    }

    pub fn subscription_types(mut self, value: Vec<AdminUserOutSubscriptionTypesItem>) -> Self {
        self.subscription_types = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdminUserOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`uuid`](AdminUserOutBuilder::uuid)
    /// - [`raw_roles`](AdminUserOutBuilder::raw_roles)
    /// - [`raw_role`](AdminUserOutBuilder::raw_role)
    /// - [`created_at`](AdminUserOutBuilder::created_at)
    pub fn build(self) -> Result<AdminUserOut, BuildError> {
        Ok(AdminUserOut {
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
            oid_id: self.oid_id,
            name: self.name,
            email: self.email,
            is_sso_outsider: self.is_sso_outsider,
            workspaces: self.workspaces,
            subscriptions: self.subscriptions,
            raw_roles: self
                .raw_roles
                .ok_or_else(|| BuildError::missing_field("raw_roles"))?,
            raw_role: self
                .raw_role
                .ok_or_else(|| BuildError::missing_field("raw_role"))?,
            subscription_types: self.subscription_types,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            first_name: self.first_name,
            last_name: self.last_name,
        })
    }
}
