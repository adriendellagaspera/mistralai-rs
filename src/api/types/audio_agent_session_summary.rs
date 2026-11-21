pub use crate::prelude::*;

/// A session of a saved agent, without its transcript.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioAgentSessionSummary {
    #[serde(default)]
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_metadata: Option<AmazonConnectContact>,
    /// Seconds from the first participant joining to the session ending, null if nobody joined or it is still running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(default)]
    pub id: String,
    /// Set when the session came in over telephony.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_id: Option<String>,
    /// When the first participant joined, null if nobody ever did.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<FixedOffset>>,
}

impl AudioAgentSessionSummary {
    pub fn builder() -> AudioAgentSessionSummaryBuilder {
        <AudioAgentSessionSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioAgentSessionSummaryBuilder {
    agent_id: Option<String>,
    bridge_metadata: Option<AmazonConnectContact>,
    duration: Option<i64>,
    id: Option<String>,
    phone_number_id: Option<String>,
    started_at: Option<DateTime<FixedOffset>>,
}

impl AudioAgentSessionSummaryBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn bridge_metadata(mut self, value: AmazonConnectContact) -> Self {
        self.bridge_metadata = Some(value);
        self
    }

    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.phone_number_id = Some(value.into());
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioAgentSessionSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](AudioAgentSessionSummaryBuilder::agent_id)
    /// - [`id`](AudioAgentSessionSummaryBuilder::id)
    pub fn build(self) -> Result<AudioAgentSessionSummary, BuildError> {
        Ok(AudioAgentSessionSummary {
            agent_id: self
                .agent_id
                .ok_or_else(|| BuildError::missing_field("agent_id"))?,
            bridge_metadata: self.bridge_metadata,
            duration: self.duration,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            phone_number_id: self.phone_number_id,
            started_at: self.started_at,
        })
    }
}
