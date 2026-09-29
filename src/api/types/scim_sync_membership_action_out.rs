pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScimSyncMembershipActionOut {
    #[serde(default)]
    pub group: ScimSyncGroupRefOut,
    #[serde(default)]
    pub users_added: Vec<ScimSyncUserRefOut>,
    #[serde(default)]
    pub users_removed: Vec<ScimSyncUserRefOut>,
}

impl ScimSyncMembershipActionOut {
    pub fn builder() -> ScimSyncMembershipActionOutBuilder {
        <ScimSyncMembershipActionOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScimSyncMembershipActionOutBuilder {
    group: Option<ScimSyncGroupRefOut>,
    users_added: Option<Vec<ScimSyncUserRefOut>>,
    users_removed: Option<Vec<ScimSyncUserRefOut>>,
}

impl ScimSyncMembershipActionOutBuilder {
    pub fn group(mut self, value: ScimSyncGroupRefOut) -> Self {
        self.group = Some(value);
        self
    }

    pub fn users_added(mut self, value: Vec<ScimSyncUserRefOut>) -> Self {
        self.users_added = Some(value);
        self
    }

    pub fn users_removed(mut self, value: Vec<ScimSyncUserRefOut>) -> Self {
        self.users_removed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScimSyncMembershipActionOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group`](ScimSyncMembershipActionOutBuilder::group)
    /// - [`users_added`](ScimSyncMembershipActionOutBuilder::users_added)
    /// - [`users_removed`](ScimSyncMembershipActionOutBuilder::users_removed)
    pub fn build(self) -> Result<ScimSyncMembershipActionOut, BuildError> {
        Ok(ScimSyncMembershipActionOut {
            group: self
                .group
                .ok_or_else(|| BuildError::missing_field("group"))?,
            users_added: self
                .users_added
                .ok_or_else(|| BuildError::missing_field("users_added"))?,
            users_removed: self
                .users_removed
                .ok_or_else(|| BuildError::missing_field("users_removed"))?,
        })
    }
}
