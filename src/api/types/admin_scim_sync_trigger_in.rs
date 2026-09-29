pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminScimSyncTriggerIn {
    /// Preview all synchronization changes without applying them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// Categories to synchronize. Required when dry_run is false; ignored for dry runs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_config: Option<AdminScimSyncConfig>,
}

impl AdminScimSyncTriggerIn {
    pub fn builder() -> AdminScimSyncTriggerInBuilder {
        <AdminScimSyncTriggerInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminScimSyncTriggerInBuilder {
    dry_run: Option<bool>,
    sync_config: Option<AdminScimSyncConfig>,
}

impl AdminScimSyncTriggerInBuilder {
    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
        self
    }

    pub fn sync_config(mut self, value: AdminScimSyncConfig) -> Self {
        self.sync_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdminScimSyncTriggerIn`].
    pub fn build(self) -> Result<AdminScimSyncTriggerIn, BuildError> {
        Ok(AdminScimSyncTriggerIn {
            dry_run: self.dry_run,
            sync_config: self.sync_config,
        })
    }
}
