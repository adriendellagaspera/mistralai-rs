pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScimSyncSummaryOut {
    #[serde(default)]
    pub groups: ScimSyncGroupsSummaryOut,
    #[serde(default)]
    pub memberships: Vec<ScimSyncMembershipActionOut>,
    #[serde(default)]
    pub users: ScimSyncUsersSummaryOut,
}

impl ScimSyncSummaryOut {
    pub fn builder() -> ScimSyncSummaryOutBuilder {
        <ScimSyncSummaryOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScimSyncSummaryOutBuilder {
    groups: Option<ScimSyncGroupsSummaryOut>,
    memberships: Option<Vec<ScimSyncMembershipActionOut>>,
    users: Option<ScimSyncUsersSummaryOut>,
}

impl ScimSyncSummaryOutBuilder {
    pub fn groups(mut self, value: ScimSyncGroupsSummaryOut) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn memberships(mut self, value: Vec<ScimSyncMembershipActionOut>) -> Self {
        self.memberships = Some(value);
        self
    }

    pub fn users(mut self, value: ScimSyncUsersSummaryOut) -> Self {
        self.users = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScimSyncSummaryOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`groups`](ScimSyncSummaryOutBuilder::groups)
    /// - [`memberships`](ScimSyncSummaryOutBuilder::memberships)
    /// - [`users`](ScimSyncSummaryOutBuilder::users)
    pub fn build(self) -> Result<ScimSyncSummaryOut, BuildError> {
        Ok(ScimSyncSummaryOut {
            groups: self
                .groups
                .ok_or_else(|| BuildError::missing_field("groups"))?,
            memberships: self
                .memberships
                .ok_or_else(|| BuildError::missing_field("memberships"))?,
            users: self
                .users
                .ok_or_else(|| BuildError::missing_field("users"))?,
        })
    }
}
