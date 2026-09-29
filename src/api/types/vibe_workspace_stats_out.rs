pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VibeWorkspaceStatsOut {
    /// Start of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub start_time: i64,
    /// End of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub end_time: i64,
    #[serde(default)]
    pub sessions: Vec<VibeSessionStat>,
    #[serde(default)]
    pub user_prompts: Vec<VibeUserPromptsStat>,
    #[serde(default)]
    pub active_users: Vec<VibeActiveUsersStat>,
    #[serde(default)]
    pub consumed_tokens: Vec<VibeConsumedTokensStat>,
    #[serde(default)]
    pub tool_calls: Vec<VibeToolCallsStat>,
    #[serde(default)]
    pub tool_calls_by_name: Vec<VibeToolCallsByNameStat>,
    #[serde(default)]
    pub session_durations: Vec<VibeSessionDurationStat>,
}

impl VibeWorkspaceStatsOut {
    pub fn builder() -> VibeWorkspaceStatsOutBuilder {
        <VibeWorkspaceStatsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VibeWorkspaceStatsOutBuilder {
    start_time: Option<i64>,
    end_time: Option<i64>,
    sessions: Option<Vec<VibeSessionStat>>,
    user_prompts: Option<Vec<VibeUserPromptsStat>>,
    active_users: Option<Vec<VibeActiveUsersStat>>,
    consumed_tokens: Option<Vec<VibeConsumedTokensStat>>,
    tool_calls: Option<Vec<VibeToolCallsStat>>,
    tool_calls_by_name: Option<Vec<VibeToolCallsByNameStat>>,
    session_durations: Option<Vec<VibeSessionDurationStat>>,
}

impl VibeWorkspaceStatsOutBuilder {
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn sessions(mut self, value: Vec<VibeSessionStat>) -> Self {
        self.sessions = Some(value);
        self
    }

    pub fn user_prompts(mut self, value: Vec<VibeUserPromptsStat>) -> Self {
        self.user_prompts = Some(value);
        self
    }

    pub fn active_users(mut self, value: Vec<VibeActiveUsersStat>) -> Self {
        self.active_users = Some(value);
        self
    }

    pub fn consumed_tokens(mut self, value: Vec<VibeConsumedTokensStat>) -> Self {
        self.consumed_tokens = Some(value);
        self
    }

    pub fn tool_calls(mut self, value: Vec<VibeToolCallsStat>) -> Self {
        self.tool_calls = Some(value);
        self
    }

    pub fn tool_calls_by_name(mut self, value: Vec<VibeToolCallsByNameStat>) -> Self {
        self.tool_calls_by_name = Some(value);
        self
    }

    pub fn session_durations(mut self, value: Vec<VibeSessionDurationStat>) -> Self {
        self.session_durations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VibeWorkspaceStatsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_time`](VibeWorkspaceStatsOutBuilder::start_time)
    /// - [`end_time`](VibeWorkspaceStatsOutBuilder::end_time)
    /// - [`sessions`](VibeWorkspaceStatsOutBuilder::sessions)
    /// - [`user_prompts`](VibeWorkspaceStatsOutBuilder::user_prompts)
    /// - [`active_users`](VibeWorkspaceStatsOutBuilder::active_users)
    /// - [`consumed_tokens`](VibeWorkspaceStatsOutBuilder::consumed_tokens)
    /// - [`tool_calls`](VibeWorkspaceStatsOutBuilder::tool_calls)
    /// - [`tool_calls_by_name`](VibeWorkspaceStatsOutBuilder::tool_calls_by_name)
    /// - [`session_durations`](VibeWorkspaceStatsOutBuilder::session_durations)
    pub fn build(self) -> Result<VibeWorkspaceStatsOut, BuildError> {
        Ok(VibeWorkspaceStatsOut {
            start_time: self
                .start_time
                .ok_or_else(|| BuildError::missing_field("start_time"))?,
            end_time: self
                .end_time
                .ok_or_else(|| BuildError::missing_field("end_time"))?,
            sessions: self
                .sessions
                .ok_or_else(|| BuildError::missing_field("sessions"))?,
            user_prompts: self
                .user_prompts
                .ok_or_else(|| BuildError::missing_field("user_prompts"))?,
            active_users: self
                .active_users
                .ok_or_else(|| BuildError::missing_field("active_users"))?,
            consumed_tokens: self
                .consumed_tokens
                .ok_or_else(|| BuildError::missing_field("consumed_tokens"))?,
            tool_calls: self
                .tool_calls
                .ok_or_else(|| BuildError::missing_field("tool_calls"))?,
            tool_calls_by_name: self
                .tool_calls_by_name
                .ok_or_else(|| BuildError::missing_field("tool_calls_by_name"))?,
            session_durations: self
                .session_durations
                .ok_or_else(|| BuildError::missing_field("session_durations"))?,
        })
    }
}
