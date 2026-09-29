pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScimSyncUsersSummaryOut {
    #[serde(default)]
    pub deprovision: Vec<ScimSyncUserActionOut>,
    #[serde(default)]
    pub provision: Vec<ScimSyncUserActionOut>,
}

impl ScimSyncUsersSummaryOut {
    pub fn builder() -> ScimSyncUsersSummaryOutBuilder {
        <ScimSyncUsersSummaryOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScimSyncUsersSummaryOutBuilder {
    deprovision: Option<Vec<ScimSyncUserActionOut>>,
    provision: Option<Vec<ScimSyncUserActionOut>>,
}

impl ScimSyncUsersSummaryOutBuilder {
    pub fn deprovision(mut self, value: Vec<ScimSyncUserActionOut>) -> Self {
        self.deprovision = Some(value);
        self
    }

    pub fn provision(mut self, value: Vec<ScimSyncUserActionOut>) -> Self {
        self.provision = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScimSyncUsersSummaryOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deprovision`](ScimSyncUsersSummaryOutBuilder::deprovision)
    /// - [`provision`](ScimSyncUsersSummaryOutBuilder::provision)
    pub fn build(self) -> Result<ScimSyncUsersSummaryOut, BuildError> {
        Ok(ScimSyncUsersSummaryOut {
            deprovision: self
                .deprovision
                .ok_or_else(|| BuildError::missing_field("deprovision"))?,
            provision: self
                .provision
                .ok_or_else(|| BuildError::missing_field("provision"))?,
        })
    }
}
