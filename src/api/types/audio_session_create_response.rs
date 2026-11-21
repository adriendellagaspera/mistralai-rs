pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AudioSessionCreateResponse {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
    /// Effective function declarations after saved-agent and session overrides are merged. Connected SDKs can compare them with their local handlers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_tools: Option<Vec<VoiceAgentFunctionTool>>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub participant_token: String,
    #[serde(default)]
    pub server_url: String,
}

impl AudioSessionCreateResponse {
    pub fn builder() -> AudioSessionCreateResponseBuilder {
        <AudioSessionCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioSessionCreateResponseBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    expires_at: Option<DateTime<FixedOffset>>,
    function_tools: Option<Vec<VoiceAgentFunctionTool>>,
    id: Option<String>,
    participant_token: Option<String>,
    server_url: Option<String>,
}

impl AudioSessionCreateResponseBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn function_tools(mut self, value: Vec<VoiceAgentFunctionTool>) -> Self {
        self.function_tools = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn participant_token(mut self, value: impl Into<String>) -> Self {
        self.participant_token = Some(value.into());
        self
    }

    pub fn server_url(mut self, value: impl Into<String>) -> Self {
        self.server_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioSessionCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](AudioSessionCreateResponseBuilder::created_at)
    /// - [`expires_at`](AudioSessionCreateResponseBuilder::expires_at)
    /// - [`id`](AudioSessionCreateResponseBuilder::id)
    /// - [`participant_token`](AudioSessionCreateResponseBuilder::participant_token)
    /// - [`server_url`](AudioSessionCreateResponseBuilder::server_url)
    pub fn build(self) -> Result<AudioSessionCreateResponse, BuildError> {
        Ok(AudioSessionCreateResponse {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            function_tools: self.function_tools,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            participant_token: self
                .participant_token
                .ok_or_else(|| BuildError::missing_field("participant_token"))?,
            server_url: self
                .server_url
                .ok_or_else(|| BuildError::missing_field("server_url"))?,
        })
    }
}
