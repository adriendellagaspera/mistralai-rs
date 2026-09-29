pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdminScimSyncRunOut {
    /// Time at which the synchronization run was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Whether the run only previewed changes.
    #[serde(default)]
    pub dry_run: bool,
    /// Failure detail when the synchronization run could not complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Time at which processing finished, if it has finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub finished_at: Option<DateTime<FixedOffset>>,
    /// Unique identifier of the SCIM synchronization run.
    #[serde(default)]
    pub run_id: String,
    /// Time at which processing started, if it has started.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub started_at: Option<DateTime<FixedOffset>>,
    /// Current lifecycle status of the synchronization run.
    pub status: ScimSyncRunStatus,
    /// Preview or result summary, available after the synchronization plan is built.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ScimSyncSummaryOut>,
    /// Categories selected for this synchronization run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_config: Option<AdminScimSyncConfig>,
}

impl AdminScimSyncRunOut {
    pub fn builder() -> AdminScimSyncRunOutBuilder {
        <AdminScimSyncRunOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminScimSyncRunOutBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    dry_run: Option<bool>,
    error_message: Option<String>,
    finished_at: Option<DateTime<FixedOffset>>,
    run_id: Option<String>,
    started_at: Option<DateTime<FixedOffset>>,
    status: Option<ScimSyncRunStatus>,
    summary: Option<ScimSyncSummaryOut>,
    sync_config: Option<AdminScimSyncConfig>,
}

impl AdminScimSyncRunOutBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn finished_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.finished_at = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn status(mut self, value: ScimSyncRunStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn summary(mut self, value: ScimSyncSummaryOut) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn sync_config(mut self, value: AdminScimSyncConfig) -> Self {
        self.sync_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdminScimSyncRunOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](AdminScimSyncRunOutBuilder::created_at)
    /// - [`dry_run`](AdminScimSyncRunOutBuilder::dry_run)
    /// - [`run_id`](AdminScimSyncRunOutBuilder::run_id)
    /// - [`status`](AdminScimSyncRunOutBuilder::status)
    pub fn build(self) -> Result<AdminScimSyncRunOut, BuildError> {
        Ok(AdminScimSyncRunOut {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            dry_run: self
                .dry_run
                .ok_or_else(|| BuildError::missing_field("dry_run"))?,
            error_message: self.error_message,
            finished_at: self.finished_at,
            run_id: self
                .run_id
                .ok_or_else(|| BuildError::missing_field("run_id"))?,
            started_at: self.started_at,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            summary: self.summary,
            sync_config: self.sync_config,
        })
    }
}
