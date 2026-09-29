pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditLogOut {
    /// Time when the audit log entry was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Type of actor that performed the action.
    pub actor_type: ActorType,
    /// Details about who performed the action.
    #[serde(default)]
    pub actor_metadata: HashMap<String, String>,
    /// Type of action recorded in the audit log.
    pub event_type: AuditLogEventType,
    /// Details about the recorded action.
    #[serde(default)]
    pub event_metadata: HashMap<String, String>,
    /// Type of resource affected by the action.
    pub target_type: TargetType,
    /// Details about the affected resource.
    #[serde(default)]
    pub target_metadata: HashMap<String, String>,
    /// Audit log entry ID.
    #[serde(default)]
    pub log_id: i64,
    /// Organization ID for the audit log entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_uuid: Option<String>,
    /// Workspace ID for the audit log entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_uuid: Option<String>,
}

impl AuditLogOut {
    pub fn builder() -> AuditLogOutBuilder {
        <AuditLogOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuditLogOutBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    actor_type: Option<ActorType>,
    actor_metadata: Option<HashMap<String, String>>,
    event_type: Option<AuditLogEventType>,
    event_metadata: Option<HashMap<String, String>>,
    target_type: Option<TargetType>,
    target_metadata: Option<HashMap<String, String>>,
    log_id: Option<i64>,
    organization_uuid: Option<String>,
    workspace_uuid: Option<String>,
}

impl AuditLogOutBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn actor_type(mut self, value: ActorType) -> Self {
        self.actor_type = Some(value);
        self
    }

    pub fn actor_metadata(mut self, value: HashMap<String, String>) -> Self {
        self.actor_metadata = Some(value);
        self
    }

    pub fn event_type(mut self, value: AuditLogEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn event_metadata(mut self, value: HashMap<String, String>) -> Self {
        self.event_metadata = Some(value);
        self
    }

    pub fn target_type(mut self, value: TargetType) -> Self {
        self.target_type = Some(value);
        self
    }

    pub fn target_metadata(mut self, value: HashMap<String, String>) -> Self {
        self.target_metadata = Some(value);
        self
    }

    pub fn log_id(mut self, value: i64) -> Self {
        self.log_id = Some(value);
        self
    }

    pub fn organization_uuid(mut self, value: impl Into<String>) -> Self {
        self.organization_uuid = Some(value.into());
        self
    }

    pub fn workspace_uuid(mut self, value: impl Into<String>) -> Self {
        self.workspace_uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AuditLogOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](AuditLogOutBuilder::created_at)
    /// - [`actor_type`](AuditLogOutBuilder::actor_type)
    /// - [`actor_metadata`](AuditLogOutBuilder::actor_metadata)
    /// - [`event_type`](AuditLogOutBuilder::event_type)
    /// - [`event_metadata`](AuditLogOutBuilder::event_metadata)
    /// - [`target_type`](AuditLogOutBuilder::target_type)
    /// - [`target_metadata`](AuditLogOutBuilder::target_metadata)
    /// - [`log_id`](AuditLogOutBuilder::log_id)
    pub fn build(self) -> Result<AuditLogOut, BuildError> {
        Ok(AuditLogOut {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            actor_type: self
                .actor_type
                .ok_or_else(|| BuildError::missing_field("actor_type"))?,
            actor_metadata: self
                .actor_metadata
                .ok_or_else(|| BuildError::missing_field("actor_metadata"))?,
            event_type: self
                .event_type
                .ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_metadata: self
                .event_metadata
                .ok_or_else(|| BuildError::missing_field("event_metadata"))?,
            target_type: self
                .target_type
                .ok_or_else(|| BuildError::missing_field("target_type"))?,
            target_metadata: self
                .target_metadata
                .ok_or_else(|| BuildError::missing_field("target_metadata"))?,
            log_id: self
                .log_id
                .ok_or_else(|| BuildError::missing_field("log_id"))?,
            organization_uuid: self.organization_uuid,
            workspace_uuid: self.workspace_uuid,
        })
    }
}
