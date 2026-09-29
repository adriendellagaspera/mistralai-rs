pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdminScimSyncRunOut {
    /// Unique identifier of the SCIM synchronization run.
    #[serde(default)]
    pub run_id: String,
    /// Current lifecycle status of the synchronization run.
    pub status: ScimSyncRunStatus,
    /// Whether the run only previewed changes.
    #[serde(default)]
    pub dry_run: bool,
    /// Categories selected for this synchronization run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_config: Option<AdminScimSyncConfig>,
    /// Time at which the synchronization run was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Time at which processing started, if it has started.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub started_at: Option<DateTime<FixedOffset>>,
    /// Time at which processing finished, if it has finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub finished_at: Option<DateTime<FixedOffset>>,
    /// Preview or result summary, available after the synchronization plan is built.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ScimSyncSummaryOut>,
    /// Failure detail when the synchronization run could not complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl AdminScimSyncRunOut {
    pub fn builder() -> AdminScimSyncRunOutBuilder {
        <AdminScimSyncRunOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminScimSyncRunOutBuilder {
    run_id: Option<String>,
    status: Option<ScimSyncRunStatus>,
    dry_run: Option<bool>,
    sync_config: Option<AdminScimSyncConfig>,
    created_at: Option<DateTime<FixedOffset>>,
    started_at: Option<DateTime<FixedOffset>>,
    finished_at: Option<DateTime<FixedOffset>>,
    summary: Option<ScimSyncSummaryOut>,
    error_message: Option<String>,
}

impl AdminScimSyncRunOutBuilder {
    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ScimSyncRunStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
        self
    }

    pub fn sync_config(mut self, value: AdminScimSyncConfig) -> Self {
        self.sync_config = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn finished_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.finished_at = Some(value);
        self
    }

    pub fn summary(mut self, value: ScimSyncSummaryOut) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdminScimSyncRunOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`run_id`](AdminScimSyncRunOutBuilder::run_id)
    /// - [`status`](AdminScimSyncRunOutBuilder::status)
    /// - [`dry_run`](AdminScimSyncRunOutBuilder::dry_run)
    /// - [`created_at`](AdminScimSyncRunOutBuilder::created_at)
    pub fn build(self) -> Result<AdminScimSyncRunOut, BuildError> {
        Ok(AdminScimSyncRunOut {
            run_id: self
                .run_id
                .ok_or_else(|| BuildError::missing_field("run_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            dry_run: self
                .dry_run
                .ok_or_else(|| BuildError::missing_field("dry_run"))?,
            sync_config: self.sync_config,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            started_at: self.started_at,
            finished_at: self.finished_at,
            summary: self.summary,
            error_message: self.error_message,
        })
    }
}
