pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VibeNextEditActiveUsersStat {
    #[serde(default)]
    pub day: NaiveDate,
    #[serde(default)]
    pub nb_active_users: i64,
}

impl VibeNextEditActiveUsersStat {
    pub fn builder() -> VibeNextEditActiveUsersStatBuilder {
        <VibeNextEditActiveUsersStatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VibeNextEditActiveUsersStatBuilder {
    day: Option<NaiveDate>,
    nb_active_users: Option<i64>,
}

impl VibeNextEditActiveUsersStatBuilder {
    pub fn day(mut self, value: NaiveDate) -> Self {
        self.day = Some(value);
        self
    }

    pub fn nb_active_users(mut self, value: i64) -> Self {
        self.nb_active_users = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VibeNextEditActiveUsersStat`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](VibeNextEditActiveUsersStatBuilder::day)
    /// - [`nb_active_users`](VibeNextEditActiveUsersStatBuilder::nb_active_users)
    pub fn build(self) -> Result<VibeNextEditActiveUsersStat, BuildError> {
        Ok(VibeNextEditActiveUsersStat {
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
            nb_active_users: self
                .nb_active_users
                .ok_or_else(|| BuildError::missing_field("nb_active_users"))?,
        })
    }
}
