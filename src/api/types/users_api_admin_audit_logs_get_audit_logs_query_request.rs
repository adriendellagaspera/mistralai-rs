pub use crate::prelude::*;

/// Query parameters for users_api_admin_audit_logs_get_audit_logs
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsersApiAdminAuditLogsGetAuditLogsQueryRequest {
    /// Actor types to include in the audit log results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_type: Option<Vec<ActorType>>,
    /// Event types to include in the audit log results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<Vec<AuditLogEventType>>,
    /// Target resource types to include in the audit log results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<Vec<TargetType>>,
    /// Filter logs by the UUID of the user who performed the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_user_uuid: Option<String>,
    /// Sort order for audit log entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<UsersApiAdminAuditLogsGetAuditLogsAuditLogsRequestSort>,
    /// Return audit log entries after this time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<DateTime<FixedOffset>>,
    /// Return audit log entries before this time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<DateTime<FixedOffset>>,
    /// Maximum number of results to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl UsersApiAdminAuditLogsGetAuditLogsQueryRequest {
    pub fn builder() -> UsersApiAdminAuditLogsGetAuditLogsQueryRequestBuilder {
        <UsersApiAdminAuditLogsGetAuditLogsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsersApiAdminAuditLogsGetAuditLogsQueryRequestBuilder {
    actor_type: Option<Vec<ActorType>>,
    event_type: Option<Vec<AuditLogEventType>>,
    target_type: Option<Vec<TargetType>>,
    actor_user_uuid: Option<String>,
    sort: Option<UsersApiAdminAuditLogsGetAuditLogsAuditLogsRequestSort>,
    after: Option<DateTime<FixedOffset>>,
    before: Option<DateTime<FixedOffset>>,
    limit: Option<i64>,
}

impl UsersApiAdminAuditLogsGetAuditLogsQueryRequestBuilder {
    pub fn actor_type(mut self, value: Vec<ActorType>) -> Self {
        self.actor_type = Some(value);
        self
    }

    pub fn event_type(mut self, value: Vec<AuditLogEventType>) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn target_type(mut self, value: Vec<TargetType>) -> Self {
        self.target_type = Some(value);
        self
    }

    pub fn actor_user_uuid(mut self, value: impl Into<String>) -> Self {
        self.actor_user_uuid = Some(value.into());
        self
    }

    pub fn sort(mut self, value: UsersApiAdminAuditLogsGetAuditLogsAuditLogsRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.after = Some(value);
        self
    }

    pub fn before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.before = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsersApiAdminAuditLogsGetAuditLogsQueryRequest`].
    pub fn build(self) -> Result<UsersApiAdminAuditLogsGetAuditLogsQueryRequest, BuildError> {
        Ok(UsersApiAdminAuditLogsGetAuditLogsQueryRequest {
            actor_type: self.actor_type,
            event_type: self.event_type,
            target_type: self.target_type,
            actor_user_uuid: self.actor_user_uuid,
            sort: self.sort,
            after: self.after,
            before: self.before,
            limit: self.limit,
        })
    }
}
