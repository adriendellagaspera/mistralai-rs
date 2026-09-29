pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VibeNextEditSuggestionStat {
    #[serde(default)]
    pub day: NaiveDate,
    #[serde(default)]
    pub total_suggestions: i64,
    #[serde(default)]
    pub outcome_accepted: i64,
    #[serde(default)]
    pub outcome_rejected: i64,
    #[serde(default)]
    pub outcome_timeout: i64,
    #[serde(default)]
    pub outcome_aborted: i64,
    #[serde(default)]
    pub outcome_dismissed: i64,
    #[serde(default)]
    pub force_generated_true: i64,
    #[serde(default)]
    pub force_generated_false: i64,
    #[serde(default)]
    pub surface_widget: i64,
    #[serde(default)]
    pub surface_ghost_text: i64,
}

impl VibeNextEditSuggestionStat {
    pub fn builder() -> VibeNextEditSuggestionStatBuilder {
        <VibeNextEditSuggestionStatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VibeNextEditSuggestionStatBuilder {
    day: Option<NaiveDate>,
    total_suggestions: Option<i64>,
    outcome_accepted: Option<i64>,
    outcome_rejected: Option<i64>,
    outcome_timeout: Option<i64>,
    outcome_aborted: Option<i64>,
    outcome_dismissed: Option<i64>,
    force_generated_true: Option<i64>,
    force_generated_false: Option<i64>,
    surface_widget: Option<i64>,
    surface_ghost_text: Option<i64>,
}

impl VibeNextEditSuggestionStatBuilder {
    pub fn day(mut self, value: NaiveDate) -> Self {
        self.day = Some(value);
        self
    }

    pub fn total_suggestions(mut self, value: i64) -> Self {
        self.total_suggestions = Some(value);
        self
    }

    pub fn outcome_accepted(mut self, value: i64) -> Self {
        self.outcome_accepted = Some(value);
        self
    }

    pub fn outcome_rejected(mut self, value: i64) -> Self {
        self.outcome_rejected = Some(value);
        self
    }

    pub fn outcome_timeout(mut self, value: i64) -> Self {
        self.outcome_timeout = Some(value);
        self
    }

    pub fn outcome_aborted(mut self, value: i64) -> Self {
        self.outcome_aborted = Some(value);
        self
    }

    pub fn outcome_dismissed(mut self, value: i64) -> Self {
        self.outcome_dismissed = Some(value);
        self
    }

    pub fn force_generated_true(mut self, value: i64) -> Self {
        self.force_generated_true = Some(value);
        self
    }

    pub fn force_generated_false(mut self, value: i64) -> Self {
        self.force_generated_false = Some(value);
        self
    }

    pub fn surface_widget(mut self, value: i64) -> Self {
        self.surface_widget = Some(value);
        self
    }

    pub fn surface_ghost_text(mut self, value: i64) -> Self {
        self.surface_ghost_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VibeNextEditSuggestionStat`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](VibeNextEditSuggestionStatBuilder::day)
    /// - [`total_suggestions`](VibeNextEditSuggestionStatBuilder::total_suggestions)
    /// - [`outcome_accepted`](VibeNextEditSuggestionStatBuilder::outcome_accepted)
    /// - [`outcome_rejected`](VibeNextEditSuggestionStatBuilder::outcome_rejected)
    /// - [`outcome_timeout`](VibeNextEditSuggestionStatBuilder::outcome_timeout)
    /// - [`outcome_aborted`](VibeNextEditSuggestionStatBuilder::outcome_aborted)
    /// - [`outcome_dismissed`](VibeNextEditSuggestionStatBuilder::outcome_dismissed)
    /// - [`force_generated_true`](VibeNextEditSuggestionStatBuilder::force_generated_true)
    /// - [`force_generated_false`](VibeNextEditSuggestionStatBuilder::force_generated_false)
    /// - [`surface_widget`](VibeNextEditSuggestionStatBuilder::surface_widget)
    /// - [`surface_ghost_text`](VibeNextEditSuggestionStatBuilder::surface_ghost_text)
    pub fn build(self) -> Result<VibeNextEditSuggestionStat, BuildError> {
        Ok(VibeNextEditSuggestionStat {
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
            total_suggestions: self
                .total_suggestions
                .ok_or_else(|| BuildError::missing_field("total_suggestions"))?,
            outcome_accepted: self
                .outcome_accepted
                .ok_or_else(|| BuildError::missing_field("outcome_accepted"))?,
            outcome_rejected: self
                .outcome_rejected
                .ok_or_else(|| BuildError::missing_field("outcome_rejected"))?,
            outcome_timeout: self
                .outcome_timeout
                .ok_or_else(|| BuildError::missing_field("outcome_timeout"))?,
            outcome_aborted: self
                .outcome_aborted
                .ok_or_else(|| BuildError::missing_field("outcome_aborted"))?,
            outcome_dismissed: self
                .outcome_dismissed
                .ok_or_else(|| BuildError::missing_field("outcome_dismissed"))?,
            force_generated_true: self
                .force_generated_true
                .ok_or_else(|| BuildError::missing_field("force_generated_true"))?,
            force_generated_false: self
                .force_generated_false
                .ok_or_else(|| BuildError::missing_field("force_generated_false"))?,
            surface_widget: self
                .surface_widget
                .ok_or_else(|| BuildError::missing_field("surface_widget"))?,
            surface_ghost_text: self
                .surface_ghost_text
                .ok_or_else(|| BuildError::missing_field("surface_ghost_text"))?,
        })
    }
}
