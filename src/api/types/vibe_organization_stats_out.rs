pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VibeOrganizationStatsOut {
    /// End of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub end_time: i64,
    #[serde(default)]
    pub next_edit_active_users: Vec<VibeNextEditActiveUsersStat>,
    #[serde(default)]
    pub next_edit_modified_loc: Vec<VibeNextEditModifiedLocStat>,
    #[serde(default)]
    pub next_edit_suggestions: Vec<VibeNextEditSuggestionStat>,
    /// Start of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub start_time: i64,
}

impl VibeOrganizationStatsOut {
    pub fn builder() -> VibeOrganizationStatsOutBuilder {
        <VibeOrganizationStatsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VibeOrganizationStatsOutBuilder {
    end_time: Option<i64>,
    next_edit_active_users: Option<Vec<VibeNextEditActiveUsersStat>>,
    next_edit_modified_loc: Option<Vec<VibeNextEditModifiedLocStat>>,
    next_edit_suggestions: Option<Vec<VibeNextEditSuggestionStat>>,
    start_time: Option<i64>,
}

impl VibeOrganizationStatsOutBuilder {
    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn next_edit_active_users(mut self, value: Vec<VibeNextEditActiveUsersStat>) -> Self {
        self.next_edit_active_users = Some(value);
        self
    }

    pub fn next_edit_modified_loc(mut self, value: Vec<VibeNextEditModifiedLocStat>) -> Self {
        self.next_edit_modified_loc = Some(value);
        self
    }

    pub fn next_edit_suggestions(mut self, value: Vec<VibeNextEditSuggestionStat>) -> Self {
        self.next_edit_suggestions = Some(value);
        self
    }

    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VibeOrganizationStatsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`end_time`](VibeOrganizationStatsOutBuilder::end_time)
    /// - [`next_edit_active_users`](VibeOrganizationStatsOutBuilder::next_edit_active_users)
    /// - [`next_edit_modified_loc`](VibeOrganizationStatsOutBuilder::next_edit_modified_loc)
    /// - [`next_edit_suggestions`](VibeOrganizationStatsOutBuilder::next_edit_suggestions)
    /// - [`start_time`](VibeOrganizationStatsOutBuilder::start_time)
    pub fn build(self) -> Result<VibeOrganizationStatsOut, BuildError> {
        Ok(VibeOrganizationStatsOut {
            end_time: self
                .end_time
                .ok_or_else(|| BuildError::missing_field("end_time"))?,
            next_edit_active_users: self
                .next_edit_active_users
                .ok_or_else(|| BuildError::missing_field("next_edit_active_users"))?,
            next_edit_modified_loc: self
                .next_edit_modified_loc
                .ok_or_else(|| BuildError::missing_field("next_edit_modified_loc"))?,
            next_edit_suggestions: self
                .next_edit_suggestions
                .ok_or_else(|| BuildError::missing_field("next_edit_suggestions"))?,
            start_time: self
                .start_time
                .ok_or_else(|| BuildError::missing_field("start_time"))?,
        })
    }
}
