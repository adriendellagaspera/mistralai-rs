pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminScimSyncTriggerOut {
    /// Unique identifier of the created SCIM synchronization run.
    #[serde(default)]
    pub run_id: String,
}

impl AdminScimSyncTriggerOut {
    pub fn builder() -> AdminScimSyncTriggerOutBuilder {
        <AdminScimSyncTriggerOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminScimSyncTriggerOutBuilder {
    run_id: Option<String>,
}

impl AdminScimSyncTriggerOutBuilder {
    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdminScimSyncTriggerOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`run_id`](AdminScimSyncTriggerOutBuilder::run_id)
    pub fn build(self) -> Result<AdminScimSyncTriggerOut, BuildError> {
        Ok(AdminScimSyncTriggerOut {
            run_id: self
                .run_id
                .ok_or_else(|| BuildError::missing_field("run_id"))?,
        })
    }
}
