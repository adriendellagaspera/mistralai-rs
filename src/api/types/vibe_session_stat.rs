pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VibeSessionStat {
    #[serde(default)]
    pub day: NaiveDate,
    #[serde(default)]
    pub nb_sessions: i64,
    #[serde(default)]
    pub nb_sessions_acp: i64,
    #[serde(default)]
    pub nb_sessions_cli: i64,
    #[serde(default)]
    pub nb_sessions_programmatic: i64,
}

impl VibeSessionStat {
    pub fn builder() -> VibeSessionStatBuilder {
        <VibeSessionStatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VibeSessionStatBuilder {
    day: Option<NaiveDate>,
    nb_sessions: Option<i64>,
    nb_sessions_acp: Option<i64>,
    nb_sessions_cli: Option<i64>,
    nb_sessions_programmatic: Option<i64>,
}

impl VibeSessionStatBuilder {
    pub fn day(mut self, value: NaiveDate) -> Self {
        self.day = Some(value);
        self
    }

    pub fn nb_sessions(mut self, value: i64) -> Self {
        self.nb_sessions = Some(value);
        self
    }

    pub fn nb_sessions_acp(mut self, value: i64) -> Self {
        self.nb_sessions_acp = Some(value);
        self
    }

    pub fn nb_sessions_cli(mut self, value: i64) -> Self {
        self.nb_sessions_cli = Some(value);
        self
    }

    pub fn nb_sessions_programmatic(mut self, value: i64) -> Self {
        self.nb_sessions_programmatic = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VibeSessionStat`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](VibeSessionStatBuilder::day)
    /// - [`nb_sessions`](VibeSessionStatBuilder::nb_sessions)
    /// - [`nb_sessions_acp`](VibeSessionStatBuilder::nb_sessions_acp)
    /// - [`nb_sessions_cli`](VibeSessionStatBuilder::nb_sessions_cli)
    /// - [`nb_sessions_programmatic`](VibeSessionStatBuilder::nb_sessions_programmatic)
    pub fn build(self) -> Result<VibeSessionStat, BuildError> {
        Ok(VibeSessionStat {
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
            nb_sessions: self
                .nb_sessions
                .ok_or_else(|| BuildError::missing_field("nb_sessions"))?,
            nb_sessions_acp: self
                .nb_sessions_acp
                .ok_or_else(|| BuildError::missing_field("nb_sessions_acp"))?,
            nb_sessions_cli: self
                .nb_sessions_cli
                .ok_or_else(|| BuildError::missing_field("nb_sessions_cli"))?,
            nb_sessions_programmatic: self
                .nb_sessions_programmatic
                .ok_or_else(|| BuildError::missing_field("nb_sessions_programmatic"))?,
        })
    }
}
