pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdminUserOut {
    /// Date the member was added to the Organization.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Email address of the Organization member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// User first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Whether the member is outside the SSO domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sso_outsider: Option<bool>,
    /// Last time the member signed in to the Organization, accurate to about 30 minutes. Null if they never signed in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login_at: Option<DateTime<FixedOffset>>,
    /// User last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Name of the Organization member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identity provider ID for the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid_id: Option<String>,
    /// Deprecated single organization role. Use 'raw_roles' instead.
    pub raw_role: StaticOrganizationRoles,
    /// Organization roles assigned to the member.
    #[serde(default)]
    pub raw_roles: Vec<StaticOrganizationRoles>,
    /// Account status of the member. Deactivated and blocked members cannot access the Organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MemberStatus>,
    /// Product seats assigned to the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_types: Option<Vec<AdminUserOutSubscriptionTypesItem>>,
    /// Subscriptions assigned to the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<MemberSubscriptionOut>>,
    /// Organization member ID.
    #[serde(default)]
    pub uuid: String,
    /// Deprecated: limited to the first 100 Workspaces the member belongs to, ordered by name. Use GET /users/{user_id}/workspaces instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<Vec<MemberWorkspaceInfo>>,
    /// Total number of Workspaces the member belongs to, including those not returned in 'workspaces'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces_total: Option<i64>,
}

impl AdminUserOut {
    pub fn builder() -> AdminUserOutBuilder {
        <AdminUserOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminUserOutBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    email: Option<String>,
    first_name: Option<String>,
    is_sso_outsider: Option<bool>,
    last_login_at: Option<DateTime<FixedOffset>>,
    last_name: Option<String>,
    name: Option<String>,
    oid_id: Option<String>,
    raw_role: Option<StaticOrganizationRoles>,
    raw_roles: Option<Vec<StaticOrganizationRoles>>,
    status: Option<MemberStatus>,
    subscription_types: Option<Vec<AdminUserOutSubscriptionTypesItem>>,
    subscriptions: Option<Vec<MemberSubscriptionOut>>,
    uuid: Option<String>,
    workspaces: Option<Vec<MemberWorkspaceInfo>>,
    workspaces_total: Option<i64>,
}

impl AdminUserOutBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn is_sso_outsider(mut self, value: bool) -> Self {
        self.is_sso_outsider = Some(value);
        self
    }

    pub fn last_login_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_login_at = Some(value);
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
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

    pub fn raw_role(mut self, value: StaticOrganizationRoles) -> Self {
        self.raw_role = Some(value);
        self
    }

    pub fn raw_roles(mut self, value: Vec<StaticOrganizationRoles>) -> Self {
        self.raw_roles = Some(value);
        self
    }

    pub fn status(mut self, value: MemberStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subscription_types(mut self, value: Vec<AdminUserOutSubscriptionTypesItem>) -> Self {
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

    pub fn workspaces_total(mut self, value: i64) -> Self {
        self.workspaces_total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdminUserOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](AdminUserOutBuilder::created_at)
    /// - [`raw_role`](AdminUserOutBuilder::raw_role)
    /// - [`raw_roles`](AdminUserOutBuilder::raw_roles)
    /// - [`uuid`](AdminUserOutBuilder::uuid)
    pub fn build(self) -> Result<AdminUserOut, BuildError> {
        Ok(AdminUserOut {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            email: self.email,
            first_name: self.first_name,
            is_sso_outsider: self.is_sso_outsider,
            last_login_at: self.last_login_at,
            last_name: self.last_name,
            name: self.name,
            oid_id: self.oid_id,
            raw_role: self
                .raw_role
                .ok_or_else(|| BuildError::missing_field("raw_role"))?,
            raw_roles: self
                .raw_roles
                .ok_or_else(|| BuildError::missing_field("raw_roles"))?,
            status: self.status,
            subscription_types: self.subscription_types,
            subscriptions: self.subscriptions,
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
            workspaces: self.workspaces,
            workspaces_total: self.workspaces_total,
        })
    }
}
