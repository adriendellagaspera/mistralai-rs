pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VibeActiveUsersStat {
    #[serde(default)]
    pub day: NaiveDate,
    #[serde(default)]
    pub nb_active_users_total: i64,
    #[serde(default)]
    pub nb_active_users_cli: i64,
    #[serde(default)]
    pub nb_active_users_acp: i64,
    #[serde(default)]
    pub nb_active_users_programmatic: i64,
}

impl VibeActiveUsersStat {
    pub fn builder() -> VibeActiveUsersStatBuilder {
        <VibeActiveUsersStatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VibeActiveUsersStatBuilder {
    day: Option<NaiveDate>,
    nb_active_users_total: Option<i64>,
    nb_active_users_cli: Option<i64>,
    nb_active_users_acp: Option<i64>,
    nb_active_users_programmatic: Option<i64>,
}

impl VibeActiveUsersStatBuilder {
    pub fn day(mut self, value: NaiveDate) -> Self {
        self.day = Some(value);
        self
    }

    pub fn nb_active_users_total(mut self, value: i64) -> Self {
        self.nb_active_users_total = Some(value);
        self
    }

    pub fn nb_active_users_cli(mut self, value: i64) -> Self {
        self.nb_active_users_cli = Some(value);
        self
    }

    pub fn nb_active_users_acp(mut self, value: i64) -> Self {
        self.nb_active_users_acp = Some(value);
        self
    }

    pub fn nb_active_users_programmatic(mut self, value: i64) -> Self {
        self.nb_active_users_programmatic = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VibeActiveUsersStat`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](VibeActiveUsersStatBuilder::day)
    /// - [`nb_active_users_total`](VibeActiveUsersStatBuilder::nb_active_users_total)
    /// - [`nb_active_users_cli`](VibeActiveUsersStatBuilder::nb_active_users_cli)
    /// - [`nb_active_users_acp`](VibeActiveUsersStatBuilder::nb_active_users_acp)
    /// - [`nb_active_users_programmatic`](VibeActiveUsersStatBuilder::nb_active_users_programmatic)
    pub fn build(self) -> Result<VibeActiveUsersStat, BuildError> {
        Ok(VibeActiveUsersStat {
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
            nb_active_users_total: self
                .nb_active_users_total
                .ok_or_else(|| BuildError::missing_field("nb_active_users_total"))?,
            nb_active_users_cli: self
                .nb_active_users_cli
                .ok_or_else(|| BuildError::missing_field("nb_active_users_cli"))?,
            nb_active_users_acp: self
                .nb_active_users_acp
                .ok_or_else(|| BuildError::missing_field("nb_active_users_acp"))?,
            nb_active_users_programmatic: self
                .nb_active_users_programmatic
                .ok_or_else(|| BuildError::missing_field("nb_active_users_programmatic"))?,
        })
    }
}
