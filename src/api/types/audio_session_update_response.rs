pub use crate::prelude::*;

/// The session the update was sent to. The agent applies the update at its
/// next turn boundary and reports the outcome through the `session_updated`
/// and `session_update_failed` webhooks.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AudioSessionUpdateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_metadata: Option<AmazonConnectContact>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub num_participants: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_call: Option<AudioSessionPhoneCall>,
    pub status: AudioSessionUpdateResponseStatus,
}

impl AudioSessionUpdateResponse {
    pub fn builder() -> AudioSessionUpdateResponseBuilder {
        <AudioSessionUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioSessionUpdateResponseBuilder {
    bridge_metadata: Option<AmazonConnectContact>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    num_participants: Option<i64>,
    phone_call: Option<AudioSessionPhoneCall>,
    status: Option<AudioSessionUpdateResponseStatus>,
}

impl AudioSessionUpdateResponseBuilder {
    pub fn bridge_metadata(mut self, value: AmazonConnectContact) -> Self {
        self.bridge_metadata = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn num_participants(mut self, value: i64) -> Self {
        self.num_participants = Some(value);
        self
    }

    pub fn phone_call(mut self, value: AudioSessionPhoneCall) -> Self {
        self.phone_call = Some(value);
        self
    }

    pub fn status(mut self, value: AudioSessionUpdateResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioSessionUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](AudioSessionUpdateResponseBuilder::created_at)
    /// - [`id`](AudioSessionUpdateResponseBuilder::id)
    /// - [`num_participants`](AudioSessionUpdateResponseBuilder::num_participants)
    /// - [`status`](AudioSessionUpdateResponseBuilder::status)
    pub fn build(self) -> Result<AudioSessionUpdateResponse, BuildError> {
        Ok(AudioSessionUpdateResponse {
            bridge_metadata: self.bridge_metadata,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            num_participants: self
                .num_participants
                .ok_or_else(|| BuildError::missing_field("num_participants"))?,
            phone_call: self.phone_call,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
