pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScimSyncUserActionOut {
    #[serde(default)]
    pub user: ScimSyncUserRefOut,
}

impl ScimSyncUserActionOut {
    pub fn builder() -> ScimSyncUserActionOutBuilder {
        <ScimSyncUserActionOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScimSyncUserActionOutBuilder {
    user: Option<ScimSyncUserRefOut>,
}

impl ScimSyncUserActionOutBuilder {
    pub fn user(mut self, value: ScimSyncUserRefOut) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScimSyncUserActionOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user`](ScimSyncUserActionOutBuilder::user)
    pub fn build(self) -> Result<ScimSyncUserActionOut, BuildError> {
        Ok(ScimSyncUserActionOut {
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
