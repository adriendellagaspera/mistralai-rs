pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AudioAgentResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<AudioAgentConfigurationResponseConfig>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub owner_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl AudioAgentResponse {
    pub fn builder() -> AudioAgentResponseBuilder {
        <AudioAgentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioAgentResponseBuilder {
    configuration: Option<AudioAgentConfigurationResponseConfig>,
    created_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    id: Option<String>,
    name: Option<String>,
    owner_id: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    version: Option<i64>,
}

impl AudioAgentResponseBuilder {
    pub fn configuration(mut self, value: AudioAgentConfigurationResponseConfig) -> Self {
        self.configuration = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn owner_id(mut self, value: impl Into<String>) -> Self {
        self.owner_id = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioAgentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](AudioAgentResponseBuilder::created_at)
    /// - [`id`](AudioAgentResponseBuilder::id)
    /// - [`owner_id`](AudioAgentResponseBuilder::owner_id)
    /// - [`updated_at`](AudioAgentResponseBuilder::updated_at)
    pub fn build(self) -> Result<AudioAgentResponse, BuildError> {
        Ok(AudioAgentResponse {
            configuration: self.configuration,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self.description,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            owner_id: self
                .owner_id
                .ok_or_else(|| BuildError::missing_field("owner_id"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            version: self.version,
        })
    }
}
