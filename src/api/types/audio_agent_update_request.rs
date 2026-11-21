pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AudioAgentUpdateRequest {
    #[serde(default)]
    pub configuration: AudioAgentConfigurationRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
}

impl AudioAgentUpdateRequest {
    pub fn builder() -> AudioAgentUpdateRequestBuilder {
        <AudioAgentUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioAgentUpdateRequestBuilder {
    configuration: Option<AudioAgentConfigurationRequest>,
    description: Option<String>,
    name: Option<String>,
}

impl AudioAgentUpdateRequestBuilder {
    pub fn configuration(mut self, value: AudioAgentConfigurationRequest) -> Self {
        self.configuration = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioAgentUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`configuration`](AudioAgentUpdateRequestBuilder::configuration)
    /// - [`name`](AudioAgentUpdateRequestBuilder::name)
    pub fn build(self) -> Result<AudioAgentUpdateRequest, BuildError> {
        Ok(AudioAgentUpdateRequest {
            configuration: self
                .configuration
                .ok_or_else(|| BuildError::missing_field("configuration"))?,
            description: self.description,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
