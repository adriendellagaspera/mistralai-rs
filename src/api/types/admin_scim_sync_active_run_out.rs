pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdminScimSyncActiveRunOut {
    /// Unique identifier of the already active SCIM synchronization run.
    #[serde(default)]
    pub run_id: String,
    /// Current lifecycle status of the active synchronization run.
    pub status: ScimSyncRunStatus,
}

impl AdminScimSyncActiveRunOut {
    pub fn builder() -> AdminScimSyncActiveRunOutBuilder {
        <AdminScimSyncActiveRunOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminScimSyncActiveRunOutBuilder {
    run_id: Option<String>,
    status: Option<ScimSyncRunStatus>,
}

impl AdminScimSyncActiveRunOutBuilder {
    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ScimSyncRunStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdminScimSyncActiveRunOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`run_id`](AdminScimSyncActiveRunOutBuilder::run_id)
    /// - [`status`](AdminScimSyncActiveRunOutBuilder::status)
    pub fn build(self) -> Result<AdminScimSyncActiveRunOut, BuildError> {
        Ok(AdminScimSyncActiveRunOut {
            run_id: self
                .run_id
                .ok_or_else(|| BuildError::missing_field("run_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
